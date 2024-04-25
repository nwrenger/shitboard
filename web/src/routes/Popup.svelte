<script lang="ts">
	import { audio_interface } from '$lib/stores';
</script>

<div class="space-y-4">
	<div>
		<div class="flex">
			<span class="font-bold flex-auto">shitboard</span>
			<a
				class="anchor"
				href="https://github.com/nwrenger/shitboard"
				target="_blank"
				title="Repository"><i class="fa-solid fa-up-right-from-square" /></a
			>
		</div>
		<div class="flex">
			<span class="opacity-50 flex-auto">nwrenger</span>
			<a class="anchor" href="https://github.com/nwrenger" target="_blank" title="Profile"
				><i class="fa-solid fa-up-right-from-square" /></a
			>
		</div>
	</div>

	<p>A Chaos-Fueled Soundboard App.</p>
	<label class="label">
		<span>Volume</span>
		<input type="range" max="1.0" min="0.0" step="0.01" bind:value={$audio_interface.volume} />
	</label>
</div>
<div class="arrow bg-surface-100-800-token" />
