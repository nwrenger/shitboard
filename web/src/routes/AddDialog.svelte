<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Label } from '$lib/components/ui/label';
	import Input from '$lib/components/ui/input/input.svelte';
	import { Button } from '$lib/components/ui/button';
	import { CircleAlert, LoaderCircle, Plus } from 'lucide-svelte';
	import * as Alert from '$lib/components/ui/alert';
	import { bufferToBase64, showError, isError } from '$lib';
	import api from '$lib/api';

	export let tooltip;
	export let resources: api.Resource[];

	const maxSize = 2_000_000;

	let dialogOpen = false;
	let title: string = '';
	let audio_data: Uint8Array | undefined = undefined;
	let response: Promise<any>;

	// reset data
	$: if (dialogOpen == false) {
		title = '';
		audio_data = undefined;
	}

	async function add() {
		let response = await api.add_resource({
			title,
			audio_data: bufferToBase64(audio_data || new Uint8Array())
		});
		if (!isError(response)) {
			resources.push(response);
			resources = resources.sort((a: api.Resource, b: api.Resource) => b.time_stamp - a.time_stamp);
			// close dialog
			dialogOpen = false;
		} else {
			showError(response);
		}
	}

	function readFile(event: Event): Promise<Uint8Array> {
		let target = event.target as HTMLInputElement;
		let files = target.files;
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

	function outSideClick(event: Event) {
		const target = event.target as HTMLElement;
		if (!target.closest('#toaster')) {
			dialogOpen = false;
		} else {
			event.preventDefault();
		}
	}

	$: tooBig = audio_data && audio_data.length >= maxSize;
	$: filled = title?.length > 0 && (audio_data?.length || 0) > 0;
	$: valid = !tooBig && filled;
</script>

<Dialog.Root
	bind:open={dialogOpen}
	onOpenChange={(value) => (dialogOpen = value)}
	onOutsideClick={outSideClick}
>
	<Dialog.Trigger asChild let:builder={dialog}
		><Button builders={[tooltip, dialog]} size="icon"><Plus /></Button></Dialog.Trigger
	>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>Add Clip</Dialog.Title>
			<Dialog.Description>
				<p>
					Upload and verify your title and audio file; once uploaded, no changes can be made, nor
					deletions!
				</p>
				<div class="grid gap-4 py-4">
					<div class="space-y-1">
						<Label for="title" class="text-right">Title</Label>
						<Input
							bind:value={title}
							id="title"
							placeholder="The Universe and beyond..."
							class="col-span-3"
						/>
					</div>
					<div class="space-y-1">
						<Label for="audio-clip" class="text-right">Audio Clip</Label>
						<Input
							on:change={(e) => readFile(e).then((data) => (audio_data = data))}
							id="audio-clip"
							type="file"
							class="col-span-3"
						/>
					</div>
				</div>
			</Dialog.Description>
		</Dialog.Header>
		{#if tooBig}
			<Alert.Root variant="destructive">
				<CircleAlert class="h-4 w-4" />
				<Alert.Title>Error</Alert.Title>
				<Alert.Description>The file size cannot be greater than 2 MB</Alert.Description>
			</Alert.Root>
		{/if}
		<Dialog.Footer>
			<Button disabled={!valid} type="submit" on:click={async () => (response = add())}>
				{#await response}
					<LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
				{/await}
				Add
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
