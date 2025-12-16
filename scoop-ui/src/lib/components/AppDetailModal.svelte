<script>
    import { invoke } from '@tauri-apps/api/core';
    import { open as openUrl } from '@tauri-apps/plugin-shell';

    export let app = null;
    export let isOpen = false;
    export let onClose = () => {};

    let appDetail = null;
    let loading = false;
    let error = null;

    $: if (isOpen && app) {
        loadAppDetail();
    }

    async function loadAppDetail() {
        loading = true;
        error = null;
        try {
            appDetail = await invoke('get_app_detail', { 
                appName: app.name, 
                bucket: app.bucket 
            });
        } catch (e) {
            console.error('Failed to load app detail:', e);
            error = e;
        } finally {
            loading = false;
        }
    }

    function handleClose() {
        appDetail = null;
        onClose();
    }

    function handleBackdropClick(e) {
        if (e.target === e.currentTarget) {
            handleClose();
        }
    }

    function handleKeydown(e) {
        if (e.key === 'Escape') {
            handleClose();
        }
    }

    async function openHomepage() {
        if (appDetail?.homepage) {
            await openUrl(appDetail.homepage);
        }
    }

    async function installApp() {
        // TODO: Implement install functionality
        console.log('Install:', app.name);
    }
</script>

<svelte:window on:keydown={handleKeydown} />

{#if isOpen}
    <div class="modal-backdrop" on:click={handleBackdropClick}>
        <div class="modal-content">
            <button class="close-btn" on:click={handleClose}>
                <span class="material-symbols-outlined">close</span>
            </button>

            {#if loading}
                <div class="loading">
                    <p>Loading app details...</p>
                </div>
            {:else if error}
                <div class="error">
                    <p>Failed to load app details: {error}</p>
                </div>
            {:else if appDetail}
                <!-- Header -->
                <div class="header">
                    <div class="icon-wrapper">
                        <span class="material-symbols-outlined icon">{app.icon || 'apps'}</span>
                    </div>
                    <div class="header-info">
                        <h2 class="app-name">{appDetail.name}</h2>
                        <div class="meta">
                            <span class="version">v{appDetail.version}</span>
                            <span class="separator">•</span>
                            <span class="bucket">{appDetail.bucket}</span>
                        </div>
                    </div>
                </div>

                <!-- Description -->
                {#if appDetail.description}
                    <div class="section">
                        <p class="description">{appDetail.description}</p>
                    </div>
                {/if}

                <!-- Info Grid -->
                <div class="info-grid">
                    {#if appDetail.homepage}
                        <div class="info-item">
                            <span class="material-symbols-outlined info-icon">home</span>
                            <div>
                                <div class="info-label">Homepage</div>
                                <a href={appDetail.homepage} class="info-value link" on:click|preventDefault={openHomepage}>
                                    {appDetail.homepage}
                                </a>
                            </div>
                        </div>
                    {/if}

                    {#if appDetail.license}
                        <div class="info-item">
                            <span class="material-symbols-outlined info-icon">description</span>
                            <div>
                                <div class="info-label">License</div>
                                <div class="info-value">{appDetail.license}</div>
                            </div>
                        </div>
                    {/if}
                </div>

                <!-- Dependencies -->
                {#if appDetail.depends && appDetail.depends.length > 0}
                    <div class="section">
                        <h3 class="section-title">
                            <span class="material-symbols-outlined">package_2</span>
                            Dependencies
                        </h3>
                        <div class="tags">
                            {#each appDetail.depends as dep}
                                <span class="tag">{dep}</span>
                            {/each}
                        </div>
                    </div>
                {/if}

                <!-- Binaries -->
                {#if appDetail.bin && appDetail.bin.length > 0}
                    <div class="section">
                        <h3 class="section-title">
                            <span class="material-symbols-outlined">terminal</span>
                            Executables
                        </h3>
                        <div class="tags">
                            {#each appDetail.bin as binary}
                                <span class="tag">{binary}</span>
                            {/each}
                        </div>
                    </div>
                {/if}

                <!-- Notes -->
                {#if appDetail.notes && appDetail.notes.length > 0}
                    <div class="section">
                        <h3 class="section-title">
                            <span class="material-symbols-outlined">info</span>
                            Notes
                        </h3>
                        <ul class="notes-list">
                            {#each appDetail.notes as note}
                                <li>{note}</li>
                            {/each}
                        </ul>
                    </div>
                {/if}

                <!-- Actions -->
                <div class="actions">
                    <button class="btn btn-primary" on:click={installApp}>
                        <span class="material-symbols-outlined">download</span>
                        Install
                    </button>
                    {#if appDetail.homepage}
                        <button class="btn btn-secondary" on:click={openHomepage}>
                            <span class="material-symbols-outlined">open_in_new</span>
                            Open Homepage
                        </button>
                    {/if}
                </div>
            {/if}
        </div>
    </div>
{/if}

<style>
    .modal-backdrop {
        position: fixed;
        inset: 0;
        background-color: rgba(0, 0, 0, 0.5);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 50;
        padding: 1rem;
    }

    .modal-content {
        background-color: #ffffff;
        border-radius: 1rem;
        max-width: 42rem;
        width: 100%;
        max-height: 90vh;
        overflow-y: auto;
        padding: 2rem;
        position: relative;
        box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
    }

    :global(.dark) .modal-content {
        background-color: #1e293b;
    }

    .close-btn {
        position: absolute;
        top: 1rem;
        right: 1rem;
        background: none;
        border: none;
        cursor: pointer;
        color: var(--text-muted);
        padding: 0.5rem;
        border-radius: 0.5rem;
        transition: background-color 0.15s;
    }

    .close-btn:hover {
        background-color: #f1f5f9;
    }

    :global(.dark) .close-btn:hover {
        background-color: #334155;
    }

    .loading, .error {
        text-align: center;
        padding: 3rem 2rem;
        color: var(--text-muted);
    }

    .error {
        color: #ef4444;
    }

    .header {
        display: flex;
        align-items: center;
        gap: 1rem;
        margin-bottom: 1.5rem;
    }

    .icon-wrapper {
        width: 4rem;
        height: 4rem;
        border-radius: 1rem;
        background-color: #f1f5f9;
        display: flex;
        align-items: center;
        justify-content: center;
        flex-shrink: 0;
    }

    :global(.dark) .icon-wrapper {
        background-color: #334155;
    }

    .icon {
        font-size: 2rem;
        color: #94a3b8;
    }

    .header-info {
        flex: 1;
        min-width: 0;
    }

    .app-name {
        font-size: 1.5rem;
        font-weight: 700;
        margin: 0 0 0.25rem 0;
        color: var(--text-main);
    }

    .meta {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        font-size: 0.875rem;
        color: var(--text-muted);
    }

    .version {
        font-weight: 600;
        color: var(--primary);
    }

    .separator {
        color: var(--text-muted);
    }

    .section {
        margin-bottom: 1.5rem;
    }

    .description {
        font-size: 1rem;
        line-height: 1.6;
        color: var(--text-main);
        margin: 0;
    }

    .info-grid {
        display: grid;
        gap: 1rem;
        margin-bottom: 1.5rem;
    }

    .info-item {
        display: flex;
        gap: 0.75rem;
        padding: 1rem;
        background-color: #f8fafc;
        border-radius: 0.5rem;
    }

    :global(.dark) .info-item {
        background-color: #0f172a;
    }

    .info-icon {
        font-size: 1.25rem;
        color: var(--primary);
        flex-shrink: 0;
    }

    .info-label {
        font-size: 0.75rem;
        font-weight: 600;
        text-transform: uppercase;
        color: var(--text-muted);
        margin-bottom: 0.25rem;
    }

    .info-value {
        font-size: 0.875rem;
        color: var(--text-main);
        word-break: break-all;
    }

    .link {
        color: var(--primary);
        text-decoration: none;
    }

    .link:hover {
        text-decoration: underline;
    }

    .section-title {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        font-size: 1rem;
        font-weight: 600;
        margin: 0 0 0.75rem 0;
        color: var(--text-main);
    }

    .section-title .material-symbols-outlined {
        font-size: 1.25rem;
        color: var(--primary);
    }

    .tags {
        display: flex;
        flex-wrap: wrap;
        gap: 0.5rem;
    }

    .tag {
        padding: 0.25rem 0.75rem;
        background-color: #f1f5f9;
        color: var(--text-main);
        border-radius: 9999px;
        font-size: 0.75rem;
        font-weight: 500;
    }

    :global(.dark) .tag {
        background-color: #334155;
    }

    .notes-list {
        margin: 0;
        padding-left: 1.5rem;
        color: var(--text-main);
    }

    .notes-list li {
        margin-bottom: 0.5rem;
        font-size: 0.875rem;
        line-height: 1.5;
    }

    .actions {
        display: flex;
        gap: 0.75rem;
        margin-top: 2rem;
        padding-top: 1.5rem;
        border-top: 1px solid var(--border);
    }

    .btn {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
        padding: 0.75rem 1.5rem;
        border-radius: 0.5rem;
        font-weight: 600;
        font-size: 0.875rem;
        border: none;
        cursor: pointer;
        transition: all 0.15s;
    }

    .btn-primary {
        background-color: var(--primary);
        color: #ffffff;
    }

    .btn-primary:hover {
        background-color: #2563eb;
    }

    .btn-secondary {
        background-color: #f1f5f9;
        color: var(--text-main);
    }

    :global(.dark) .btn-secondary {
        background-color: #334155;
    }

    .btn-secondary:hover {
        background-color: #e2e8f0;
    }

    :global(.dark) .btn-secondary:hover {
        background-color: #475569;
    }

    .btn .material-symbols-outlined {
        font-size: 1.125rem;
    }
</style>
