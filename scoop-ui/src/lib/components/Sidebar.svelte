<script>
    import { page } from '$app/stores';
</script>

<aside class="sidebar">
    <!-- App Header/Title -->
    <div class="header">
        <h1 class="app-title">Scoop UI</h1>
        <p class="app-version">v1.2.0</p>
    </div>

    <!-- Search Bar -->
    <div class="search-container">
        <div class="search-input-wrapper">
            <div class="search-icon">
                <span class="material-symbols-outlined">search</span>
            </div>
            <input 
                class="search-input" 
                id="search" 
                placeholder="Search Scoop..." 
                type="text"
                on:keydown={(e) => {
                    if (e.key === 'Enter') {
                        // @ts-ignore
                        const query = e.target.value;
                        if (query.trim()) {
                            import('$app/navigation').then(({ goto }) => {
                                goto(`/search?q=${encodeURIComponent(query)}`);
                            });
                        }
                    }
                }} 
            />
        </div>
    </div>

    <!-- Navigation -->
    <nav class="nav-list">
        <a class="nav-item" href="/" class:active={$page.url.pathname === '/'}>
            <span class="material-symbols-outlined icon">rocket_launch</span>
            <span class="label">Discover</span>
        </a>
        <a class="nav-item" href="/installed" class:active={$page.url.pathname.startsWith('/installed')}>
            <span class="material-symbols-outlined icon">deployed_code</span>
            <span class="label">Installed</span>
        </a>
        <!-- <a class="nav-item" href="/updates" class:active={$page.url.pathname.startsWith('/updates')}>
            <span class="material-symbols-outlined icon">update</span>
            <span class="label">Updates</span>
            <span class="badge">3</span>
        </a> -->
        <a class="nav-item" href="/buckets" class:active={$page.url.pathname.startsWith('/buckets')}>
            <span class="material-symbols-outlined icon">folder_open</span>
            <span class="label">Buckets</span>
        </a>
    </nav>

    <!-- Bottom Actions -->
    <div class="bottom-actions">
        <a class="nav-item" href="/settings" class:active={$page.url.pathname.startsWith('/settings')}>
            <span class="material-symbols-outlined icon">settings</span>
            <span class="label">Settings</span>
        </a>
    </div>
</aside>

<style>
    .sidebar {
        width: 16rem; /* 64 * 0.25rem = 16rem = 256px */
        flex-shrink: 0;
        background-color: var(--bg-subtle);
        border-right: 1px solid var(--border);
        display: flex;
        flex-direction: column;
        height: 100%;
        padding-top: 2rem;
        padding-bottom: 1rem;
        padding-left: 1rem;
        padding-right: 1rem;
        overflow-y: auto;
    }

    .header {
        margin-bottom: 1.5rem;
        padding-left: 0.5rem;
        padding-right: 0.5rem;
    }

    .app-title {
        font-size: 1.125rem;
        font-weight: 700;
        letter-spacing: -0.025em;
        color: var(--text-main);
        margin: 0;
    }

    .app-version {
        font-size: 0.75rem;
        font-weight: 400;
        color: var(--text-muted);
        margin: 0;
    }

    .search-container {
        margin-bottom: 1.5rem;
    }

    .search-input-wrapper {
        position: relative;
        display: flex;
        align-items: center;
        width: 100%;
        height: 2.25rem; /* 9 */
        border-radius: 0.5rem;
        overflow: hidden;
        background-color: rgba(226, 232, 240, 0.5); /* slate-200 / 50% */
    }

    :global(.dark) .search-input-wrapper {
        background-color: rgba(51, 65, 85, 0.5); /* slate-700 / 50% */
    }

    .search-input-wrapper:focus-within {
        box-shadow: 0 0 0 2px rgba(25, 127, 230, 0.5); /* ring-primary/50 */
    }

    .search-icon {
        display: grid;
        place-items: center;
        height: 100%;
        width: 2.25rem;
        color: var(--text-muted);
    }

    .search-icon span {
        font-size: 20px;
    }

    .search-input {
        height: 100%;
        width: 100%;
        outline: none;
        font-size: 0.875rem;
        color: var(--text-main);
        background: transparent;
        padding-right: 0.5rem;
        border: none;
    }

    .search-input::placeholder {
        color: var(--text-muted);
    }

    .nav-list {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
        flex: 1;
    }

    .nav-item {
        display: flex;
        align-items: center;
        gap: 0.75rem; /* 3 */
        padding: 0.5rem 0.75rem;
        border-radius: 0.5rem;
        color: #475569; /* slate-600 */
        text-decoration: none;
        transition: background-color 0.15s ease, color 0.15s ease;
    }

    :global(.dark) .nav-item {
        color: #cbd5e1; /* slate-300 */
    }

    .nav-item:hover {
        background-color: rgba(226, 232, 240, 0.5);
    }

    :global(.dark) .nav-item:hover {
        background-color: rgba(51, 65, 85, 0.5);
    }

    .nav-item.active {
        background-color: rgba(25, 127, 230, 0.1); /* primary/10 */
        color: var(--primary);
    }

    :global(.dark) .nav-item.active {
        background-color: rgba(25, 127, 230, 0.2);
    }

    .nav-item .icon {
        font-size: 22px;
    }

    .nav-item.active .icon {
        font-variation-settings: 'FILL' 1;
    }

    .nav-item .label {
        font-size: 0.875rem;
        font-weight: 500;
    }

    .badge {
        margin-left: auto;
        background-color: var(--primary);
        color: #ffffff;
        font-size: 0.625rem; /* 10px */
        font-weight: 700;
        padding: 0.125rem 0.375rem;
        border-radius: 9999px;
    }

    .bottom-actions {
        margin-top: auto;
        padding-top: 1rem;
        border-top: 1px solid var(--border);
    }
</style>
