<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    
    export let name: string;
    export let description: string;
    export let icon: string;
    export let isIconSymbol = false;
    export let onClick = () => {};
    export let installed = false;
    export let installing = false;
    export let showInstallButton = true;
    
    const dispatch = createEventDispatcher();
    
    function handleInstall(e: Event) {
        e.stopPropagation();
        dispatch('install', { name });
    }
</script>

<div class="app-card" role="button" tabindex="0" on:click={onClick} on:keydown={(e) => e.key === 'Enter' && onClick()}>
    <div class="icon-wrapper">
        {#if isIconSymbol}
            <span class="material-symbols-outlined icon-symbol">{icon}</span>
        {:else}
            <img src={icon} alt="{name} logo" class="icon-img" />
        {/if}
    </div>
    
    <div class="content">
        <h4 class="app-name">{name}</h4>
        <p class="app-desc">{description}</p>
    </div>

    {#if showInstallButton}
        {#if installed}
            <div class="installed-badge">
                <span class="material-symbols-outlined">check_circle</span>
                已安装
            </div>
        {:else if installing}
            <button class="get-btn installing" disabled>
                <span class="material-symbols-outlined spin">progress_activity</span>
                安装中
            </button>
        {:else}
            <button class="get-btn" on:click={handleInstall}>
                <span class="material-symbols-outlined">download</span>
                安装
            </button>
        {/if}
    {/if}
</div>

<style>
    .app-card {
        display: flex;
        align-items: center;
        gap: 0.75rem; /* 3 */
        padding: 0.75rem;
        border-radius: 0.75rem; /* xl */
        background-color: #ffffff;
        border: 1px solid var(--border);
        transition: background-color 0.15s, border-color 0.15s;
        cursor: pointer;
    }

    :global(.dark) .app-card {
        background-color: var(--bg-subtle);
    }

    .app-card:hover {
        background-color: #f8fafc; /* slate-50 */
    }

    :global(.dark) .app-card:hover {
        background-color: rgba(30, 41, 59, 0.8); /* slate-800/80 */
    }

    .icon-wrapper {
        width: 3.5rem; /* 14 */
        height: 3.5rem;
        flex-shrink: 0;
        border-radius: 0.75rem;
        background-color: #f1f5f9; /* slate-100 */
        display: flex;
        align-items: center;
        justify-content: center;
        overflow: hidden;
    }

    :global(.dark) .icon-wrapper {
        background-color: #1e293b; /* slate-800 */
    }

    .icon-img {
        width: 2rem; /* 8 */
        height: 2rem;
        object-fit: contain;
    }

    .icon-symbol {
        font-size: 1.875rem; /* 30px */
        color: #94a3b8; /* slate-400 */
    }

    .content {
        flex: 1;
        min-width: 0;
        display: flex;
        flex-direction: column;
        gap: 0.125rem;
    }

    .app-name {
        font-size: 0.875rem; /* sm */
        font-weight: 600;
        color: var(--text-main);
        margin: 0;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .app-desc {
        font-size: 0.75rem; /* xs */
        color: var(--text-muted);
        margin: 0;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .get-btn {
        display: flex;
        align-items: center;
        gap: 0.25rem;
        background-color: #f1f5f9; /* slate-100 */
        color: var(--primary);
        font-weight: 700;
        font-size: 0.75rem; /* xs */
        padding: 0.375rem 1rem;
        border-radius: 9999px;
        border: none;
        cursor: pointer;
        transition: background-color 0.15s, color 0.15s;
    }

    :global(.dark) .get-btn {
        background-color: #334155; /* slate-700 */
    }

    .get-btn:hover:not(:disabled) {
        background-color: var(--primary);
        color: #ffffff;
    }
    
    .get-btn:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }
    
    .get-btn.installing {
        background-color: rgba(25, 127, 230, 0.1);
    }
    
    .get-btn .material-symbols-outlined {
        font-size: 16px;
    }
    
    .installed-badge {
        display: flex;
        align-items: center;
        gap: 0.25rem;
        color: #10b981;
        font-weight: 600;
        font-size: 0.75rem;
        padding: 0.375rem 0.75rem;
    }
    
    .installed-badge .material-symbols-outlined {
        font-size: 16px;
    }
    
    @keyframes spin {
        from { transform: rotate(0deg); }
        to { transform: rotate(360deg); }
    }
    
    .spin {
        animation: spin 1s linear infinite;
    }
</style>
