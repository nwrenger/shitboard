<script>
	import * as Avatar from '$lib/components/ui/avatar';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { Toaster } from '$lib/components/ui/sonner';
	import '../app.pcss';
	import { Button } from '$lib/components/ui/button';
	import { ExternalLink, Moon, Sun } from 'lucide-svelte';
	import { ModeWatcher, toggleMode, mode } from 'mode-watcher';
	import { Slider } from '$lib/components/ui/slider';
	import { volume } from '$lib/stores';
</script>

<ModeWatcher />

<Toaster id="toaster" theme={$mode} class={'z-[100]'} />

<div class="container h-14 max-w-6xl pl-2 pr-2 md:pl-4 md:pr-4">
	<div class="flex h-[70px] items-center justify-between gap-3">
		<div class="flex items-center gap-1.5">
			<Tooltip.Root openDelay={0} closeOnPointerDown={false}>
				<Tooltip.Trigger>
					<Avatar.Root>
						<Avatar.Image src="favicon.png" alt="shitboard" />
						<Avatar.Fallback>SB</Avatar.Fallback>
					</Avatar.Root>
				</Tooltip.Trigger>
				<Tooltip.Content>
					<div class="space-y-4">
						<div>
							<div class="flex">
								<span class="flex-auto font-bold">shitboard</span>
								<a
									class="anchor"
									href="https://github.com/nwrenger/shitboard"
									target="_blank"
									title="Repository"><ExternalLink size={16} stroke-width={2.5} /></a
								>
							</div>
							<div class="flex">
								<span class="flex-auto opacity-50">nwrenger</span>
								<a class="anchor" href="https://github.com/nwrenger" target="_blank" title="Profile"
									><ExternalLink size={16} stroke-width={2.5} /></a
								>
							</div>
						</div>

						<p>A Chaos-Fueled Soundboard App.</p>
					</div>
				</Tooltip.Content>
			</Tooltip.Root>
		</div>
		<div class="flex w-2/3 items-center justify-end md:w-1/2">
			<Slider
				bind:value={$volume}
				max={1.0}
				min={0.0}
				step={0.01}
				class="mr-4 w-9/12 md:w-2/3 xl:w-1/3"
			/>

			<Button on:click={toggleMode} variant="outline" size="icon">
				<Sun
					class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
				/>
				<Moon
					class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
				/>
				<span class="sr-only">Toggle theme</span>
			</Button>
		</div>
	</div>
</div>

<slot />
