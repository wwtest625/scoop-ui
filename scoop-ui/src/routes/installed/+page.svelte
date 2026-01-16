<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { get } from 'svelte/store';
    import { getInstalledApps, updateApp, updateAllApps, getAppSizes, uninstallApp, checkUpdatesAsync, type ScoopApp } from '$lib/scoop';
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

    // --- Aesthetic Helpers ---

    function getAppInitials(name: string): string {
        if (!name) return '?';
        return name.substring(0, 2).toUpperCase();
    }

    function getAppColor(name: string): string {
        if (!name) return '#64748b';
        let hash = 0;
        for (let i = 0; i < name.length; i++) {
            hash = name.charCodeAt(i) + ((hash << 5) - hash);
        }
        // Generate HSL color with good saturation and lightness for "premium" look
        const h = Math.abs(hash) % 360;
        return `hsl(${h}, 70%, 45%)`; // 45% lightness for readable white text
    }

    function getAppBackground(name: string): string {
        if (!name) return '#f1f5f9';
        let hash = 0;
        for (let i = 0; i < name.length; i++) {
            hash = name.charCodeAt(i) + ((hash << 5) - hash);
        }
        const h = Math.abs(hash) % 360;
        return `hsl(${h}, 80%, 96%)`; // Very light background for the row/avatar
    }

    // -------------------------
    
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
        <div class="header-content">
            <div class="header-main">
                <div class="title-section">
                    <h1 class="page-title">
                        已安装
                        <span class="app-count">{installedApps.length}</span>
                    </h1>
                     <button 
                        class="btn-icon" 
                        title="刷新列表"
                        disabled={loading}
                        on:click={refreshApps}
                    >
                        <span class="material-symbols-outlined" class:spinning={loading}>refresh</span>
                    </button>
                    {#if checkingUpdates}
                        <div class="status-badge checking">
                            <span class="material-symbols-outlined spin-slow">sync</span>
                            <span>检查更新...</span>
                        </div>
                    {/if}
                </div>

                <div class="actions-section">
                    {#if updatableCount > 0}
                        <button 
                            class="btn-primary" 
                            on:click={handleUpdateAllApps}
                        >
                            <span class="material-symbols-outlined">upgrade</span>
                            <span>全部更新 ({updatableCount})</span>
                        </button>
                    {/if}
                    
                    <div class="search-wrapper">
                        <span class="material-symbols-outlined search-icon">search</span>
                        <input class="search-input" placeholder="搜索应用..." type="text"/>
                        <div class="shortcut-hint"><kbd>⌘K</kbd></div>
                    </div>
                </div>
            </div>

            <div class="filter-bar">
                {#each filters as filter}
                    <button 
                        class="filter-chip" 
                        class:active={activeFilter === filter}
                        on:click={() => activeFilter = filter}
                    >
                        <span>{filter}</span>
                        {#if filter === '可更新' && updatableCount > 0}
                            <span class="badge-dot"></span>
                        {/if}
                    </button>
                {/each}
            </div>
        </div>
    </header>

    <div class="content-area">
        <div class="list-container">
            <!-- Table Header -->
            <div class="list-header">
                <div class="col-main">应用名称</div>
                <div class="col-meta">版本 & Bucket</div>
                <div class="col-action">操作</div>
            </div>

            <!-- List Items -->
            {#if loading}
                <div class="state-empty">
                    <div class="spinner"></div>
                    <p>加载中...</p>
                </div>
            {:else if error}
                <div class="state-error">
                    <span class="material-symbols-outlined icon">error</span>
                    <p>加载失败</p>
                    <pre>{error}</pre>
                </div>
            {:else if displayedApps.length === 0}
                 <div class="state-empty">
                    <span class="material-symbols-outlined icon">inbox</span>
                    <p>没有找到应用</p>
                 </div>
            {:else}
                <div class="app-list">
                    {#each displayedApps as app}
                        <!-- svelte-ignore a11y-no-static-element-interactions -->
                        <!-- svelte-ignore a11y-click-events-have-key-events -->
                        <div class="list-item" class:updating={updatingApps.has(app.name)}>
                            <!-- Main Info -->
                            <div class="col-main">
                                <div class="app-avatar" style="background-color: {getAppColor(app.name)}">
                                    <span>{getAppInitials(app.name)}</span>
                                    {#if app.has_update}
                                        <div class="avatar-badge"></div>
                                    {/if}
                                </div>
                                <div class="app-info">
                                    <h3 class="app-name">{app.name}</h3>
                                    <p class="app-desc" title={app.description}>
                                        {#if activeFilter === '最近安装'}
                                            <span class="highlight">安装于 {formatRelativeTime(app.updated)}</span>
                                            <span class="sep">•</span>
                                        {:else if activeFilter === '体积最大'}
                                            <span class="highlight">大小: {formatSize(app.install_size)}</span>
                                            <span class="sep">•</span>
                                        {/if}
                                        {app.description || '暂无描述'}
                                    </p>
                                </div>
                            </div>

                            <!-- Meta -->
                            <div class="col-meta">
                                <div class="version-tag">
                                    <span class="ver-num">{app.version}</span>
                                </div>
                                <span class="bucket-tag">
                                    <span class="material-symbols-outlined">inventory_2</span>
                                    {app.bucket}
                                </span>
                            </div>

                            <!-- Actions -->
                            <div class="col-action">
                                {#if app.has_update}
                                    <button 
                                        class="btn-action update" 
                                        title="更新应用"
                                        disabled={updatingApps.has(app.name)}
                                        on:click={() => handleUpdateApp(app.name)}
                                    >
                                        {#if updatingApps.has(app.name)}
                                            <span class="material-symbols-outlined spinning">progress_activity</span>
                                        {:else}
                                            <span class="material-symbols-outlined">upgrade</span>
                                        {/if}
                                    </button>
                                {/if}
                                <button 
                                    class="btn-action delete" 
                                    title="卸载应用"
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
                </div>
            {/if}
        </div>
        
         <div class="list-footer">
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
    :root {
        --active-color: #2563eb;
        --active-bg: #eff6ff;
        --danger-color: #ef4444;
        --text-primary: #1e293b;
        --text-secondary: #64748b;
        --border-color: #e2e8f0;
    }

    .installed-page {
        display: flex;
        flex-direction: column;
        height: 100%;
        background-color: #f8fafc; /* Lighter background */
    }

    /* Header Styles */
    .page-header {
        background-color: #ffffff;
        border-bottom: 1px solid var(--border-color);
        padding: 1.5rem 2rem 0 2rem;
        flex-shrink: 0;
        z-index: 10;
    }

    .header-content {
        max-width: 1200px;
        margin: 0 auto;
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    .header-main {
        display: flex;
        justify-content: space-between;
        align-items: center;
        flex-wrap: wrap;
        gap: 1rem;
    }

    .title-section {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    .page-title {
        font-size: 1.5rem;
        font-weight: 800;
        color: var(--text-primary);
        margin: 0;
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }
    
    .app-count {
        font-size: 0.875rem;
        background-color: #f1f5f9;
        color: var(--text-secondary);
        padding: 0.25rem 0.75rem;
        border-radius: 999px;
        font-weight: 600;
    }
    
    .status-badge {
        display: flex;
        align-items: center;
        gap: 0.25rem;
        font-size: 0.75rem;
        color: var(--active-color);
        background-color: var(--active-bg);
        padding: 0.25rem 0.5rem;
        border-radius: 0.375rem;
        font-weight: 500;
    }

    .actions-section {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    /* Buttons */
    .btn-icon {
        width: 2rem;
        height: 2rem;
        display: flex;
        justify-content: center;
        align-items: center;
        border-radius: 0.5rem;
        border: 1px solid transparent;
        color: var(--text-secondary);
        background: transparent;
        cursor: pointer;
        transition: all 0.2s;
    }
    
    .btn-icon:hover {
        background-color: #f1f5f9;
        color: var(--text-primary);
    }
    
    .btn-primary {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.5rem 1rem;
        background-color: var(--text-primary);
        color: white;
        border-radius: 0.5rem;
        font-size: 0.875rem;
        font-weight: 600;
        border: none;
        cursor: pointer;
        transition: transform 0.1s;
    }
    
    .btn-primary:active {
        transform: scale(0.98);
    }

    /* Search Input */
    .search-wrapper {
        position: relative;
        display: flex;
        align-items: center;
        width: 16rem;
    }

    .search-input {
        width: 100%;
        padding: 0.5rem 0.75rem 0.5rem 2.25rem;
        border-radius: 0.5rem;
        border: 1px solid var(--border-color);
        background-color: #ffffff;
        font-size: 0.875rem;
        transition: all 0.2s;
    }
    
    .search-input:focus {
        border-color: var(--active-color);
        box-shadow: 0 0 0 2px var(--active-bg);
        outline: none;
    }

    .search-icon {
        position: absolute;
        left: 0.625rem;
        font-size: 1.125rem;
        color: #94a3b8;
        pointer-events: none;
    }

    .shortcut-hint {
        position: absolute;
        right: 0.5rem;
    }
    
    .shortcut-hint kbd {
        background: #f8fafc;
        border: 1px solid #cbd5e1;
        border-radius: 0.25rem;
        padding: 0.1rem 0.3rem;
        font-size: 0.625rem;
        font-family: inherit;
        color: #64748b;
    }

    /* Filter Bar */
    .filter-bar {
        display: flex;
        gap: 1.5rem;
        margin-top: 0.5rem;
    }

    .filter-chip {
        background: none;
        border: none;
        padding: 0.75rem 0;
        font-size: 0.875rem;
        font-weight: 500;
        color: var(--text-secondary);
        cursor: pointer;
        position: relative;
        transition: color 0.2s;
    }
    
    .filter-chip:hover {
        color: var(--text-primary);
    }
    
    .filter-chip.active {
        color: var(--active-color);
        font-weight: 600;
    }
    
    .filter-chip.active::after {
        content: '';
        position: absolute;
        bottom: 0;
        left: 0;
        width: 100%;
        height: 2px;
        background-color: var(--active-color);
        border-radius: 2px 2px 0 0;
    }
    
    .badge-dot {
        display: inline-block;
        width: 6px;
        height: 6px;
        background-color: var(--active-color);
        border-radius: 50%;
        margin-left: 0.25rem;
        vertical-align: middle;
    }

    /* Content Area */
    .content-area {
        flex: 1;
        overflow-y: auto;
        padding: 2rem;
    }
    
    .list-container {
        max-width: 1200px;
        margin: 0 auto;
        background-color: #ffffff;
        border-radius: 1rem;
        border: 1px solid var(--border-color);
        box-shadow: 0 1px 3px rgba(0,0,0,0.05);
        overflow: hidden;
    }

    /* Grid Layout */
    .list-header {
        display: grid;
        grid-template-columns: 3fr 1.5fr auto;
        padding: 1rem 1.5rem;
        background-color: #f8fafc;
        border-bottom: 1px solid var(--border-color);
        font-size: 0.75rem;
        font-weight: 600;
        color: #94a3b8;
        text-transform: uppercase;
        letter-spacing: 0.05em;
    }
    
    .list-item {
        display: grid;
        grid-template-columns: 3fr 1.5fr auto;
        padding: 1rem 1.5rem;
        align-items: center;
        border-bottom: 1px solid var(--border-color);
        transition: all 0.15s ease-out;
    }
    
    .list-item:last-child {
        border-bottom: none;
    }
    
    .list-item:hover {
        background-color: #f8fafc;
    }

    /* Columns */
    .col-main {
        display: flex;
        align-items: center;
        gap: 1.25rem;
        overflow: hidden; /* Prevent text overflow */
    }
    
    .col-meta {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: flex-start; /* Left align text */
        gap: 0.25rem;
    }
    
    .col-action {
        display: flex;
        justify-content: flex-end;
        align-items: center;
        gap: 0.5rem;
    }

    /* App Elements */
    .app-avatar {
        width: 3rem;
        height: 3rem;
        border-radius: 0.75rem;
        display: flex;
        align-items: center;
        justify-content: center;
        color: white;
        font-weight: 700;
        font-size: 1.125rem;
        flex-shrink: 0;
        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        position: relative;
    }
    
    .avatar-badge {
        position: absolute;
        top: -2px;
        right: -2px;
        width: 10px;
        height: 10px;
        background-color: var(--active-color);
        border: 2px solid white;
        border-radius: 50%;
    }

    .app-info {
        display: flex;
        flex-direction: column;
        justify-content: center;
        min-width: 0; /* Enable truncation */
    }

    .app-name {
        margin: 0;
        font-size: 1rem;
        font-weight: 700;
        color: var(--text-primary);
        line-height: 1.4;
    }

    .app-desc {
        margin: 0;
        font-size: 0.8125rem;
        color: var(--text-secondary);
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
        max-width: 100%;
        line-height: 1.4;
    }
    
    .app-desc .sep {
        margin: 0 0.25rem;
        opacity: 0.5;
    }
    
    .highlight {
        color: var(--text-primary);
        font-weight: 500;
    }

    /* Meta Tags */
    .version-tag {
        display: inline-flex;
        
    }
    
    .ver-num {
        font-family: 'JetBrains Mono', 'Fira Code', monospace; /* Monospaced for numbers */
        font-size: 0.8125rem;
        font-weight: 600;
        color: var(--text-primary);
        background-color: #f1f5f9;
        padding: 0.125rem 0.5rem;
        border-radius: 0.25rem;
    }

    .bucket-tag {
        font-size: 0.75rem;
        color: #94a3b8;
        display: flex;
        align-items: center;
        gap: 0.25rem;
    }
    
    .bucket-tag span {
        font-size: 0.875rem;
    }

    /* Action Buttons */
    .btn-action {
        width: 2.25rem;
        height: 2.25rem;
        border-radius: 0.5rem;
        border: 1px solid transparent;
        background: transparent;
        display: flex;
        align-items: center;
        justify-content: center;
        cursor: pointer;
        transition: all 0.2s;
        color: var(--text-secondary);
    }
    
    .btn-action:hover {
        background-color: white;
        border-color: var(--border-color);
        box-shadow: 0 1px 2px rgba(0,0,0,0.05);
    }
    
    .btn-action.update:hover {
        color: var(--active-color);
        background-color: var(--active-bg);
        border-color: transparent;
    }
    
    .btn-action.delete:hover {
        color: var(--danger-color);
        background-color: #fef2f2;
        border-color: transparent;
    }

    /* Empty States */
    .state-empty {
        padding: 4rem;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        color: #94a3b8;
    }
    
    .state-empty .icon {
        font-size: 3rem;
        margin-bottom: 1rem;
        opacity: 0.5;
    }
    
    .state-error {
        padding: 3rem;
        text-align: center;
        color: var(--danger-color);
    }
    
    .list-footer {
        max-width: 1200px;
        margin: 1rem auto 0;
        text-align: center;
        font-size: 0.75rem;
        color: #94a3b8;
    }

    /* Spinners */
    .spinning {
        animation: spin 1s linear infinite;
    }
    
    .spin-slow {
        animation: spin 2s linear infinite;
    }
    
    @keyframes spin {
        from { transform: rotate(0deg); }
        to { transform: rotate(360deg); }
    }
</style>
