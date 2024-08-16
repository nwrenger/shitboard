const PREFIX = '/api';

namespace api {
    export interface Files {
        title: string;
        audio_data: string;
    }

    export interface Resource {
        title: string;
        audio_file: string;
        time_stamp: number;
    }

    /**
        The api compatible error type.
        
        More specific error messages are removed to be api compatible.
        Those messages are logged however.
    */
    export enum Error {
        /**
            The user provided arguments are malformed
        */
        Arguments = "Arguments",
        /**
            A file could not be found or opened
        */
        FileOpen = "FileOpen",
        /**
            A file with that name already exists
        */
        AlreadyExists = "AlreadyExists",
        /**
            An uploaded file has an invalid type
        */
        InvalidFileType = "InvalidFileType",
        /**
            Could not connect to server
        */
        Network = "Network",
        /**
            Invalid file format
        */
        InvalidFormat = "InvalidFormat",
        /**
            No matching results
        */
        NothingFound = "NothingFound",
        /**
            Conversion error, decoding, ...
        */
        Conversion = "Conversion",
    }

    /**
        Result type using the api error.
    */
    export type Result<T> = T | Error;

    async function fetch_api(endpoint: string, options: RequestInit): Promise<any> {
        const response = await fetch(endpoint, {
            headers: {
                "Content-Type": "application/json",
                ...options.headers,
            },
            ...options,
        });
        if (response.headers.get('Content-Length') === '0') {
			return;
		} else {
			return response.json();
		}
    }

    export async function add_resource(data: Files): Promise<Result<Resource>> {
        return fetch_api(`${PREFIX}/resource`, {
            method: "POST", 
            body: JSON.stringify(data)
        });
    }

    export async function resources(): Promise<Result<Resource[]>> {
        return fetch_api(`${PREFIX}/resource`, {
            method: "GET", 
        });
    }
}

export default api;
