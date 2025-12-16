<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    
    export let show = false;
    export let title = "进度";
    export let message = "";
    export let progress = 0; // 0-1
    export let status: 'idle' | 'running' | 'success' | 'error' = 'idle';
    export let logs: string[] = [];
    export let cancelable = false;
    
    const dispatch = createEventDispatcher();
    
    function handleCancel() {
        dispatch('cancel');
    }
    
    function handleClose() {
        if (status === 'success' || status === 'error') {
            dispatch('close');
        }
    }
</script>

{#if show}
    <div class="modal-backdrop" on:click|self={handleClose}>
        <div class="modal">
            <div class="modal-header">
                <h2>{title}</h2>
                {#if status === 'success' || status === 'error'}
                    <button class="close-btn" on:click={handleClose}>
                        <span class="material-symbols-outlined">close</span>
                    </button>
                {/if}
            </div>
            
            <div class="modal-body">
                <!-- 状态图标 -->
                <div class="status-icon">
                    {#if status === 'running'}
                        <span class="material-symbols-outlined spin">progress_activity</span>
                    {:else if status === 'success'}
                        <span class="material-symbols-outlined success">check_circle</span>
                    {:else if status === 'error'}
                        <span class="material-symbols-outlined error">error</span>
                    {/if}
                </div>
                
                <!-- 消息 -->
                <p class="message">{message}</p>
                
                <!-- 进度条 -->
                {#if status === 'running'}
                    <div class="progress-bar">
                        <div class="progress-fill" style="width: {progress * 100}%"></div>
                    </div>
                    <p class="progress-text">{Math.round(progress * 100)}%</p>
                {/if}
                
                <!-- 日志 -->
                {#if logs.length > 0}
                    <div class="logs">
                        {#each logs as log}
                            <div class="log-line">{log}</div>
                        {/each}
                    </div>
                {/if}
            </div>
            
            <div class="modal-footer">
                {#if cancelable && status === 'running'}
                    <button class="btn-cancel" on:click={handleCancel}>取消</button>
                {/if}
                {#if status === 'success' || status === 'error'}
                    <button class="btn-primary" on:click={handleClose}>关闭</button>
                {/if}
            </div>
        </div>
    </div>
{/if}

<style>
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
        z-index: 1000;
        backdrop-filter: blur(2px);
    }
    
    .modal {
        background-color: var(--bg-surface);
        border: 1px solid var(--border);
        border-radius: 1rem;
        padding: 1.5rem;
        width: 90%;
        max-width: 500px;
        box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
    }
    
    .modal-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 1.5rem;
    }
    
    .modal-header h2 {
        margin: 0;
        font-size: 1.25rem;
        font-weight: 700;
        color: var(--text-main);
    }
    
    .close-btn {
        background: transparent;
        border: none;
        cursor: pointer;
        color: var(--text-muted);
        padding: 0.25rem;
        border-radius: 0.25rem;
        display: flex;
        align-items: center;
        justify-content: center;
    }
    
    .close-btn:hover {
        background-color: rgba(0, 0, 0, 0.05);
    }
    
    .modal-body {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
        margin-bottom: 1.5rem;
    }
    
    .status-icon {
        font-size: 3rem;
    }
    
    .status-icon .spin {
        color: var(--primary);
        animation: spin 1s linear infinite;
    }
    
    .status-icon .success {
        color: #10b981;
    }
    
    .status-icon .error {
        color: #ef4444;
    }
    
    .message {
        text-align: center;
        color: var(--text-main);
        margin: 0;
        font-size: 0.875rem;
    }
    
    .progress-bar {
        width: 100%;
        height: 0.5rem;
        background-color: var(--bg-surface-2);
        border-radius: 9999px;
        overflow: hidden;
    }
    
    .progress-fill {
        height: 100%;
        background-color: var(--primary);
        transition: width 0.3s ease;
    }
    
    .progress-text {
        font-size: 0.75rem;
        color: var(--text-muted);
        margin: 0;
    }
    
    .logs {
        width: 100%;
        max-height: 200px;
        overflow-y: auto;
        background-color: #1e293b;
        border-radius: 0.5rem;
        padding: 0.75rem;
        font-family: 'Consolas', 'Monaco', monospace;
        font-size: 0.75rem;
    }
    
    .log-line {
        color: #cbd5e1;
        margin-bottom: 0.25rem;
        white-space: pre-wrap;
        word-break: break-all;
    }
    
    .modal-footer {
        display: flex;
        justify-content: flex-end;
        gap: 0.75rem;
    }
    
    .btn-primary {
        padding: 0.5rem 1.5rem;
        border-radius: 0.5rem;
        border: none;
        background-color: var(--primary);
        color: white;
        font-weight: 600;
        cursor: pointer;
        transition: opacity 0.2s;
    }
    
    .btn-primary:hover {
        opacity: 0.9;
    }
    
    .btn-cancel {
        padding: 0.5rem 1.5rem;
        border-radius: 0.5rem;
        border: 1px solid var(--border);
        background-color: transparent;
        color: var(--text-main);
        font-weight: 600;
        cursor: pointer;
    }
    
    .btn-cancel:hover {
        background-color: rgba(0, 0, 0, 0.05);
    }
    
    @keyframes spin {
        from { transform: rotate(0deg); }
        to { transform: rotate(360deg); }
    }
</style>
