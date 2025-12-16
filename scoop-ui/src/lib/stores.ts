import { writable } from 'svelte/store';
import { getInstalledApps, getBuckets, checkUpdatesAsync, type ScoopApp, type ScoopBucket } from './scoop';

// Stores
export const installedAppsStore = writable<ScoopApp[]>([]);
export const bucketsStore = writable<ScoopBucket[]>([]);
export const appLoadingStore = writable<boolean>(true); // Global app loading state
export const initializationErrorStore = writable<string | null>(null);

const CACHE_KEY_APPS = 'scoop_installed_apps';
const CACHE_KEY_BUCKETS = 'scoop_buckets_cache';

// Helper to load data
export async function initializeData() {
    console.log('Initializing Scoop UI data...');
    appLoadingStore.set(true);
    initializationErrorStore.set(null);

    // Load from cache first for instant UI
    loadFromCache();

    // Fetch fresh data in parallel
    try {
        const [apps, buckets] = await Promise.all([
            getInstalledApps(),  // Now fast! No blocking update check
            getBuckets()
        ]);

        // Update stores
        installedAppsStore.set(apps);
        bucketsStore.set(buckets);

        // Update cache
        saveToCache(apps, buckets);

        console.log(`Loaded ${apps.length} apps and ${buckets.length} buckets.`);

        // Start async update check in background (non-blocking)
        checkUpdatesAsync().catch(err => {
            console.warn('Background update check failed:', err);
        });
    } catch (err) {
        console.error('Failed to initialize data:', err);
        initializationErrorStore.set(String(err));
    } finally {
        appLoadingStore.set(false);
    }
}

function loadFromCache() {
    try {
        const cachedApps = localStorage.getItem(CACHE_KEY_APPS);
        if (cachedApps) {
            const parsed = JSON.parse(cachedApps);
            // Handle both old format {data: [], timestamp: ...} and new format []
            const data = Array.isArray(parsed) ? parsed : (parsed.data || []);
            installedAppsStore.set(data);
        }

        const cachedBuckets = localStorage.getItem(CACHE_KEY_BUCKETS);
        if (cachedBuckets) {
            bucketsStore.set(JSON.parse(cachedBuckets));
        }
    } catch (e) {
        console.warn('Failed to load from cache', e);
    }
}

function saveToCache(apps: ScoopApp[], buckets: ScoopBucket[]) {
    try {
        localStorage.setItem(CACHE_KEY_APPS, JSON.stringify(apps)); // Simplify cache format
        localStorage.setItem(CACHE_KEY_BUCKETS, JSON.stringify(buckets));
    } catch (e) {
        console.warn('Failed to save to cache', e);
    }
}
