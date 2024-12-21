import fs from 'fs';
import path from 'path';

const BASE_FILES_DIR = 'static/files';

export interface Resource {
	title: string;
	file_name: string;
	time_stamp: number;
}

export interface Database {
	resources: Resource[];
}

const db: Database = { resources: [] };

const re = fs.readdirSync(BASE_FILES_DIR, { withFileTypes: true });

re.forEach((item: { name: any }) => {
	if (path.extname(item.name) === '.json') {
		const fileData = fs.readFileSync(path.join(BASE_FILES_DIR, item.name), 'utf8');
		const resource: Resource = JSON.parse(fileData);
		db.resources.push(resource);
	}
});

fs.writeFile('src/lib/data.json', JSON.stringify(db), (err: any) => {
	if (err) throw err;
});
