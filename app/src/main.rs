#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod req;

use std::{io::Cursor, thread};

use bytes::Bytes;
use eframe::egui;
use req::{Files, Resource};
use reqwest::{Client, Error};
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
                    SenderTypeServer::DownloadFile(url) => {
                        let bytes = req::download_file(&client, &url).await?;
                        tx.send(SenderTypeUi::File(bytes)).await.unwrap_or_default();
                    }
                }
                Ok::<(), reqwest::Error>(())
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
        tx.send(SenderTypeServer::GetResources).await.unwrap();
    });

    // start ui
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native(
        "shitboard",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(App {
                selected_tab: AppTabs::Sounds,
                resources: Vec::default(),
                send_server,
                rec_ui,
                error_modal: None,
            })
        }),
    )
}

struct App {
    selected_tab: AppTabs,
    resources: Vec<Resource>,
    // sender
    send_server: Sender<SenderTypeServer>,
    rec_ui: Receiver<SenderTypeUi>,
    // error
    error_modal: Option<Error>,
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
    DownloadFile(String),
}

#[derive(Debug, Default)]
pub enum SenderTypeUi {
    #[default]
    None,
    Resources(Vec<Resource>),
    File(Bytes),
    Error(Error),
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
                        }
                        // todo: this operation should be cached for performance reasons
                        SenderTypeUi::File(bytes) => {
                            play_audio(bytes);
                        }
                        SenderTypeUi::Error(err) => {
                            self.error_modal = Some(err);
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
                                if let Some(error_message) = self.error_modal.as_ref() {
                                    ui.label(format!("{}", error_message));
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

                    for resource in &self.resources {
                        if ui.button(&resource.title).clicked() {
                            let tx = self.send_server.clone();
                            let audio_file = resource.audio_file.to_string_lossy().to_string();
                            tokio::spawn(async move {
                                tx.send(SenderTypeServer::DownloadFile(audio_file))
                                    .await
                                    .unwrap();
                            });
                        }
                    }
                }
                AppTabs::Settings => {}
            }
        });
    }
}

/// Plays the audio in a different thread from given bytes.
fn play_audio(bytes: Bytes) {
    // todo: error handling
    thread::spawn(|| {
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).unwrap();

        sink.append(Decoder::new(Cursor::new(bytes)).unwrap());

        sink.sleep_until_end();
    });
}
