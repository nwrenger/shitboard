<script lang="ts">
	import { error_message, type Resource } from '$lib';
	import { Button } from '$lib/components/ui/button';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { Pause, Play } from 'lucide-svelte';
	import { onMount } from 'svelte';
	import { toast } from 'svelte-sonner';
	import AddDialog from './AddDialog.svelte';
	import { volume } from '$lib/stores';

	let audio: HTMLAudioElement;
	let resources: Resource[] = [];
	let currentAudio: string | undefined;

	$: if (audio) audio.volume = $volume[0];

	// get initial data && state
	onMount(async () => {
		let response = await fetch('/api/resource', {
			method: 'GET'
		});
		if (response.ok) {
			resources = (await response.json()).sort(
				(a: Resource, b: Resource) => b.time_stamp - a.time_stamp
			);
		} else {
			toast.error(error_message(await response.text()), {
				duration: 5000,
				important: true,
				action: {
					label: 'Close',
					onClick: () => {}
				}
			});
		}
	});

	function generateColor(time_stamp: number) {
		let red = (time_stamp & 0xff0000) >> 16;
		let green = (time_stamp & 0x00ff00) >> 8;
		let blue = time_stamp & 0x0000ff;

		return `#${red.toString(16).padStart(2, '0')}${green.toString(16).padStart(2, '0')}${blue.toString(16).padStart(2, '0')}`;
	}
</script>

<svelte:head>
	<title>shitboard</title>
	<meta
		name="description"
		content=" A Chaos-Fueled Soundboard App where creativity and permanence collide."
	/>
</svelte:head>

<div class="container mx-auto flex max-w-6xl flex-col items-center space-y-8 p-4">
	<div class="grid w-full grid-cols-4 gap-4 sm:grid-cols-6 md:grid-cols-12">
		<Tooltip.Root openDelay={0}>
			<Tooltip.Trigger asChild let:builder={tooltip}>
				<AddDialog bind:resources {tooltip} />
			</Tooltip.Trigger>
			<Tooltip.Content>
				<p>Add new Sound</p>
			</Tooltip.Content>
		</Tooltip.Root>
		{#each resources as resource}
			<Tooltip.Root openDelay={0} closeOnPointerDown={false}>
				<Tooltip.Trigger asChild let:builder>
					<Button
						builders={[builder]}
						size="icon"
						variant="outline"
						on:click={() => {
							if (currentAudio == resource.audio_file) {
								currentAudio = undefined;
							} else {
								currentAudio = resource.audio_file;
							}
						}}
					>
						{#if currentAudio == resource.audio_file}
							<Pause size={20} color={generateColor(resource.time_stamp)} />
						{:else}
							<Play size={20} color={generateColor(resource.time_stamp)} />
						{/if}
					</Button>
				</Tooltip.Trigger>
				<Tooltip.Content>
					<p>{resource.title}</p>
				</Tooltip.Content>
			</Tooltip.Root>
		{/each}
	</div>
	{#if currentAudio}
		<audio
			bind:this={audio}
			autoplay
			src={currentAudio}
			on:ended={() => (currentAudio = undefined)}
		>
			<source src={currentAudio} />
		</audio>
	{/if}
</div>
