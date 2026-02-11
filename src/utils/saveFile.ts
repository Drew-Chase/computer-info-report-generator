import {save} from "@tauri-apps/plugin-dialog";
import {writeTextFile} from "@tauri-apps/plugin-fs";

interface SaveFileOptions {
    content: string;
    defaultName: string;
    filterName: string;
    extensions: string[];
}

export async function saveFile({content, defaultName, filterName, extensions}: SaveFileOptions): Promise<boolean> {
    const path = await save({
        defaultPath: defaultName,
        filters: [{name: filterName, extensions}],
    });

    if (!path) return false;

    await writeTextFile(path, content);
    return true;
}
