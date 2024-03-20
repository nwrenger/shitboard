<script lang="ts">
	import type { SvelteComponent } from 'svelte';
	// Stores
	import {
		getModalStore,
		popup,
		type PopupSettings,
		type ToastSettings
	} from '@skeletonlabs/skeleton';
	import { getToastStore } from '@skeletonlabs/skeleton';
	import Spinner from '$lib/Spinner.svelte';
	import { bufferToBase64 } from '$lib/utils';

	// Props
	/** Exposes parent props to this component. */
	export let parent: SvelteComponent;

	const modalStore = getModalStore();
	const toastStore = getToastStore();
	const maxSize = 2_000_000;

	// Base Classes
	const cBase = 'card p-4 w-modal shadow-xl space-y-4';
	const cHeader = 'text-2xl font-bold';

	let response: Promise<any>;
	let title = '';
	let files_audio: FileList;
	let files_picture: FileList;
	let audio_data: Uint8Array;
	let picture_data: Uint8Array;

	function encodedData(title: string, audio_data: Uint8Array, picture_data: Uint8Array): string {
		const audioDataEncoded = bufferToBase64(audio_data);
		const pictureDataEncoded = bufferToBase64(picture_data);

		return JSON.stringify({
			title,
			audio_data: audioDataEncoded,
			picture_data: pictureDataEncoded
		});
	}

	async function add() {
		try {
			let body = encodedData(title, audio_data, picture_data);
			let response = await fetch('/api/resource', {
				method: 'POST',
				body,
				headers: {
					'Content-Type': 'application/json'
				}
			});
			if (response.ok) {
				let data = await response.json();
				if ($modalStore[0].response) $modalStore[0].response(data);
				modalStore.close();
			} else {
				const t: ToastSettings = {
					message: await response.text(),
					background: 'variant-filled-error'
				};
				toastStore.trigger(t);
			}
		} catch (e) {
			const t: ToastSettings = {
				message: e as string,
				background: 'variant-filled-error'
			};
			toastStore.trigger(t);
		}
	}

	function readFile(files: FileList): Promise<Uint8Array> {
		return new Promise((resolve, reject) => {
			if (files && files.length > 0) {
				const file = files[0];
				const reader = new FileReader();

				reader.onload = (event) => {
					if (event.target && typeof event.target.result === 'object') {
						const arrayBuffer = event.target.result;
						const buffer = new Uint8Array(arrayBuffer ?? []);
						resolve(buffer);
					} else {
						reject(new Error('Failed to load file'));
					}
				};

				reader.onerror = () => {
					reject(new Error('Error reading file'));
				};

				reader.readAsArrayBuffer(file);
			} else {
				reject(new Error('No file provided'));
			}
		});
	}

	const popupHover: PopupSettings = {
		event: 'hover',
		placement: 'top',
		target: 'popupHover-add'
	};
</script>

<!-- @component This is a Modal for adding a Resource. -->

{#if $modalStore[0]}
	<div class={cBase}>
		<header class={cHeader}>Add</header>

		<label class="label">
			<span>Title</span>
			<input class="input" type="text" placeholder="Title" bind:value={title} />
		</label>

		<label class="label">
			<span>Audio File</span>
			<input
				class="input"
				type="file"
				bind:files={files_audio}
				on:change={() => readFile(files_audio).then((data) => (audio_data = data))}
			/>
		</label>

		<label class="label">
			<span>Picture</span>
			<input
				class="input"
				type="file"
				bind:files={files_picture}
				on:change={() => readFile(files_picture).then((data) => (picture_data = data))}
			/>
		</label>

		{#if encodedData(title, audio_data, picture_data).length > maxSize}
			<p class="text-error-500 italic text-center">
				Please Note that the combined file sizes cannot be greater than {maxSize / 1_000_000} MB!
			</p>
		{/if}

		<!-- prettier-ignore -->
		<footer class="modal-footer {parent.regionFooter}">
        <button class="btn {parent.buttonNeutral}" on:click={parent.onClose}>Close</button>
        <button class="btn {parent.buttonPositive}" use:popup={popupHover} on:click={async () => response = add()} disabled={!(title && audio_data && picture_data && encodedData(title, audio_data, picture_data).length < maxSize)}><Spinner {response} /> Add</button>

		<div class="card p-4 variant-filled-warning" data-popup="popupHover-add">
			<p>Everything you'll add stays here forever!</p>
			<div class="arrow variant-filled-warning" />
		</div>
    </footer>
	</div>
{/if}
