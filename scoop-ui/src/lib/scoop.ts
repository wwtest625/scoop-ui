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

export async function updateScoop(): Promise<string> {
    try {
        const result = await invoke<string>('update_scoop');
        return result;
    } catch (error) {
        console.error('Failed to update scoop:', error);
        throw error;
    }
}

export async function getAppSizes(appNames: string[]): Promise<Record<string, number>> {
    try {
        const result = await invoke<Record<string, number>>('get_app_sizes', { appNames });
        return result;
    } catch (error) {
        console.error('Failed to get app sizes:', error);
        return {};
    }
}

export async function searchLocalApps(query: string): Promise<SearchResult[]> {
    try {
        const results = await invoke<SearchResult[]>('search_local_packets', { query });
        return results;
    } catch (error) {
        console.error('Failed to search local apps:', error);
        return [];
    }
}

export interface ScoopBucket {
    name: string;
    source: string;
    updated: number;
}

export async function getBuckets(): Promise<ScoopBucket[]> {
    try {
        const buckets = await invoke<ScoopBucket[]>('get_buckets');
        return buckets;
    } catch (error) {
        console.error('Failed to get buckets:', error);
        return [];
    }
}

export async function addBucket(name: string, url: string | null): Promise<string> {
    try {
        const result = await invoke<string>('add_bucket', { name, url });
        return result;
    } catch (error) {
        console.error('Failed to add bucket:', error);
        throw error;
    }
}

export async function removeBucket(name: string): Promise<string> {
    try {
        const result = await invoke<string>('remove_bucket', { name });
        return result;
    } catch (error) {
        console.error('Failed to remove bucket:', error);
        throw error;
    }
}

export async function installApp(appName: string): Promise<string> {
    try {
        const result = await invoke<string>('install_app', { appName });
        return result;
    } catch (error) {
        console.error('Failed to install app:', error);
        throw error;
    }
}

export async function uninstallApp(appName: string): Promise<string> {
    try {
        const result = await invoke<string>('uninstall_app', { appName });
        return result;
    } catch (error) {
        console.error('Failed to uninstall app:', error);
        throw error;
    }
}

export async function checkDependencies(appName: string, bucket?: string): Promise<string[]> {
    try {
        const result = await invoke<string[]>('check_dependencies', { appName, bucket });
        return result;
    } catch (error) {
        console.error('Failed to check dependencies:', error);
        return [];
    }
}

export async function isAppInstalled(appName: string): Promise<boolean> {
    try {
        const result = await invoke<boolean>('is_app_installed', { appName });
        return result;
    } catch (error) {
        console.error('Failed to check if app is installed:', error);
        return false;
    }
}

export async function checkUpdatesAsync(): Promise<string> {
    try {
        const result = await invoke<string>('check_updates_async');
        return result;
    } catch (error) {
        console.error('Failed to check updates:', error);
        throw error;
    }
}
