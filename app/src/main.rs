#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod error;
pub mod req;

use std::{cmp::Ordering, fs, io::Cursor, thread};

use base64::{prelude::BASE64_STANDARD, Engine};
use bytes::Bytes;
use eframe::egui;
use error::{error_message, Error};
use req::{Files, Resource};
use reqwest::Client;
use rodio::{Decoder, OutputStream, Sink};
use tokio::sync::mpsc::{self, Receiver, Sender};

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    // logging
    env_logger::init();

    let (send_server, mut rec_server) = mpsc::channel::<SenderTypeServer>(32);
    let (send_ui, rec_ui) = mpsc::channel::<SenderTypeUi>(32);
    let client = Client::new();

    // start gui server request receiver
    tokio::spawn(async move {
        while let Some(message) = rec_server.recv().await {
            let thread = async {
                let tx = send_ui.clone();
                match message {
                    SenderTypeServer::GetResources => {
                        let data = req::get_resources(&client).await?;
                        // println!("{:?}", data);
                        tx.send(SenderTypeUi::Resources(data))
                            .await
                            .unwrap_or_default();
                    }
                    SenderTypeServer::AddResource(mut resources, files) => {
                        let resource = req::add_resource(&client, files).await?;
                        resources.push(resource);
                        tx.send(SenderTypeUi::Resources(resources))
                            .await
                            .unwrap_or_default();
                    }
                    SenderTypeServer::DownloadAudio(url) => {
                        let bytes = req::download_file(&client, &url).await?;
                        tx.send(SenderTypeUi::PlayAudio(bytes))
                            .await
                            .unwrap_or_default();
                    }
                }
                Ok::<(), Error>(())
            }
            .await;
            if let Err(err) = thread {
                send_ui
                    .clone()
                    .send(SenderTypeUi::Error(err))
                    .await
                    .unwrap_or_default();
            }
        }
    });

    // getting initial data
    let tx = send_server.clone();
    tokio::spawn(async move {
        tx.send(SenderTypeServer::GetResources)
            .await
            .unwrap_or_default();
    });

    // start ui
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 400.0])
            .with_min_inner_size([300.0, 200.0]),
        ..Default::default()
    };
    eframe::run_native(
        "shitboard",
        options,
        Box::new(|_| {
            Box::new(App {
                selected_tab: AppTabs::Sounds,
                resources: Vec::default(),
                send_server,
                rec_ui,
                error_modal: None,
                add_modal: None,
                audio_settings: AudioSettings::default(),
            })
        }),
    )
}

struct App {
    // tabs
    selected_tab: AppTabs,
    // cached data
    resources: Vec<Resource>,
    // sender
    send_server: Sender<SenderTypeServer>,
    rec_ui: Receiver<SenderTypeUi>,
    // modals
    error_modal: Option<Error>,
    add_modal: Option<Files>,
    // settings
    audio_settings: AudioSettings,
}

#[derive(PartialEq, Clone)]
enum AppTabs {
    Sounds,
    Settings,
}

#[derive(Clone, Debug)]
pub enum SenderTypeServer {
    GetResources,
    AddResource(Vec<Resource>, Files),
    DownloadAudio(String),
}

#[derive(Debug, Default)]
pub enum SenderTypeUi {
    #[default]
    None,
    Resources(Vec<Resource>),
    PlayAudio(Bytes),
    Error(Error),
}

#[derive(Debug)]
struct AudioSettings {
    /// From `100.0` to `0.0`, need to be divided by `100.0` for setting volume
    volume: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self { volume: 100.0 }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.selectable_value(&mut self.selected_tab, AppTabs::Sounds, "Sounds");
                ui.selectable_value(&mut self.selected_tab, AppTabs::Settings, "Settings");
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.selected_tab {
                AppTabs::Sounds => {
                    // getting async data
                    match self.rec_ui.try_recv().unwrap_or_default() {
                        SenderTypeUi::None => {}
                        SenderTypeUi::Resources(resources) => {
                            self.resources = resources;
                            self.resources.sort_by(|a, b| {
                                a.time_stamp
                                    .partial_cmp(&b.time_stamp)
                                    .unwrap_or(Ordering::Equal)
                            });
                            // make sure changes be visible
                            ctx.request_repaint();
                        }
                        // todo: caching for performance
                        SenderTypeUi::PlayAudio(bytes) => {
                            play_audio(bytes, self.audio_settings.volume);
                            // make sure changes be visible
                            ctx.request_repaint();
                        }
                        SenderTypeUi::Error(err) => {
                            self.error_modal = Some(err);
                            // make sure changes be visible
                            ctx.request_repaint();
                        }
                    };

                    // error modal
                    if self.error_modal.is_some() {
                        egui::Window::new("Error")
                            .title_bar(true)
                            .collapsible(false)
                            .resizable(false)
                            .max_width(ui.available_width())
                            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0.0, 0.0))
                            .show(ui.ctx(), |ui| {
                                if let Some(error) = self.error_modal.as_ref() {
                                    let error_message = match error {
                                        Error::Http(error) => format!("Http Error: {}", error),
                                        Error::Custom(error) => error_message(error),
                                    };
                                    ui.label(error_message);
                                }
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::TOP),
                                    |ui| {
                                        if ui.button("Close").clicked() {
                                            self.error_modal = None;
                                        }
                                    },
                                );
                            });
                    }

                    // add modal
                    if self.add_modal.is_some() {
                        egui::Window::new("Add")
                            .title_bar(true)
                            .collapsible(false)
                            .resizable(false)
                            .max_width(ui.available_width())
                            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0.0, 0.0))
                            .show(ui.ctx(), |ui| {
                                if let Some(files) = &mut self.add_modal {
                                    ui.text_edit_singleline(&mut files.title);
                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Min)
                                            .with_main_justify(true),
                                        |ui| {
                                            let title = if files.audio_data.is_empty() {
                                                "Pick File"
                                            } else {
                                                "Picked"
                                            };
                                            if ui.button(title).clicked() {
                                                if let Some(file) =
                                                    rfd::FileDialog::new().pick_file()
                                                {
                                                    files.audio_data = BASE64_STANDARD
                                                        .encode(fs::read(file).unwrap_or_default());
                                                }
                                            }
                                        },
                                    );
                                }

                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::TOP),
                                    |ui| {
                                        if ui.button("Close").clicked() {
                                            self.add_modal = None;
                                        }
                                        let files = self.add_modal.clone().unwrap_or_default();
                                        ui.add_enabled_ui(
                                            !files.title.is_empty() && !files.audio_data.is_empty(),
                                            |ui| {
                                                if ui.button("Add").clicked() {
                                                    let tx = self.send_server.clone();
                                                    let resources = self.resources.clone();
                                                    tokio::spawn(async move {
                                                        tx.send(SenderTypeServer::AddResource(
                                                            resources, files,
                                                        ))
                                                        .await
                                                        .unwrap_or_default();
                                                    });
                                                    self.add_modal = None;
                                                }
                                            },
                                        );
                                    },
                                );
                            });
                    }

                    ui.horizontal(|ui| {
                        if ui.button("Reload").clicked() {
                            let tx = self.send_server.clone();
                            tokio::spawn(async move {
                                tx.send(SenderTypeServer::GetResources)
                                    .await
                                    .unwrap_or_default();
                            });
                        }

                        if ui.button("Add Clip").clicked() {
                            self.add_modal = Some(Files::default());
                        }
                    });
                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.set_height(ui.available_height());
                        ui.horizontal_wrapped(|ui| {
                            for resource in &self.resources {
                                if ui.button(&resource.title).clicked() {
                                    let tx = self.send_server.clone();
                                    let audio_file =
                                        resource.audio_file.to_string_lossy().to_string();
                                    tokio::spawn(async move {
                                        tx.send(SenderTypeServer::DownloadAudio(audio_file))
                                            .await
                                            .unwrap_or_default();
                                    });
                                }
                            }
                        });
                    });
                }
                AppTabs::Settings => {
                    ui.label("Volume");
                    ui.add(
                        egui::Slider::new(&mut self.audio_settings.volume, 0.0..=100.0)
                            .max_decimals(2)
                            .suffix("%")
                            .trailing_fill(true)
                            .handle_shape(egui::style::HandleShape::Rect { aspect_ratio: 0.5 }),
                    );
                }
            }
        });
    }
}

/// Plays the audio in a different thread from given bytes.
fn play_audio(bytes: Bytes, volume: f32) {
    // todo: error handling
    thread::spawn(move || {
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).unwrap();

        sink.set_volume(volume / 100.0);

        sink.append(Decoder::new(Cursor::new(bytes)).unwrap());

        sink.sleep_until_end();
    });
}
