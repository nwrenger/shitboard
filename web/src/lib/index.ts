import { toast } from 'svelte-sonner';
import api from './api';

export function handleResult<T>(result: api.Result<T>, predicate: (out: T) => void) {
    if (isError(result)) {
        showError(result);
    } else {
        predicate(result);
    }
}

export function isError(result: any): result is api.Error {
    const errors: string[] = Object.keys(api.Error);
    return errors.includes(result);
}

export function showError(error: api.Error) {
    toast.error(errorMessage(error), {
        duration: 5000,
        important: true,
        action: {
            label: 'Close',
            onClick: () => {}
        }
    });
}

export function errorMessage(error: api.Error): string {
    switch (error) {
        case api.Error.Arguments:
            return 'The user provided arguments are malformed';
        case api.Error.FileOpen:
            return 'A file could not be found or opened';
        case api.Error.AlreadyExists:
            return 'A file with that name already exists';
        case api.Error.InvalidFileType:
            return 'An uploaded file has an invalid type';
        case api.Error.Network:
            return 'Could not connect to server';
        case api.Error.InvalidFormat:
            return 'Invalid file format';
        case api.Error.NothingFound:
            return 'No matching results';
        case api.Error.Conversion:
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
