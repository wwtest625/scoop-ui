import { invoke } from '@tauri-apps/api/core';

export interface ScoopApp {
    name: string;
    version: string;
    bucket: string;
    description: string;
    updated: number;        // Unix timestamp in milliseconds
    has_update: boolean;    // Whether an update is available
    install_size: number;   // Size in bytes
}

export interface SearchResult {
    name: string;
    version: string;
    bucket: string;
    description: string;
}

export async function getInstalledApps(): Promise<ScoopApp[]> {
    try {
        const apps = await invoke<ScoopApp[]>('get_installed_apps');
        return apps;
    } catch (error) {
        console.error('Failed to get installed apps:', error);
        return [];
    }
}

export async function searchApps(query: string): Promise<SearchResult[]> {
    try {
        const results = await invoke<SearchResult[]>('search_apps', { query });
        return results;
    } catch (error) {
        console.error('Failed to search apps:', error);
        return [];
    }
}

export async function updateApp(appName: string): Promise<string> {
    try {
        const result = await invoke<string>('update_app', { appName });
        return result;
    } catch (error) {
        console.error('Failed to update app:', error);
        throw error;
    }
}

export async function updateAllApps(): Promise<string> {
    try {
        const result = await invoke<string>('update_all_apps');
        return result;
    } catch (error) {
        console.error('Failed to update all apps:', error);
        throw error;
    }
}
