export interface Resource {
	title: string;
	audio_file: string;
	time_stamp: number;
}

export function error_message(error: string): string {
	switch (error) {
		case 'Arguments':
			return 'The user provided arguments are malformed';
		case 'FileOpen':
			return 'A file could not be found or opened';
		case 'AlreadyExists':
			return 'A file with that name already exists';
		case 'InvalidFileType':
			return 'An uploaded file has an invalid type';
		case 'Network':
			return 'Could not connect to server';
		case 'InvalidFormat':
			return 'Invalid file format';
		case 'NothingFound':
			return 'No matching results';
		case 'Conversion':
			return 'Conversion error, decoding, ...';
		default:
			return 'An unknown error has occurred.\nTry refreshing the page!';
	}
}

export function bufferToBase64(buffer: ArrayBuffer): string {
	let binary = '';
	const bytes = new Uint8Array(buffer);
	const len = bytes.byteLength;
	for (let i = 0; i < len; i++) {
		binary += String.fromCharCode(bytes[i]);
	}
	if (typeof window !== 'undefined' && typeof window.btoa === 'function') {
		return window.btoa(binary);
	} else {
		return '';
	}
}
