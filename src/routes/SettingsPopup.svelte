<script lang="ts">
	import { audio_interface } from '$lib/stores';
	import { Accordion, AccordionItem } from '@skeletonlabs/skeleton';
</script>

<div class="space-y-2">
	<div>
		<p class="font-bold">shitboard</p>
		<p class="opacity-50">@nwrenger</p>
	</div>
	<Accordion>
		<AccordionItem>
			<svelte:fragment slot="lead"><i class="fa-solid fa-circle-info"></i></svelte:fragment>
			<svelte:fragment slot="summary">Info</svelte:fragment>
			<svelte:fragment slot="content">
				<p>
					A Chaos-Fueled Soundboard App where creativity and permanence collide. Unleash your sounds
					into the wild with no take-backs. Upload anything; once it's up, it's there forever!
				</p>
			</svelte:fragment>
		</AccordionItem>
		<!-- svelte-ignore a11y-label-has-associated-control -->
		<AccordionItem>
			<svelte:fragment slot="lead"><i class="fa-solid fa-gear"></i></svelte:fragment>
			<svelte:fragment slot="summary">Settings</svelte:fragment>
			<svelte:fragment slot="content">
				<label class="label">
					<span>Volume</span>
					<input type="range" max="1.0" step="0.01" bind:value={$audio_interface.volume} />
				</label>
			</svelte:fragment>
		</AccordionItem>
	</Accordion>
</div>
<div class="arrow bg-surface-100-800-token" />
