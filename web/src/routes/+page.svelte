<script lang="ts">
	import {
		getModalStore,
		popup,
		type ModalSettings,
		type PopupSettings,
		type ToastSettings,
		getToastStore
	} from '@skeletonlabs/skeleton';
	import { audio_interface } from '$lib/stores';
	import { onMount } from 'svelte';
	import { clamp, error_message } from '$lib/utils';

	interface Resource {
		title: string;
		audio_file: string;
		time_stamp: number;
	}

	const toastStore = getToastStore();
	const modalStore = getModalStore();

	let resources: Resource[] = [];

	// get initial data
	onMount(async () => {
		let response = await fetch('/api/resource', {
			method: 'GET'
		});
		if (response.ok) {
			resources = (await response.json()).sort(
				(a: Resource, b: Resource) => b.time_stamp - a.time_stamp
			);
		} else {
			const t: ToastSettings = {
				message: error_message(await response.text()),
				background: 'variant-filled-error'
			};
			toastStore.trigger(t);
		}
	});

	function create() {
		const modal: ModalSettings = {
			type: 'component',
			component: 'addModal',
			// response
			response: (r: Resource | undefined) => {
				if (r) {
					resources.push(r);
					resources.sort((a: Resource, b: Resource) => b.time_stamp - a.time_stamp);
					resources = resources;
				}
			}
		};

		modalStore.trigger(modal);
	}

	async function playAudio(path: string) {
		const audio = new Audio(path);
		audio.volume =
			$audio_interface.volume !== 0
				? clamp(0.01 * Math.exp(Math.log(1.0 / 0.01) * $audio_interface.volume), 0.01, 1.0)
				: 0.0;
		audio.play().catch((error) => {
			const t: ToastSettings = {
				message: 'Error playing audio: ' + error,
				background: 'variant-filled-error'
			};
			toastStore.trigger(t);
		});
	}

	function generateColor(timestamp: number) {
		const str = timestamp.toString();
		let hash = 0;
		for (let i = 0; i < str.length; i++) {
			const char = str.charCodeAt(i);
			hash = (hash << 5) - hash + char;
			hash = hash & hash;
		}
		const hue = Math.abs(hash % 360);
		return `hsl(${hue}, 75%, 40%)`;
	}

	const popupHover: PopupSettings = {
		event: 'hover',
		placement: 'top',
		target: ''
	};
</script>

<svelte:head>
	<title>shitboard</title>
	<meta
		name="description"
		content="A Chaos-Fueled Soundboard App where creativity and permanence collide. Unleash your sounds
	into the wild with no take-backs. Upload anything; once it's up, it's there forever!"
	/>
</svelte:head>

<div class="container space-y-8 flex flex-col items-center !max-w-6xl mx-auto p-4">
	<div class="grid sm:grid-cols-6 md:grid-cols-12 gap-2 grid-cols-4 w-full">
		<button
			class="btn-icon variant-filled-primary p-2 rounded-md [&>*]:pointer-events-none"
			on:click={create}
			use:popup={{ ...popupHover, target: 'popupHover-plus' }}
			><i class="fa-solid fa-plus"></i></button
		>
		<div class="card p-4 variant-filled-primary" data-popup="popupHover-plus">
			<p>Add here new Sounds!</p>
			<div class="arrow variant-filled-primary" />
		</div>
		{#each resources as resource, i}
			<button
				class="btn-icon variant-filled p-2 rounded-md [&>*]:pointer-events-none"
				on:click={() => playAudio(resource.audio_file)}
				use:popup={{ ...popupHover, target: 'popupHover-' + i }}
			>
				<i class="fa-solid fa-play" style="color: {generateColor(resource.time_stamp)}"></i>
			</button>
			<div class="card p-4 variant-filled" data-popup="popupHover-{i}">
				<p>{resource.title}</p>
				<div class="arrow variant-filled" />
			</div>
		{/each}
	</div>
</div>
