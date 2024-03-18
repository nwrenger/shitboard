import { localStorageStore } from '@skeletonlabs/skeleton';
import { type Writable } from 'svelte/store';

interface AudioInterface {
	volume: number;
}

export const audio_interface: Writable<AudioInterface> = localStorageStore('audio_interface', {
	volume: 1.0
});
