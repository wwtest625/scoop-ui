<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { get } from 'svelte/store';
    import { getInstalledApps, updateApp, getAppSizes, uninstallApp, checkUpdatesAsync, type ScoopApp } from '$lib/scoop';
    import { installedAppsStore, updatingAppsStore } from '$lib/stores';
    import ConfirmDialog from '$lib/components/ConfirmDialog.svelte';
    import ProgressModal from '$lib/components/ProgressModal.svelte';
    import { listen } from '@tauri-apps/api/event';

    let installedApps: ScoopApp[] = [];
    let loading = true;
    let error: string | null = null;
    let updatingApps: Set<string> = new Set(); // Track which apps are being updated
    let loadingSizes = false; // Track if sizes are being loaded
    
    // 卸载相关状态
    let showUninstallConfirm = false;
    let uninstallAppName = "";
    let uninstallingApps: Set<string> = new Set();
    
    // 进度对话框
    let showProgress = false;
    let progressTitle = "";
    let progressMessage = "";
    let progressValue = 0;
    let progressStatus: 'idle' | 'running' | 'success' | 'error' = 'idle';
    let progressLogs: string[] = [];
    
    let unlistenUninstall: (() => void) | null = null;
    let unlistenUpdate: (() => void) | null = null;
    let unlistenUpdateAll: (() => void) | null = null;
    let unlistenUpdatesChecked: (() => void) | null = null;
    
    // 更新检查状态
    let checkingUpdates = false;
    
    // Subscribe to store
    $: installedApps = $installedAppsStore;
    $: if ($installedAppsStore.length > 0) {
        loading = false;
    }

    // Fallback icon since `scoop export` doesn't provide icons
    const DEFAULT_ICON = 'terminal';
    
    // Refresh function now updates the store
    async function refreshApps() {
        loading = true;
        try {
            const apps = await getInstalledApps();
            installedAppsStore.set(apps);
            // Update cache
            localStorage.setItem('scoop_installed_apps', JSON.stringify(apps));
            
            // Start async update check
            checkingUpdates = true;
            checkUpdatesAsync().catch(err => {
                console.warn('Failed to check updates:', err);
                checkingUpdates = false;
            });
        } catch (e) {
            error = e as string;
        } finally {
            loading = false;
        }
    }

    onMount(async () => {
        // If store is empty, try refresh (maybe first load failed or came here directly)
        if ($installedAppsStore.length === 0) {
             await refreshApps();
        } else {
             loading = false;
        }
        
        // 从全局 store 恢复更新状态
        const currentUpdating = get(updatingAppsStore);
        currentUpdating.forEach((status, appName) => {
            if (status.status === 'updating') {
                updatingApps.add(appName);
            }
        });
        updatingApps = updatingApps;
        
        // 监听卸载进度事件
        unlistenUninstall = await listen('uninstall-progress', (event: any) => {
            const data = event.payload;
            progressMessage = data.message;
            progressValue = data.progress;
            
            // 只在状态变化时添加日志
            if (data.status === 'starting' || data.status === 'completed' || data.status === 'error') {
                progressLogs = [...progressLogs, data.message];
            }
            
            if (data.status === 'completed') {
                progressStatus = 'success';
                uninstallingApps.delete(data.app_name);
                uninstallingApps = uninstallingApps;
                
                // 刷新已安装列表
                setTimeout(async () => {
                    await refreshApps();
                    showProgress = false;
                    progressLogs = [];
                }, 1500);
            } else if (data.status === 'error') {
                progressStatus = 'error';
                uninstallingApps.delete(data.app_name);
                uninstallingApps = uninstallingApps;
            }
        });
        
        // 监听单个应用更新进度事件
        unlistenUpdate = await listen('update-progress', (event: any) => {
            const data = event.payload;
            progressMessage = data.message;
            progressValue = data.progress;
            
            // 只在状态变化时添加日志,避免重复
            if (data.status === 'starting' || data.status === 'completed' || data.status === 'error') {
                progressLogs = [...progressLogs, data.message];
            }
            
            if (data.status === 'completed') {
                progressStatus = 'success';
                updatingApps.delete(data.app_name);
                updatingApps = updatingApps;
                
                // 更新全局 store
                updatingAppsStore.update(store => {
                    store.delete(data.app_name);
                    return store;
                });
                
                // 刷新已安装列表
                setTimeout(async () => {
                    await refreshApps();
                    showProgress = false;
                    progressLogs = [];
                }, 1500);
            } else if (data.status === 'error') {
                progressStatus = 'error';
                updatingApps.delete(data.app_name);
                updatingApps = updatingApps;
                
                // 更新全局 store
                updatingAppsStore.update(store => {
                    store.set(data.app_name, { appName: data.app_name, status: 'error', message: data.message });
                    return store;
                });
            } else if (data.status === 'updating' || data.status === 'starting') {
                // 更新全局 store
                updatingAppsStore.update(store => {
                    store.set(data.app_name, { appName: data.app_name, status: 'updating' });
                    return store;
                });
            }
        });
        
        // 监听批量更新进度事件
        unlistenUpdateAll = await listen('update-all-progress', (event: any) => {
            const data = event.payload;
            progressMessage = data.message;
            progressValue = data.progress;
            
            // 只在状态变化时添加日志
            if (data.status === 'starting' || data.status === 'completed' || data.status === 'error') {
                progressLogs = [...progressLogs, data.message];
            }
            
            if (data.status === 'completed') {
                progressStatus = 'success';
                
                // 刷新已安装列表
                setTimeout(async () => {
                    await refreshApps();
                    showProgress = false;
                    progressLogs = [];
                }, 1500);
            } else if (data.status === 'error') {
                progressStatus = 'error';
            }
        });
        
        // 监听异步更新检查完成事件
        unlistenUpdatesChecked = await listen('updates-checked', (event: any) => {
            const data = event.payload;
            const updatableApps = new Set(data.updatable_apps);
            
            console.log('Updates checked, found:', data.updatable_apps.length, 'updatable apps');
            
            // 更新应用的 has_update 状态
            installedApps = installedApps.map(app => ({
                ...app,
                has_update: updatableApps.has(app.name)
            }));
            
            // 更新 store
            installedAppsStore.set(installedApps);
            
            // 更新缓存
            localStorage.setItem('scoop_installed_apps', JSON.stringify(installedApps));
            
            checkingUpdates = false;
        });
    });
    
    onDestroy(() => {
        if (unlistenUninstall) {
            unlistenUninstall();
        }
        if (unlistenUpdate) {
            unlistenUpdate();
        }
        if (unlistenUpdateAll) {
            unlistenUpdateAll();
        }
        if (unlistenUpdatesChecked) {
            unlistenUpdatesChecked();
        }
    });
    
    // Load app sizes on demand
    async function loadAppSizes() {
        if (loadingSizes || installedApps.length === 0) return;
        
        // Check if sizes already loaded
        if (installedApps.some(app => app.install_size > 0)) return;
        
        loadingSizes = true;
        try {
            const appNames = installedApps.map(app => app.name);
            const sizes = await getAppSizes(appNames);
            
            // Update app sizes
            installedApps = installedApps.map(app => ({
                ...app,
                install_size: sizes[app.name] || 0
            }));
        } catch (e) {
            console.error('Failed to load app sizes:', e);
        } finally {
            loadingSizes = false;
        }
    }
    
    // Watch for filter changes to load sizes when needed
    $: if (activeFilter === '体积最大') {
        loadAppSizes();
    }

    const filters = ['全部应用', '可更新', '最近安装', '体积最大'];
    let activeFilter = '全部应用';
    
    // Derived state for filtering
    $: displayedApps = (() => {
        let filtered = [...installedApps];
        
        switch (activeFilter) {
            case '可更新':
                filtered = filtered.filter(app => app.has_update);
                break;
            case '最近安装':
                // Sort by updated timestamp (most recent first)
                filtered.sort((a, b) => b.updated - a.updated);
                break;
            case '体积最大':
                // Sort by install size (largest first)
                filtered.sort((a, b) => b.install_size - a.install_size);
                break;
            default:
                // '全部应用' - no filtering, keep original order
                break;
        }
        
        return filtered;
    })();
    
    // Count of updatable apps for badge
    $: updatableCount = installedApps.filter(app => app.has_update).length;
    
    // Helper function to format bytes to human readable size
    function formatSize(bytes: number): string {
        if (bytes === 0) return '0 B';
        const k = 1024;
        const sizes = ['B', 'KB', 'MB', 'GB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
    }
    
    // Helper function to format timestamp to relative time
    function formatRelativeTime(timestamp: number): string {
        if (timestamp === 0) return '未知';
        const now = Date.now();
        const diff = now - timestamp;
        const days = Math.floor(diff / (1000 * 60 * 60 * 24));
        
        if (days === 0) return '今天';
        if (days === 1) return '昨天';
        if (days < 7) return `${days} 天前`;
        if (days < 30) return `${Math.floor(days / 7)} 周前`;
        if (days < 365) return `${Math.floor(days / 30)} 个月前`;
        return `${Math.floor(days / 365)} 年前`;
    }
    
    // Handle app update
    async function handleUpdateApp(appName: string) {
        updatingApps.add(appName);
        updatingApps = updatingApps;
        
        // 保存到全局 store
        updatingAppsStore.update(store => {
            store.set(appName, { appName, status: 'updating' });
            return store;
        });
        
        showProgress = true;
        progressTitle = `更新 ${appName}`;
        progressMessage = "准备更新...";
        progressValue = 0;
        progressStatus = 'running';
        progressLogs = [];
        
        try {
            await updateApp(appName);
            // 成功会通过事件更新
        } catch (e) {
            progressStatus = 'error';
            progressMessage = `更新失败: ${e}`;
            updatingApps.delete(appName);
            updatingApps = updatingApps;
            
            // 更新全局 store
            updatingAppsStore.update(store => {
                store.delete(appName);
                return store;
            });
        }
    }
    
    // Handle batch update all apps
    async function handleUpdateAllApps() {
        if (updatableCount === 0) {
            alert("没有可更新的应用");
            return;
        }
        
        showProgress = true;
        progressTitle = "批量更新";
        progressMessage = "准备更新所有应用...";
        progressValue = 0;
        progressStatus = 'running';
        progressLogs = [];
        
        try {
            await updateAllApps();
            // 成功会通过事件更新
        } catch (e) {
            progressStatus = 'error';
            progressMessage = `批量更新失败: ${e}`;
        }
    }
    
    // 显示卸载确认对话框
    function showUninstallDialog(appName: string) {
        uninstallAppName = appName;
        showUninstallConfirm = true;
    }
    
    // 确认卸载
    async function handleConfirmUninstall() {
        showUninstallConfirm = false;
        
        uninstallingApps.add(uninstallAppName);
        uninstallingApps = uninstallingApps;
        
        showProgress = true;
        progressTitle = `卸载 ${uninstallAppName}`;
        progressMessage = "准备卸载...";
        progressValue = 0;
        progressStatus = 'running';
        progressLogs = [];
        
        try {
            await uninstallApp(uninstallAppName);
            // 成功会通过事件更新
        } catch (e) {
            progressStatus = 'error';
            progressMessage = `卸载失败: ${e}`;
            uninstallingApps.delete(uninstallAppName);
            uninstallingApps = uninstallingApps;
        }
    }
    
    function handleCancelUninstall() {
        showUninstallConfirm = false;
        uninstallAppName = "";
    }
    
    function handleProgressClose() {
        showProgress = false;
        progressLogs = [];
    }
</script>

<div class="installed-page">
    <header class="page-header">
        <div class="header-top">
            <div class="header-titles">
                <div style="display: flex; align-items: center; gap: 1rem;">
                    <h1 class="page-title">已安装</h1>
                    <button 
                        class="btn-refresh" 
                        title="刷新列表"
                        disabled={loading}
                        on:click={refreshApps}
                    >
                        <span class="material-symbols-outlined" class:spinning={loading}>refresh</span>
                    </button>
                    {#if checkingUpdates}
                        <div class="checking-updates-hint">
                            <span class="material-symbols-outlined spin-slow">sync</span>
                            <span>正在检查更新...</span>
                        </div>
                    {/if}
                    {#if updatableCount > 0}
                        <button 
                            class="btn-update-all" 
                            title="更新所有应用"
                            on:click={handleUpdateAllApps}
                        >
                            <span class="material-symbols-outlined">upgrade</span>
                            <span>全部更新 ({updatableCount})</span>
                        </button>
                    {/if}
                </div>
                <p class="page-subtitle">本地库共有 {installedApps.length} 个应用</p>
            </div>
            
            <div class="search-container">
                <div class="search-wrapper">
                    <span class="material-symbols-outlined search-icon">search</span>
                    <input class="search-input" placeholder="搜索应用名称 (如 git, nodejs)..." type="text"/>
                    <div class="shortcut-hint"><kbd>⌘K</kbd></div>
                </div>
            </div>
        </div>

        <div class="filter-chips">
            {#each filters as filter}
                <button 
                    class="filter-chip" 
                    class:active={activeFilter === filter}
                    on:click={() => activeFilter = filter}
                >
                    <span>{filter}</span>
                    {#if filter === '可更新' && updatableCount > 0}
                        <span class="badge">{updatableCount}</span>
                    {/if}
                </button>
            {/each}
        </div>
    </header>

    <div class="content-area">
        <div class="list-container">
            <!-- Table Header -->
            <div class="list-header">
                <div class="col-name">应用名称</div>
                <div class="col-version">版本</div>
                <div class="col-action">操作</div>
            </div>

            <!-- List Items -->
            {#if loading}
                <div class="p-8 text-center text-gray-500">Loading apps...</div>
            {:else if error}
                <div class="p-8 text-center text-red-500">
                    <p>Error loading apps:</p>
                    <pre class="text-xs mt-2 text-left bg-gray-100 p-2 rounded overflow-auto">{error}</pre>
                </div>
            {:else if displayedApps.length === 0}
                 <div class="p-8 text-center text-gray-500">No apps found.</div>
            {:else}
                {#each displayedApps as app}
                    <div class="list-item">
                        <!-- Name & Icon -->
                        <div class="col-name flex-row">
                            <div class="app-icon-wrapper">
                                <!-- Placeholder icon since scoop export doesn't provide it -->
                                <span class="material-symbols-outlined app-icon-placeholder">terminal</span>
                                {#if app.has_update}
                                    <div class="update-dot"></div>
                                {/if}
                            </div>
                            <div class="app-info">
                                <h3 class="app-name">{app.name}</h3>
                                <p class="app-desc">
                                    {#if activeFilter === '最近安装'}
                                        安装于 {formatRelativeTime(app.updated)}
                                    {:else if activeFilter === '体积最大'}
                                        大小: {formatSize(app.install_size)}
                                    {:else}
                                        {app.description || 'No description'}
                                    {/if}
                                </p>
                            </div>
                        </div>

                        <!-- Version -->
                        <div class="col-version flex-col-center">
                            <div class="version-row">
                                <span class="ver-current">{app.version}</span>
                                {#if app.has_update}
                                    <span class="update-badge">可更新</span>
                                {/if}
                            </div>
                            <span class="bucket-name">{app.bucket}</span>
                        </div>

                        <!-- Actions -->
                        <div class="col-action flex-end">
                            {#if app.has_update}
                                <button 
                                    class="btn-update" 
                                    title="更新应用"
                                    disabled={updatingApps.has(app.name)}
                                    on:click={() => handleUpdateApp(app.name)}
                                >
                                    {#if updatingApps.has(app.name)}
                                        <span class="material-symbols-outlined spinning">progress_activity</span>
                                    {:else}
                                        <span class="material-symbols-outlined">upgrade</span>
                                    {/if}
                                    <span>更新</span>
                                </button>
                            {/if}
                            <button 
                                class="btn-icon-only" 
                                title="卸载"
                                disabled={uninstallingApps.has(app.name)}
                                on:click={() => showUninstallDialog(app.name)}
                            >
                                {#if uninstallingApps.has(app.name)}
                                    <span class="material-symbols-outlined spinning">progress_activity</span>
                                {:else}
                                    <span class="material-symbols-outlined">delete</span>
                                {/if}
                            </button>
                        </div>
                    </div>
                {/each}
            {/if}
        </div>

        <div class="pagination">
            <p>已显示 {displayedApps.length} 个应用</p>
        </div>
    </div>
</div>

<!-- 卸载确认对话框 -->
<ConfirmDialog
    bind:show={showUninstallConfirm}
    title="确认卸载"
    message={`确定要卸载 ${uninstallAppName} 吗?此操作无法撤销。`}
    confirmText="卸载"
    cancelText="取消"
    type="danger"
    on:confirm={handleConfirmUninstall}
    on:cancel={handleCancelUninstall}
/>

<!-- 卸载进度对话框 -->
<ProgressModal
    bind:show={showProgress}
    title={progressTitle}
    message={progressMessage}
    progress={progressValue}
    status={progressStatus}
    logs={progressLogs}
    on:close={handleProgressClose}
/>

<style>
    .installed-page {
        display: flex;
        flex-direction: column;
        height: 100%;
        position: relative;
    }

    .page-header {
        padding: 2rem 2rem 1rem 2rem;
        background-color: var(--bg-light);
        flex-shrink: 0;
        z-index: 10;
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    .header-top {
        display: flex;
        justify-content: space-between;
        align-items: flex-end;
        gap: 1rem;
        flex-wrap: wrap;
    }

    .header-titles {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
    }

    .page-title {
        font-size: 1.875rem; /* 3xl */
        font-weight: 900;
        margin: 0;
        color: var(--text-main);
    }

    .page-subtitle {
        color: var(--text-muted);
        font-weight: 500;
        margin: 0;
    }

    .search-container {
        width: 100%;
        max-width: 24rem; /* max-w-sm */
    }

    .search-wrapper {
        position: relative;
        display: flex;
        align-items: center;
    }

    .search-input {
        width: 100%;
        padding: 0.625rem 0.75rem 0.625rem 2.5rem;
        border-radius: 0.75rem; /* xl */
        border: none;
        background-color: #ffffff;
        font-size: 0.875rem;
        font-weight: 500;
        outline: none;
        box-shadow: 0 1px 2px 0 rgba(0,0,0,0.05);
    }

    :global(.dark) .search-input {
        background-color: var(--surface-dark);
        color: #fff;
    }

    .search-icon {
        position: absolute;
        left: 0.75rem;
        color: #94a3b8;
        pointer-events: none;
    }

    .shortcut-hint {
        position: absolute;
        right: 0.5rem;
    }

    .shortcut-hint kbd {
        border: 1px solid var(--border);
        border-radius: 0.25rem;
        padding: 0.125rem 0.375rem;
        font-size: 0.625rem;
        font-weight: 700;
        color: #94a3b8;
    }

    .filter-chips {
        display: flex;
        gap: 0.5rem;
        overflow-x: auto;
        padding-bottom: 0.25rem;
    }

    .filter-chip {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.375rem 1rem;
        border-radius: 9999px;
        font-size: 0.875rem;
        font-weight: 500;
        cursor: pointer;
        border: 1px solid var(--border);
        background-color: #ffffff;
        color: #475569;
        transition: all 0.1s;
    }

    :global(.dark) .filter-chip {
        background-color: var(--surface-dark);
        color: #cbd5e1;
    }

    .filter-chip.active {
        background-color: #0f172a;
        color: #ffffff;
        border-color: #0f172a;
    }

    :global(.dark) .filter-chip.active {
        background-color: #ffffff;
        color: #0f172a;
        border-color: #ffffff;
    }

    .badge {
        background-color: rgba(25, 127, 230, 0.1);
        color: var(--primary);
        padding: 0 0.375rem;
        border-radius: 0.25rem;
        font-size: 0.75rem;
        font-weight: 700;
    }

    .content-area {
        flex: 1;
        overflow-y: auto;
        padding: 0 2rem 2rem 2rem;
    }

    .list-container {
        background-color: #ffffff;
        border-radius: 1rem;
        border: 1px solid var(--border);
        overflow: hidden;
        box-shadow: 0 1px 2px 0 rgba(0,0,0,0.05);
        max-width: 64rem; /* 5xl */
        margin: 0 auto;
    }

    :global(.dark) .list-container {
        background-color: var(--bg-subtle);
    }

    /* Grid Layout for List */
    .list-header, .list-item {
        display: grid;
        grid-template-columns: 5fr 3fr 4fr;
        gap: 1rem;
        align-items: center;
        padding: 0 1.5rem;
    }

    .list-header {
        padding-top: 0.75rem;
        padding-bottom: 0.75rem;
        background-color: #f8fafc;
        border-bottom: 1px solid var(--border);
        font-size: 0.75rem;
        font-weight: 600;
        color: #64748b;
        text-transform: uppercase;
        letter-spacing: 0.05em;
    }

    :global(.dark) .list-header {
        background-color: rgba(30, 41, 59, 0.3);
    }

    .list-item {
        padding-top: 1rem;
        padding-bottom: 1rem;
        border-bottom: 1px solid var(--border);
        transition: background-color 0.1s;
    }

    .list-item:hover {
        background-color: #f8fafc;
    }

    :global(.dark) .list-item:hover {
        background-color: rgba(30, 41, 59, 0.4);
    }

    .flex-row { display: flex; align-items: center; gap: 1rem; }
    .flex-col-center { display: flex; flex-direction: column; justify-content: center; }
    .flex-end { display: flex; align-items: center; justify-content: flex-end; gap: 0.5rem; }

    .app-icon-wrapper {
        position: relative;
        width: 3rem; height: 3rem;
        flex-shrink: 0;
    }

    .app-icon {
        width: 100%; height: 100%;
        border-radius: 0.75rem;
        object-fit: cover;
        border: 1px solid var(--border);
    }

    .update-dot {
        position: absolute;
        top: -0.125rem; right: -0.125rem;
        width: 0.75rem; height: 0.75rem;
        background-color: #ef4444;
        border-radius: 9999px;
        border: 2px solid #ffffff;
    }
    :global(.dark) .update-dot { border-color: var(--surface-dark); }

    .app-info {
        min-width: 0;
        flex: 1;
    }

    .app-name {
        font-size: 1rem;
        font-weight: 700;
        margin: 0;
        color: var(--text-main);
        white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
    }

    .app-desc {
        font-size: 0.75rem;
        color: var(--text-muted);
        margin: 0;
        white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
    }

    .version-row {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .ver-current { font-size: 0.875rem; font-weight: 500; color: #334155; }
    :global(.dark) .ver-current { color: #cbd5e1; }

    .ver-old { font-size: 0.875rem; font-weight: 500; color: var(--text-muted); text-decoration: line-through; }
    .ver-arrow { font-size: 0.75rem; color: var(--text-muted); }
    .ver-new { font-size: 0.875rem; font-weight: 700; color: var(--primary); }

    .bucket-name {
        font-size: 0.625rem;
        color: var(--text-muted);
    }

    .btn-primary {
        background-color: var(--primary);
        color: white;
        border: none;
        border-radius: 9999px;
        height: 2.25rem;
        padding: 0 1.25rem;
        font-size: 0.875rem;
        font-weight: 600;
        cursor: pointer;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        transition: opacity 0.1s;
    }
    .btn-primary:active { transform: scale(0.95); }

    .btn-secondary {
        background-color: #f1f5f9;
        color: #0f172a;
        border: none;
        border-radius: 9999px;
        height: 2.25rem;
        padding: 0 1.25rem;
        font-size: 0.875rem;
        font-weight: 600;
        cursor: pointer;
    }
    :global(.dark) .btn-secondary {
        background-color: #1e293b;
        color: #e2e8f0;
    }
    .btn-secondary:hover { background-color: #e2e8f0; }

    .btn-icon-only {
        background: transparent;
        border: none;
        width: 2.25rem; height: 2.25rem;
        border-radius: 9999px;
        display: flex; align-items: center; justify-content: center;
        color: #94a3b8;
        cursor: pointer;
    }
    .btn-icon-only:hover { background-color: rgba(239, 68, 68, 0.1); color: #ef4444; }

    .btn-icon { font-size: 18px; }

    .pagination {
        margin-top: 1.5rem;
        display: flex;
        justify-content: center;
        color: var(--text-muted);
        font-size: 0.875rem;
    }

    .app-icon-placeholder {
        width: 100%; height: 100%;
        border-radius: 0.75rem;
        background-color: #f1f5f9;
        display: flex;
        align-items: center;
        justify-content: center;
        color: #94a3b8;
        font-size: 24px;
        border: 1px solid var(--border);
    }
    :global(.dark) .app-icon-placeholder {
        background-color: var(--surface-dark);
        color: #64748b;
    }
    
    .update-badge {
        display: inline-block;
        padding: 0.125rem 0.5rem;
        border-radius: 0.25rem;
        font-size: 0.625rem;
        font-weight: 700;
        background-color: rgba(239, 68, 68, 0.1);
        color: #ef4444;
        margin-left: 0.5rem;
    }
    
    .btn-update {
        display: flex;
        align-items: center;
        gap: 0.375rem;
        padding: 0.375rem 0.875rem;
        border-radius: 0.5rem;
        font-size: 0.875rem;
        font-weight: 600;
        cursor: pointer;
        border: none;
        background-color: #197fe6;
        color: white;
        transition: all 0.2s;
    }
    
    .btn-update:hover:not(:disabled) {
        background-color: #1565c0;
        transform: translateY(-1px);
    }
    
    .btn-update:active:not(:disabled) {
        transform: translateY(0);
    }
    
    .btn-update:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }
    
    .btn-update .material-symbols-outlined {
        font-size: 18px;
    }
    
    @keyframes spin {
        from {
            transform: rotate(0deg);
        }
        to {
            transform: rotate(360deg);
        }
    }
    
    .spinning {
        animation: spin 1s linear infinite;
    }
    
    .btn-refresh {
        background: var(--primary);
        border: none;
        width: 2.5rem;
        height: 2.5rem;
        border-radius: 0.5rem;
        display: flex;
        align-items: center;
        justify-content: center;
        cursor: pointer;
        color: white;
        transition: all 0.2s;
    }
    
    .btn-refresh:hover:not(:disabled) {
        opacity: 0.9;
    }
    
    :global(.dark) .btn-refresh:hover:not(:disabled) {
        opacity: 0.9;
    }
    
    .btn-refresh:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }
    
    .btn-refresh .material-symbols-outlined {
        font-size: 24px;
    }
    
    .btn-update-all {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.5rem 1rem;
        border-radius: 0.5rem;
        border: none;
        background-color: var(--primary);
        color: white;
        font-weight: 600;
        font-size: 0.875rem;
        cursor: pointer;
        transition: opacity 0.2s;
    }
    
    .btn-update-all:hover {
        opacity: 0.9;
    }
    
    .btn-update-all .material-symbols-outlined {
        font-size: 20px;
    }
</style>