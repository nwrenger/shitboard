<script lang="ts">
	import {
		getModalStore,
		popup,
		type ModalSettings,
		type PopupSettings,
		type ToastSettings,
		getToastStore
	} from '@skeletonlabs/skeleton';
	import type { PageData } from './$types';
	import { audio_interface } from '$lib/stores';

	const toastStore = getToastStore();
	const modalStore = getModalStore();

	interface HTMLAudioElement {
		setSinkId?(sinkId: string): Promise<void>;
	}

	export let data: PageData;

	function create() {
		const modal: ModalSettings = {
			type: 'component',
			component: 'addModal'
		};

		modalStore.trigger(modal);
	}

	async function playAudio(path: string) {
		const audio = new Audio(path);
		audio.volume *= $audio_interface.volume;
		audio.play().catch((error) => {
			const t: ToastSettings = {
				message: 'Error playing audio: ' + error,
				background: 'variant-filled-error'
			};
			toastStore.trigger(t);
		});
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

<div class="container h-full mx-auto flex justify-center items-center">
	<div class="space-y-8 flex flex-col items-center lg:w-2/3 w-11/12 pb-6">
		<div class="grid sm:grid-cols-6 md:grid-cols-10 gap-4 grid-cols-5">
			<button
				class="btn variant-filled-primary p-2 rounded-md max-w-16 [&>*]:pointer-events-none"
				on:click={create}
				use:popup={{ ...popupHover, target: 'popupHover-plus' }}
				><i class="fa-solid fa-plus"></i></button
			>
			<div class="card p-4 variant-filled-primary" data-popup="popupHover-plus">
				<p>Add here new Sounds!</p>
				<div class="arrow variant-filled-primary" />
			</div>
			{#each data.resources as resource, i}
				<button
					class="btn variant-filled p-2 rounded-md max-w-16 [&>*]:pointer-events-none"
					on:click={() => playAudio('files/' + resource.audio_file)}
					use:popup={{ ...popupHover, target: 'popupHover-' + i }}
					><img
						src={'files/' + resource.picture_file}
						alt={resource.title.slice(0, 2)}
						class="rounded-sm aspect-square"
					/></button
				>
				<div class="card p-4 variant-filled" data-popup="popupHover-{i}">
					<p>{resource.title}</p>
					<div class="arrow variant-filled" />
				</div>
			{/each}
		</div>
	</div>
</div>
