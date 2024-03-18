import { getResources } from '$lib/server/mem';
import { addResource, type Resource } from '$lib/server/mem';
import { error } from '@sveltejs/kit';
import { base64ToBuffer } from '$lib/utils';
import type { Actions } from './$types';

export function load() {
	return {
		resources: getResources()
	};
}

export const actions = {
	add: async ({ request }) => {
		const data = await request.json();

		if (data.title && data.audio_data && data.picture_data) {
			const audioDataDecoded = base64ToBuffer(data.audio_data);
			const pictureDataDecoded = base64ToBuffer(data.picture_data);
			try {
				const response: Resource[] = addResource(data.title, audioDataDecoded, pictureDataDecoded);
				return {
					resources: response
				};
			} catch (e: unknown) {
				return error(400, (e as Error).message);
			}
		} else {
			return error(400, 'Invalid Data Type');
		}
	}
} satisfies Actions;
