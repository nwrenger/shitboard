<script lang="ts">
	import '../app.css';
	import * as Avatar from '$lib/components/ui/avatar';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Button } from '$lib/components/ui/button';
	import { Moon, Sun, User, Github } from 'lucide-svelte';
	import { ModeWatcher, toggleMode } from 'mode-watcher';
	import { Slider } from '$lib/components/ui/slider';
	import { volume } from '$lib/stores';
	import { GITHUB_URL } from '$lib';

	let { children } = $props();
</script>

<ModeWatcher disableTransitions={false} />

<header
	class="sticky top-0 z-50 w-full border-b border-border/40 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60"
>
	<div class="container max-w-6xl pl-4 pr-4">
		<div class="flex h-[70px] items-center justify-between gap-3">
			<div class="flex items-center gap-1.5">
				<DropdownMenu.Root>
					<DropdownMenu.Trigger>
						<Avatar.Root>
							<Avatar.Image src="favicon.png" alt="shitboard" />
							<Avatar.Fallback>SB</Avatar.Fallback>
						</Avatar.Root>
					</DropdownMenu.Trigger>
					<DropdownMenu.Content>
						<DropdownMenu.Group>
							<DropdownMenu.Label>
								<a href="/" class="hover:underline" data-sveltekit-reload>shitboard</a>
							</DropdownMenu.Label>
							<DropdownMenu.Separator />
							<DropdownMenu.Item>
								{#snippet child()}
									<a
										href="{GITHUB_URL}/shitboard"
										target="_blank"
										class="hover:bg-accent hover:text-accent-foreground relative flex cursor-default select-none items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-none transition-colors data-[disabled]:pointer-events-none data-[disabled]:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0"
									>
										<Github class="mr-2 h-4 w-4" />
										<span>Github</span>
									</a>
								{/snippet}
							</DropdownMenu.Item>
							<DropdownMenu.Item>
								{#snippet child()}
									<a
										href={GITHUB_URL}
										target="_blank"
										class="hover:bg-accent hover:text-accent-foreground relative flex cursor-default select-none items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-none transition-colors data-[disabled]:pointer-events-none data-[disabled]:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0"
									>
										<User class="mr-2 h-4 w-4" />
										<span>Profile</span>
									</a>
								{/snippet}
							</DropdownMenu.Item>
						</DropdownMenu.Group>
					</DropdownMenu.Content>
				</DropdownMenu.Root>
			</div>
			<div class="flex w-2/3 items-center justify-end md:w-1/2">
				<Slider
					bind:value={$volume}
					max={1.0}
					min={0.0}
					step={0.01}
					class="mr-4 w-9/12 md:w-1/2 xl:w-1/3"
				/>

				<Button onclick={toggleMode} variant="outline" size="icon">
					<Sun
						class="!size-5 h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
					/>
					<Moon
						class="!size-5 absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
					/>
					<span class="sr-only">Toggle theme</span>
				</Button>
			</div>
		</div>
	</div>
</header>

<div class="container mx-auto flex max-w-6xl flex-col items-center space-y-8 p-4">
	{@render children()}
</div>
