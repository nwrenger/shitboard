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
	import type { ActionResult } from '@sveltejs/kit';
	import { applyAction, deserialize } from '$app/forms';
	import { invalidateAll } from '$app/navigation';

	// Props
	/** Exposes parent props to this component. */
	export let parent: SvelteComponent;

	const modalStore = getModalStore();
	const toastStore = getToastStore();

	// Base Classes
	const cBase = 'card p-4 w-modal shadow-xl space-y-4';
	const cHeader = 'text-2xl font-bold';

	let response: Promise<any>;
	let title = '';
	let files_audio: FileList;
	let files_picture: FileList;
	let audio_data: Uint8Array;
	let picture_data: Uint8Array;

	async function add() {
		const audioDataEncoded = bufferToBase64(audio_data);
		const pictureDataEncoded = bufferToBase64(picture_data);
		let response = await fetch('?/add', {
			method: 'POST',
			body: JSON.stringify({
				title,
				audio_data: audioDataEncoded,
				picture_data: pictureDataEncoded
			}),
			headers: {
				'x-sveltekit-action': 'true',
				'Content-Length': '999999999999999',
				'content-length': '999999999999999'
			}
		});
		if (response.ok) {
			const result: ActionResult = deserialize(await response.text());
			if (result.type === 'success') {
				await invalidateAll();
			}
			applyAction(result);
			modalStore.close();
		} else {
			const t: ToastSettings = {
				message: (await response.json()).error.message,
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

		<!-- prettier-ignore -->
		<footer class="modal-footer {parent.regionFooter}">
        <button class="btn {parent.buttonNeutral}" on:click={parent.onClose}>Close</button>
        <button class="btn {parent.buttonPositive}" use:popup={popupHover} on:click={async () => response = add()} disabled={!(title && audio_data && picture_data)}><Spinner {response} /> Add</button>

		<div class="card p-4 variant-filled-error" data-popup="popupHover-add">
			<p>Everything you'll add stays here forever!</p>
			<div class="arrow variant-filled-error" />
		</div>
    </footer>
	</div>
{/if}
