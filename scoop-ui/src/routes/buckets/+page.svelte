<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';
    import { listen } from '@tauri-apps/api/event';
    import { bucketsStore } from '$lib/stores';
    import { updateScoop } from '$lib/scoop';
    import ProgressModal from '$lib/components/ProgressModal.svelte';
    import recommendedBucketsData from '$lib/data/recommended-buckets.json';

    interface ScoopBucket {
        name: String,
        source: String,
        updated: number,
    }
    
    interface RecommendedBucket {
        name: string;
        url: string;
        description: string;
        apps_count: string;
        category: string;
        icon: string;
    }

    let buckets: ScoopBucket[] = [];
    let loading = true;
    let adding = false;
    let removingStr = ""; // name of bucket being removed
    let updatingScoop = false;
    let addingRecommended = new Set<string>(); // Track which recommended buckets are being added
    
    // 进度对话框
    let showProgress = false;
    let progressTitle = "";
    let progressMessage = "";
    let progressValue = 0;
    let progressStatus: 'idle' | 'running' | 'success' | 'error' = 'idle';
    let progressLogs: string[] = [];
    
    let unlistenScoopUpdate: (() => void) | null = null;
    
    let showAddModal = false;
    let newBucketName = "";
    let newBucketUrl = "";
    let addError = "";
    
    // 推荐 buckets
    const recommendedBuckets: RecommendedBucket[] = recommendedBucketsData;
    
    // 计算已安装的 bucket 名称集合
    $: installedBucketNames = new Set(buckets.map(b => b.name.toString().toLowerCase()));
    
    // Subscribe to store
    $: buckets = $bucketsStore;
    $: if ($bucketsStore.length > 0) {
        loading = false;
    }

    async function refreshBuckets() {
        loading = true;
        try {
            const b = await invoke<ScoopBucket[]>('get_buckets');
            bucketsStore.set(b);
            localStorage.setItem('scoop_buckets_cache', JSON.stringify(b));
        } catch (e) {
            console.error('Failed to load buckets:', e);
        } finally {
            loading = false;
        }
    }

    // Replace loadBuckets calls with refreshBuckets
    const loadBuckets = refreshBuckets; 

    async function handleUpdateScoop() {
        if (updatingScoop) return;
        
        if (!confirm("Update Scoop and all buckets? This might take a while.")) return;
        
        updatingScoop = true;
        showProgress = true;
        progressTitle = "更新 Scoop";
        progressMessage = "准备更新...";
        progressValue = 0;
        progressStatus = 'running';
        progressLogs = [];
        
        try {
            await updateScoop();
            // 成功会通过事件更新
        } catch (e) {
            progressStatus = 'error';
            progressMessage = `更新失败: ${e}`;
            updatingScoop = false;
        }
    }
    
    function handleProgressClose() {
        showProgress = false;
        progressLogs = [];
    }

    async function handleAddBucket() {
        if (!newBucketName) return;
        
        adding = true;
        addError = "";
        try {
            await invoke('add_bucket', { 
                name: newBucketName, 
                url: newBucketUrl ? newBucketUrl : null 
            });
            showAddModal = false;
            newBucketName = "";
            newBucketUrl = "";
            await refreshBuckets(); // Force refresh
        } catch (e) {
            addError = String(e);
        } finally {
            adding = false;
        }
    }

    async function handleRemoveBucket(name: String) {
        if (!confirm(`Are you sure you want to remove bucket "${name}"?`)) return;

        removingStr = name.toString();
        try {
            await invoke('remove_bucket', { name });
            await refreshBuckets(); // Force refresh
        } catch (e) {
            alert(`Failed to remove bucket: ${e}`);
        } finally {
            removingStr = "";
        }
    }
    
    async function handleAddRecommended(bucket: RecommendedBucket) {
        if (installedBucketNames.has(bucket.name.toLowerCase())) {
            return; // Already installed
        }
        
        addingRecommended.add(bucket.name);
        addingRecommended = addingRecommended;
        
        try {
            await invoke('add_bucket', { 
                name: bucket.name, 
                url: bucket.url 
            });
            await refreshBuckets();
        } catch (e) {
            alert(`Failed to add bucket ${bucket.name}: ${e}`);
        } finally {
            addingRecommended.delete(bucket.name);
            addingRecommended = addingRecommended;
        }
    }

    onMount(async () => {
         if ($bucketsStore.length === 0) {
             await refreshBuckets();
         } else {
             loading = false;
         }
         
         // 监听 Scoop 更新进度事件
         unlistenScoopUpdate = await listen('scoop-update-progress', (event: any) => {
             const data = event.payload;
             progressMessage = data.message;
             progressValue = data.progress;
             
             // 只在状态变化时添加日志
             if (data.status === 'starting' || data.status === 'completed' || data.status === 'error') {
                 progressLogs = [...progressLogs, data.message];
             }
             
             if (data.status === 'completed') {
                 progressStatus = 'success';
                 updatingScoop = false;
                 
                 // 刷新 buckets 列表
                 setTimeout(async () => {
                     await refreshBuckets();
                     showProgress = false;
                     progressLogs = [];
                 }, 1500);
             } else if (data.status === 'error') {
                 progressStatus = 'error';
                 updatingScoop = false;
             }
         });
    });
    
    onDestroy(() => {
        if (unlistenScoopUpdate) {
            unlistenScoopUpdate();
        }
    });
</script>

<div class="page-container">
    <div class="page-header">
        <div class="header-left">
            <h1 class="title">Buckets</h1>
            <button 
                class="btn-refresh" 
                title="Refresh List"
                disabled={loading}
                on:click={refreshBuckets}
            >
                <span class="material-symbols-outlined" class:spinning={loading}>refresh</span>
            </button>
            <button 
                class="btn-refresh" 
                title="Update Scoop & Buckets"
                disabled={loading || updatingScoop}
                on:click={handleUpdateScoop}
            >
                <span class="material-symbols-outlined" class:spinning={updatingScoop}>sync</span>
            </button>
        </div>
        <button class="add-btn" on:click={() => showAddModal = true}>
            <span class="material-symbols-outlined">add</span>
            Add Bucket
        </button>
    </div>
    
    <!-- 推荐 Buckets 区域 -->
    <div class="recommended-section">
        <h2 class="section-title">
            <span class="material-symbols-outlined">recommend</span>
            推荐 Buckets
        </h2>
        <div class="recommended-grid">
            {#each recommendedBuckets as bucket}
                {@const isInstalled = installedBucketNames.has(bucket.name.toLowerCase())}
                {@const isAdding = addingRecommended.has(bucket.name)}
                <div class="recommended-card">
                    <div class="recommended-icon">
                        <span class="material-symbols-outlined">{bucket.icon}</span>
                    </div>
                    <div class="recommended-info">
                        <div class="recommended-header">
                            <h3>{bucket.name}</h3>
                            <span class="category-badge">{bucket.category}</span>
                        </div>
                        <p class="description">{bucket.description}</p>
                        <p class="apps-count">{bucket.apps_count} 应用</p>
                    </div>
                    <div class="recommended-action">
                        {#if isInstalled}
                            <div class="installed-badge">
                                <span class="material-symbols-outlined">check_circle</span>
                                已添加
                            </div>
                        {:else if isAdding}
                            <button class="add-recommended-btn adding" disabled>
                                <span class="material-symbols-outlined spin">progress_activity</span>
                                添加中
                            </button>
                        {:else}
                            <button class="add-recommended-btn" on:click={() => handleAddRecommended(bucket)}>
                                <span class="material-symbols-outlined">add</span>
                                添加
                            </button>
                        {/if}
                    </div>
                </div>
            {/each}
        </div>
    </div>
    
    <!-- 已安装的 Buckets -->
    <div class="installed-section">
        <h2 class="section-title">
            <span class="material-symbols-outlined">folder</span>
            已安装的 Buckets
        </h2>
        
        {#if loading}
            <div class="loading-state">
                <span class="material-symbols-outlined spin">sync</span>
                <p>Loading buckets...</p>
            </div>
        {:else if buckets.length === 0}
            <div class="empty-state">
                <span class="material-symbols-outlined">folder_off</span>
                <p>No buckets found</p>
            </div>
        {:else}
            <div class="buckets-grid">
                {#each buckets as bucket}
                    <div class="bucket-card">
                        <div class="bucket-icon">
                            <span class="material-symbols-outlined">folder</span>
                        </div>
                        <div class="bucket-info">
                            <h3>{bucket.name}</h3>
                            <p class="source" title={bucket.source.toString()}>{bucket.source}</p>
                        </div>
                        <button 
                            class="remove-btn" 
                            title="Remove Bucket"
                            disabled={removingStr === bucket.name}
                            on:click={() => handleRemoveBucket(bucket.name)}
                        >
                            {#if removingStr === bucket.name}
                                <span class="material-symbols-outlined spin">sync</span>
                            {:else}
                                <span class="material-symbols-outlined">delete</span>
                            {/if}
                        </button>
                    </div>
                {/each}
            </div>
        {/if}
    </div>
</div>

{#if showAddModal}
    <div class="modal-backdrop" on:click|self={() => showAddModal = false}>
        <div class="modal">
            <h2>Add Bucket</h2>
            <div class="form-group">
                <label for="b-name">Name</label>
                <input 
                    id="b-name" 
                    type="text" 
                    bind:value={newBucketName} 
                    placeholder="e.g. extras" 
                    disabled={adding}
                />
            </div>
            <div class="form-group">
                <label for="b-url">URL (Optional)</label>
                <input 
                    id="b-url" 
                    type="text" 
                    bind:value={newBucketUrl} 
                    placeholder="https://github.com/..." 
                    disabled={adding}
                />
            </div>
            
            {#if addError}
                <div class="error-msg">{addError}</div>
            {/if}

            <div class="modal-actions">
                <button class="btn-cancel" on:click={() => showAddModal = false} disabled={adding}>Cancel</button>
                <button class="btn-confirm" on:click={handleAddBucket} disabled={!newBucketName || adding}>
                    {#if adding}
                        Adding...
                    {:else}
                        Add Bucket
                    {/if}
                </button>
            </div>
        </div>
    </div>
{/if}

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

<style>
    .page-container {
        height: 100%;
        display: flex;
        flex-direction: column;
        padding: 2rem;
        box-sizing: border-box;
    }

    .page-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 2rem;
    }
    
    .recommended-section, .installed-section {
        margin-bottom: 2rem;
    }
    
    .section-title {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        font-size: 1.25rem;
        font-weight: 600;
        color: var(--text-main);
        margin: 0 0 1rem 0;
    }
    
    .section-title .material-symbols-outlined {
        font-size: 1.5rem;
        color: var(--primary);
    }
    
    .recommended-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
        gap: 1rem;
        margin-bottom: 1rem;
    }
    
    .recommended-card {
        background-color: var(--bg-card);
        border: 1px solid var(--border);
        border-radius: 0.75rem;
        padding: 1rem;
        display: flex;
        gap: 1rem;
        transition: transform 0.2s, box-shadow 0.2s;
    }
    
    .recommended-card:hover {
        transform: translateY(-2px);
        box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
    }
    
    .recommended-icon {
        width: 3.5rem;
        height: 3.5rem;
        border-radius: 0.5rem;
        background: linear-gradient(135deg, rgba(25, 127, 230, 0.1), rgba(25, 127, 230, 0.2));
        color: var(--primary);
        display: grid;
        place-items: center;
        flex-shrink: 0;
    }
    
    .recommended-icon .material-symbols-outlined {
        font-size: 2rem;
    }
    
    .recommended-info {
        flex: 1;
        min-width: 0;
    }
    
    .recommended-header {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        margin-bottom: 0.5rem;
    }
    
    .recommended-info h3 {
        margin: 0;
        font-size: 1rem;
        font-weight: 600;
        color: var(--text-main);
    }
    
    .category-badge {
        font-size: 0.625rem;
        font-weight: 600;
        padding: 0.125rem 0.5rem;
        border-radius: 9999px;
        background-color: rgba(25, 127, 230, 0.1);
        color: var(--primary);
    }
    
    .recommended-info .description {
        margin: 0 0 0.5rem 0;
        font-size: 0.875rem;
        color: var(--text-muted);
        line-height: 1.4;
    }
    
    .recommended-info .apps-count {
        margin: 0;
        font-size: 0.75rem;
        color: var(--text-muted);
        font-weight: 500;
    }
    
    .recommended-action {
        display: flex;
        align-items: center;
    }
    
    .add-recommended-btn {
        display: flex;
        align-items: center;
        gap: 0.375rem;
        padding: 0.5rem 1rem;
        border-radius: 0.5rem;
        border: none;
        background-color: var(--primary);
        color: white;
        font-weight: 600;
        font-size: 0.875rem;
        cursor: pointer;
        transition: opacity 0.2s;
        white-space: nowrap;
    }
    
    .add-recommended-btn:hover:not(:disabled) {
        opacity: 0.9;
    }
    
    .add-recommended-btn:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }
    
    .add-recommended-btn .material-symbols-outlined {
        font-size: 18px;
    }
    
    .installed-badge {
        display: flex;
        align-items: center;
        gap: 0.375rem;
        color: #10b981;
        font-weight: 600;
        font-size: 0.875rem;
        padding: 0.5rem 0.75rem;
    }
    
    .installed-badge .material-symbols-outlined {
        font-size: 18px;
    }

    .header-left {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    .title {
        font-size: 1.875rem;
        font-weight: 700;
        color: var(--text-main);
        margin: 0;
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

    .spinning {
        animation: spin 1s linear infinite;
    }

    .add-btn {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.5rem 1rem;
        border-radius: 0.5rem;
        border: none;
        background-color: var(--primary);
        color: white;
        font-weight: 500;
        cursor: pointer;
        transition: opacity 0.2s;
    }

    .add-btn:hover {
        opacity: 0.9;
    }

    .loading-state, .empty-state {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        color: var(--text-muted);
    }

    .loading-state .spin {
        font-size: 2rem;
        animation: spin 1s linear infinite;
        margin-bottom: 0.5rem;
    }

    @keyframes spin {
        from { transform: rotate(0deg); }
        to { transform: rotate(360deg); }
    }

    .buckets-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
        gap: 1rem;
        overflow-y: auto;
    }

    .bucket-card {
        background-color: var(--bg-card); /* Ensure this var exists or assume generic */
        border: 1px solid var(--border);
        border-radius: 0.75rem;
        padding: 1rem;
        display: flex;
        align-items: center;
        gap: 1rem;
        transition: transform 0.2s, box-shadow 0.2s;
    }

    .bucket-card:hover {
        transform: translateY(-2px);
        box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
    }

    .bucket-icon {
        width: 3rem;
        height: 3rem;
        border-radius: 0.5rem;
        background-color: rgba(25, 127, 230, 0.1); /* primary/10 */
        color: var(--primary);
        display: grid;
        place-items: center;
        flex-shrink: 0;
    }

    .bucket-info {
        flex: 1;
        min-width: 0;
    }

    .bucket-info h3 {
        margin: 0;
        font-size: 1rem;
        font-weight: 600;
        color: var(--text-main);
    }

    .bucket-info .source {
        margin: 0.25rem 0 0;
        font-size: 0.75rem;
        color: var(--text-muted);
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .remove-btn {
        background: transparent;
        border: none;
        color: var(--text-muted);
        cursor: pointer;
        padding: 0.5rem;
        border-radius: 0.25rem;
        transition: color 0.2s, background-color 0.2s;
        display: grid;
        place-items: center;
    }

    .remove-btn:hover:not(:disabled) {
        color: #ef4444; /* red-500 */
        background-color: rgba(239, 68, 68, 0.1);
    }

    .remove-btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .remove-btn .spin {
        animation: spin 1s linear infinite;
        font-size: 1.25rem;
    }

    /* Modal */
    .modal-backdrop {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background-color: rgba(0, 0, 0, 0.5);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 50;
        backdrop-filter: blur(2px);
    }

    .modal {
        background-color: var(--bg-surface);
        border: 1px solid var(--border);
        border-radius: 1rem;
        padding: 1.5rem;
        width: 100%;
        max-width: 400px;
        box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
    }

    .modal h2 {
        margin-top: 0;
        margin-bottom: 1.5rem;
        color: var(--text-main);
    }

    .form-group {
        margin-bottom: 1rem;
    }

    .form-group label {
        display: block;
        margin-bottom: 0.5rem;
        font-size: 0.875rem;
        font-weight: 500;
        color: var(--text-main);
    }

    .form-group input {
        width: 100%;
        padding: 0.5rem 0.75rem;
        border-radius: 0.375rem;
        border: 1px solid var(--border);
        background-color: var(--bg-main);
        color: var(--text-main);
        box-sizing: border-box;
    }

    .error-msg {
        color: #ef4444;
        font-size: 0.875rem;
        margin-bottom: 1rem;
        padding: 0.5rem;
        background-color: rgba(239, 68, 68, 0.1);
        border-radius: 0.375rem;
    }

    .modal-actions {
        display: flex;
        justify-content: flex-end;
        gap: 0.75rem;
        margin-top: 1.5rem;
    }

    .btn-cancel {
        padding: 0.5rem 1rem;
        border-radius: 0.375rem;
        border: 1px solid var(--border);
        background-color: transparent;
        color: var(--text-main);
        cursor: pointer;
    }

    .btn-confirm {
        padding: 0.5rem 1rem;
        border-radius: 0.375rem;
        border: none;
        background-color: var(--primary);
        color: white;
        font-weight: 500;
        cursor: pointer;
    }

    .btn-confirm:disabled {
        opacity: 0.7;
        cursor: not-allowed;
    }
</style>
