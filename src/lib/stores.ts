import { persisted } from 'svelte-persisted-store';

export let volume = persisted('volume', [1.0]);
