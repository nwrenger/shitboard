<script lang="ts">
	import { onMount } from 'svelte';
	export let src: string;
	export let alt = '';

	export let width = 'w-full';
	export let height = 'h-full';
	export let ratio = 'aspect-auto';
	export let rounded = 'rounded-lg';
	const base = `${width} ${height} ${ratio} ${rounded}`;

	let loaded = false;
	let mounted = false;

	function handleLoad() {
		loaded = true;
	}

	onMount(() => (mounted = true));
</script>

{#if !loaded}
	<div class="{base} placeholder animate-pulse" />
{/if}

{#if mounted}
	<img
		{src}
		{alt}
		on:load={handleLoad}
		class="{base} object-cover shadow-md {!loaded ? 'hidden' : ''}"
	/>
{/if}
