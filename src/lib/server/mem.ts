import fs from 'fs';
import path from 'path';

export interface Resource {
	title: string;
	audio_file: string;
	picture_file: string;
	time_stamp: number;
}

interface Database {
	resources: Resource[];
}

let db: Database = { resources: [] };

function importFiles(directoryPath: string) {
	const items = fs.readdirSync(directoryPath, { withFileTypes: true });

	items.forEach((item) => {
		const fullPath = path.join(directoryPath, item.name);
		if (item.isDirectory()) {
			importFiles(fullPath);
		} else if (path.extname(item.name) === '.json') {
			const fileData = fs.readFileSync(fullPath, 'utf8');
			const resource: Resource = JSON.parse(fileData);

			db.resources.push(resource);
		}
	});
}

importFiles('static/files');

export function getResources(): Resource[] {
	return db.resources.sort((a, b) => a.time_stamp - b.time_stamp);
}

function getFileExtension(data: Buffer): string {
	// MP3 with ID3 tag
	if (data.toString('hex', 0, 3) === 'fffbd0') {
		return 'mp3';
	}

	// First 4 bytes for most file types
	const signature = data.toString('hex', 0, 4);
	switch (signature) {
		case '89504e47':
			return 'png';
		case 'ffd8ffe0':
		case 'ffd8ffe1':
		case 'ffd8ffe2':
			return 'jpg';
		case '47494638':
			return 'gif';
		case '52494646':
			if (data.toString('hex', 8, 12) === '57415645') {
				return 'wav';
			}
			break;
		case '4f676753':
			return 'ogg';
	}
	return 'unknown';
}

export function addResource(title: string, audio_data: Buffer, picture_data: Buffer): Resource[] {
	let date = new Date();
	let root = 'static/files/';
	let audioExtension = getFileExtension(audio_data);
	let pictureExtension = getFileExtension(picture_data);

	// Check if the detected extensions are valid for audio and picture
	if (!['mp3', 'wav', 'ogg'].includes(audioExtension)) {
		throw new Error('Unsupported audio format');
	}
	if (!['jpg', 'png', 'gif'].includes(pictureExtension)) {
		throw new Error('Unsupported picture format');
	}

	// Construct file names with detected extensions
	let newResource: Resource = {
		title,
		audio_file: `${title}.${audioExtension}`,
		picture_file: `${title}.${pictureExtension}`,
		time_stamp: date.getTime()
	};

	// Check if the files already exist
	if (
		fs.existsSync(path.join(root, newResource.audio_file)) ||
		fs.existsSync(path.join(root, newResource.picture_file))
	) {
		throw new Error('A resource with this name already exists!');
	}

	// Save the files
	fs.writeFileSync(path.join(root, newResource.audio_file), audio_data);
	fs.writeFileSync(path.join(root, newResource.picture_file), picture_data);
	fs.writeFileSync(path.join(root, `${title}.json`), JSON.stringify(newResource));

	db.resources.push(newResource);
	return db.resources;
}
