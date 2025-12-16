<script lang="ts">
    import AppCard from '$lib/components/AppCard.svelte';
    import ProgressModal from '$lib/components/ProgressModal.svelte';
    import DependencyDialog from '$lib/components/DependencyDialog.svelte';
    import AppDetailModal from '$lib/components/AppDetailModal.svelte';
    import { page } from '$app/stores';
    import { goto } from '$app/navigation';
    import { searchLocalApps, installApp, checkDependencies, isAppInstalled, type SearchResult } from '$lib/scoop';
    import { installedAppsStore } from '$lib/stores';
    import { listen } from '@tauri-apps/api/event';
    import { onMount, onDestroy } from 'svelte';

    const trending = ['Trending', 'VS Code', 'Google Chrome', 'NodeJS', '7zip', 'Python', 'Firefox'];
    
    // Mock data for search suggestions (Discovery view)
    const suggestedApps = [
        { name: 'Visual Studio Code', desc: 'Code editing. Redefined.', icon: 'code', isSymbol: true },
        { name: 'Windows Terminal', desc: 'Modern command line tools', icon: 'terminal', isSymbol: true },
        { name: '7-Zip', desc: 'High compression ratio archiver', icon: 'folder_zip', isSymbol: true },
        { name: 'VLC Media Player', desc: 'Open source multimedia player', icon: 'play_circle', isSymbol: true },
        { name: 'Discord', desc: 'Talk, chat, and hang out', icon: 'webhook', isSymbol: true },
        { name: 'Postman', desc: 'API Platform for developers', icon: 'science', isSymbol: true }
    ];

    let query = $page.url.searchParams.get('q') || '';
    let results: SearchResult[] = [];
    let searching = false;
    let searched = false;
    let searchError: string | null = null;
    
    // 安装状态
    let installingApps = new Set<string>();
    let installedAppsSet = new Set<string>();
    
    // 进度对话框
    let showProgress = false;
    let progressTitle = "安装应用";
    let progressMessage = "";
    let progressValue = 0;
    let progressStatus: 'idle' | 'running' | 'success' | 'error' = 'idle';
    let progressLogs: string[] = [];
    
    // 依赖对话框
    let showDependencyDialog = false;
    let currentInstallApp = "";
    let currentDependencies: string[] = [];
    let currentBucket = "";
    
    let unlistenProgress: (() => void) | null = null;
    
    // 详情对话框
    let showDetailModal = false;
    let selectedApp: any = null;

    // Update input when URL changes
    $: {
        const q = $page.url.searchParams.get('q') || '';
        if (q !== query) {
            query = q;
        }
        if (q) {
            performSearch(q);
        } else {
            results = [];
            searched = false;
        }
    }
    
    // 更新已安装应用集合
    $: {
        installedAppsSet = new Set($installedAppsStore.map(app => app.name));
    }

    function handleSearch(e: KeyboardEvent) {
        if (e.key === 'Enter') {
            // @ts-ignore
            const val = e.target.value.trim();
            if (val) {
                goto(`/search?q=${encodeURIComponent(val)}`);
            } else {
                goto(`/search`);
            }
        }
    }

    async function performSearch(q: string) {
        if (!q.trim()) return;
        
        searching = true;
        searched = true;
        searchError = null;
        
        try {
            results = await searchLocalApps(q);
        } catch (e) {
            console.error(e);
            searchError = "Failed to search apps.";
            results = [];
        } finally {
            searching = false;
        }
    }
    
    async function handleInstall(event: CustomEvent) {
        const appName = event.detail.name;
        const app = results.find(r => r.name === appName);
        if (!app) return;
        
        // 检查依赖
        try {
            const deps = await checkDependencies(appName, app.bucket);
            if (deps.length > 0) {
                // 显示依赖对话框
                currentInstallApp = appName;
                currentDependencies = deps;
                currentBucket = app.bucket;
                showDependencyDialog = true;
                return;
            }
        } catch (e) {
            console.error('Failed to check dependencies:', e);
        }
        
        // 直接安装
        await startInstall(appName);
    }
    
    async function startInstall(appName: string) {
        installingApps.add(appName);
        installingApps = installingApps;
        
        showProgress = true;
        progressTitle = `安装 ${appName}`;
        progressMessage = "准备安装...";
        progressValue = 0;
        progressStatus = 'running';
        progressLogs = [];
        
        try {
            await installApp(appName);
            // 成功会通过事件更新
        } catch (e) {
            progressStatus = 'error';
            progressMessage = `安装失败: ${e}`;
            installingApps.delete(appName);
            installingApps = installingApps;
        }
    }
    
    function handleDependencyInstallAll() {
        showDependencyDialog = false;
        // TODO: 实现批量安装依赖 + 主应用
        startInstall(currentInstallApp);
    }
    
    function handleDependencySkip() {
        showDependencyDialog = false;
        startInstall(currentInstallApp);
    }
    
    function handleDependencyCancel() {
        showDependencyDialog = false;
        currentInstallApp = "";
        currentDependencies = [];
    }
    
    function handleProgressClose() {
        showProgress = false;
        progressLogs = [];
    }
    
    function handleCardClick(app: SearchResult) {
        selectedApp = {
            name: app.name,
            bucket: app.bucket,
            icon: 'extension'
        };
        showDetailModal = true;
    }
    
    function handleDetailClose() {
        showDetailModal = false;
        selectedApp = null;
    }
    
    onMount(async () => {
        // 监听安装进度事件
        unlistenProgress = await listen('install-progress', (event: any) => {
            const data = event.payload;
            progressMessage = data.message;
            progressValue = data.progress;
            
            // 只在状态变化时添加日志,避免重复
            if (data.status === 'starting' || data.status === 'completed' || data.status === 'error') {
                progressLogs = [...progressLogs, data.message];
            }
            
            if (data.status === 'completed') {
                progressStatus = 'success';
                installingApps.delete(data.app_name);
                installingApps = installingApps;
                installedAppsSet.add(data.app_name);
                installedAppsSet = installedAppsSet;
                
                // 刷新已安装列表
                setTimeout(() => {
                    window.location.reload();
                }, 1500);
            } else if (data.status === 'error') {
                progressStatus = 'error';
                installingApps.delete(data.app_name);
                installingApps = installingApps;
            }
        });
    });
    
    onDestroy(() => {
        if (unlistenProgress) {
            unlistenProgress();
        }
    });
</script>

<div class="search-page">
    <!-- Heading -->
    <header class="header">
        <h1 class="title">Search</h1>
        <p class="subtitle">Find apps, utilities, and developer tools.</p>
    </header>

    <!-- Search Input -->
    <div class="search-box-wrapper">
        <div class="search-icon">
            <span class="material-symbols-outlined icon-lg">search</span>
        </div>
        <input 
            class="main-search-input" 
            placeholder="Games, Utilities, Development..." 
            type="text" 
            value={query}
            on:input={(e) => query = e.currentTarget.value}
            on:keydown={handleSearch}
            autofocus
        />
    </div>

    {#if query && searched}
        <!-- Search Results View -->
        <section class="section">
            <h2 class="section-title">
                Results for "{query}" 
                <span class="count-badge">{results.length}</span>
            </h2>

            {#if searching}
                <div class="loading-state">
                    <span class="material-symbols-outlined spin">progress_activity</span>
                    <p>Searching local manifests...</p>
                </div>
            {:else if searchError}
                 <div class="error-msg">{searchError}</div>
            {:else if results.length === 0}
                <div class="empty-state">
                    <span class="material-symbols-outlined icon-empty">search_off</span>
                    <p>No local apps found matching "{query}".</p>
                    <p class="empty-sub">Try "scoop search {query}" in terminal for remote buckets.</p>
                </div>
            {:else}
                <div class="grid-3">
                    {#each results as app}
                        <AppCard 
                            name={app.name} 
                            description={app.description || `In bucket: ${app.bucket}`} 
                            icon="extension" 
                            isIconSymbol={true}
                            installed={installedAppsSet.has(app.name)}
                            installing={installingApps.has(app.name)}
                            onClick={() => handleCardClick(app)}
                            on:install={handleInstall}
                        />
                    {/each}
                </div>
            {/if}
        </section>
    {:else}
        <!-- Discovery View (Default) -->
        <section class="section">
            <h2 class="section-title">Discover</h2>
            <div class="chips-container">
                {#each trending as chip, i}
                    <button class="chip" on:click={() => goto(`/search?q=${chip}`)}>
                        {#if i === 0}
                            <span class="material-symbols-outlined chip-icon">trending_up</span>
                        {/if}
                        {chip}
                    </button>
                {/each}
            </div>
        </section>

        <section class="section">
            <div class="section-header">
                <h2 class="section-title">Suggested Apps</h2>
                <button class="see-all">See All</button>
            </div>
            
            <div class="grid-3">
                {#each suggestedApps as app}
                    <AppCard 
                        name={app.name} 
                        description={app.desc} 
                        icon={app.icon} 
                        isIconSymbol={app.isSymbol}
                        showInstallButton={false}
                    />
                {/each}
            </div>
        </section>
    {/if}
</div>

<!-- 进度对话框 -->
<ProgressModal
    bind:show={showProgress}
    title={progressTitle}
    message={progressMessage}
    progress={progressValue}
    status={progressStatus}
    logs={progressLogs}
    on:close={handleProgressClose}
/>

<!-- 依赖对话框 -->
<DependencyDialog
    bind:show={showDependencyDialog}
    appName={currentInstallApp}
    dependencies={currentDependencies}
    on:install-all={handleDependencyInstallAll}
    on:skip={handleDependencySkip}
    on:cancel={handleDependencyCancel}
/>

<!-- 详情对话框 -->
<AppDetailModal
    app={selectedApp}
    isOpen={showDetailModal}
    onClose={handleDetailClose}
/>

<style>
    .search-page {
        max-width: 1200px;
        margin: 0 auto;
        padding: 2rem 1.5rem; /* px-6 py-8 */
        display: flex;
        flex-direction: column;
        gap: 2.5rem; /* 10 */
        height: 100%; /* Full height */
    }

    @media (min-width: 768px) {
        .search-page { padding: 3rem 3rem; }
    }

    .header {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .title {
        font-size: 2.25rem; /* 4xl */
        font-weight: 900; /* black */
        letter-spacing: -0.05em;
        margin: 0;
        color: var(--text-main);
    }
    @media (min-width: 768px) { .title { font-size: 3rem; } }

    .subtitle {
        font-size: 1.125rem; /* lg */
        color: var(--text-muted);
        margin: 0;
    }

    .search-box-wrapper {
        position: relative;
    }

    .search-icon {
        position: absolute;
        top: 0; bottom: 0; left: 0;
        padding-left: 1rem;
        display: flex;
        align-items: center;
        pointer-events: none;
    }
    @media (min-width: 768px) { .search-icon { padding-left: 1.5rem; } }

    .icon-lg { font-size: 28px; color: #94a3b8; }

    .main-search-input {
        width: 100%;
        height: 3.5rem; /* 14 */
        padding-left: 3.5rem;
        padding-right: 1.5rem;
        border-radius: 1rem; /* 2xl */
        background-color: #ffffff;
        border: 1px solid var(--border); /* ring-1 slate-200 */
        font-size: 1.125rem; /* lg */
        color: var(--text-main);
        outline: none;
        box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.02);
        transition: all 0.2s;
    }
    @media (min-width: 768px) {
        .main-search-input { height: 4rem; padding-left: 4rem; }
    }

    :global(.dark) .main-search-input {
        background-color: #1e2936; /* card-dark */
        border-color: #334155;
    }

    .main-search-input:focus {
        border-color: var(--primary);
        box-shadow: 0 0 0 1px var(--primary), 0 0 15px rgba(25, 127, 230, 0.3);
    }

    .section {
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    .section-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .section-title {
        font-size: 1.25rem; /* xl */
        font-weight: 700;
        margin: 0;
        color: var(--text-main);
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .count-badge {
        font-size: 0.875rem;
        background-color: var(--bg-surface-2);
        color: var(--text-muted);
        padding: 0.125rem 0.5rem;
        border-radius: 9999px;
    }

    .chips-container {
        display: flex;
        flex-wrap: wrap;
        gap: 0.75rem;
    }

    .chip {
        display: flex;
        align-items: center;
        padding: 0.625rem 1.25rem;
        border-radius: 9999px;
        background-color: #f1f5f9; /* slate-100 */
        color: #334155; /* slate-700 */
        font-weight: 500;
        font-size: 0.875rem; /* sm */
        border: none;
        cursor: pointer;
        transition: background-color 0.15s;
    }

    :global(.dark) .chip {
        background-color: #1e293b;
        color: #cbd5e1;
    }

    .chip:hover { background-color: #e2e8f0; }
    :global(.dark) .chip:hover { background-color: #334155; }

    .chip-icon { font-size: 18px; margin-right: 0.5rem; color: var(--text-muted); }

    .see-all {
        color: var(--primary);
        font-size: 0.875rem;
        font-weight: 600;
        background: none;
        border: none;
        cursor: pointer;
    }
    .see-all:hover { text-decoration: underline; }

    .grid-3 {
        display: grid;
        grid-template-columns: repeat(1, minmax(0, 1fr));
        gap: 1.25rem;
    }
    @media (min-width: 768px) { .grid-3 { grid-template-columns: repeat(2, minmax(0, 1fr)); } }
    @media (min-width: 1280px) { .grid-3 { grid-template-columns: repeat(3, minmax(0, 1fr)); } }

    /* Loading & Empty States */
    .loading-state, .empty-state {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding: 4rem 0;
        color: var(--text-muted);
        text-align: center;
    }

    .spin {
        font-size: 2.5rem;
        margin-bottom: 1rem;
        animation: spin 1s linear infinite;
        color: var(--primary);
    }

    .icon-empty {
        font-size: 3rem;
        margin-bottom: 1rem;
        color: var(--border);
    }
    
    .error-msg {
        color: #ef4444;
        text-align: center;
        padding: 2rem;
        background-color: rgba(239, 68, 68, 0.05);
        border-radius: 1rem;
    }
    
    .empty-sub { font-size: 0.875rem; margin-top: 0.5rem; }

    @keyframes spin {
        from { transform: rotate(0deg); }
        to { transform: rotate(360deg); }
    }
</style>
