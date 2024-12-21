<script lang="ts">
	import { buttonVariants } from '$lib/components/ui/button';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { Pause, Play, Plus } from 'lucide-svelte';
	import db from '$lib/data.json';
	import { volume } from '$lib/stores';
	import Button from '$lib/components/ui/button/button.svelte';
	import { GITHUB_URL } from '$lib';

	let audio: HTMLAudioElement;
	let currentAudio: string | undefined;

	$: if (audio) audio.volume = $volume[0];

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

<div class="grid w-full grid-cols-4 gap-4 sm:grid-cols-6 md:grid-cols-12">
	<Tooltip.Provider>
		<Tooltip.Root delayDuration={0}>
			<Tooltip.Trigger>
				{#snippet child({ props })}
					<Button size="icon" target="_blank" href="{GITHUB_URL}/shitboard/pulls" {...props}>
						<Plus class="!size-5" />
					</Button>
				{/snippet}
			</Tooltip.Trigger>
			<Tooltip.Content>
				<p>Add new Sound</p>
			</Tooltip.Content>
		</Tooltip.Root>
	</Tooltip.Provider>
	{#each db.resources as resource}
		<Tooltip.Provider>
			<Tooltip.Root delayDuration={0} disableCloseOnTriggerClick>
				<Tooltip.Trigger
					class={buttonVariants({ variant: 'outline', size: 'icon' })}
					onclick={() => {
						if (currentAudio == `files/${resource.file_name}`) {
							currentAudio = undefined;
						} else {
							currentAudio = `files/${resource.file_name}`;
						}
					}}
				>
					{#if currentAudio == `files/${resource.file_name}`}
						<Pause class="!size-5" color={generateColor(resource.time_stamp)} />
					{:else}
						<Play class="!size-5" color={generateColor(resource.time_stamp)} />
					{/if}
				</Tooltip.Trigger>
				<Tooltip.Content>
					<p>{resource.title}</p>
				</Tooltip.Content>
			</Tooltip.Root>
		</Tooltip.Provider>
	{/each}
</div>
{#if currentAudio}
	<audio bind:this={audio} autoplay src={currentAudio} on:ended={() => (currentAudio = undefined)}>
		<source src={currentAudio} />
	</audio>
{/if}
