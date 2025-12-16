<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    
    export let show = false;
    export let appName = "";
    export let dependencies: string[] = [];
    
    const dispatch = createEventDispatcher();
    
    function handleInstallAll() {
        dispatch('install-all');
    }
    
    function handleSkip() {
        dispatch('skip');
    }
    
    function handleCancel() {
        dispatch('cancel');
    }
</script>

{#if show}
    <div class="modal-backdrop" on:click|self={handleCancel}>
        <div class="modal">
            <div class="modal-header">
                <div class="icon-wrapper">
                    <span class="material-symbols-outlined">info</span>
                </div>
                <h2>检测到依赖项</h2>
            </div>
            
            <div class="modal-body">
                <p class="message">
                    <strong>{appName}</strong> 需要以下依赖项:
                </p>
                
                <ul class="dependency-list">
                    {#each dependencies as dep}
                        <li>
                            <span class="material-symbols-outlined">package_2</span>
                            {dep}
                        </li>
                    {/each}
                </ul>
                
                <p class="hint">是否同时安装这些依赖项?</p>
            </div>
            
            <div class="modal-footer">
                <button class="btn-cancel" on:click={handleCancel}>取消</button>
                <button class="btn-secondary" on:click={handleSkip}>跳过依赖</button>
                <button class="btn-primary" on:click={handleInstallAll}>全部安装</button>
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
        max-width: 450px;
        box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
    }
    
    .modal-header {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
        margin-bottom: 1.5rem;
    }
    
    .icon-wrapper {
        width: 3rem;
        height: 3rem;
        border-radius: 50%;
        background-color: rgba(25, 127, 230, 0.1);
        color: var(--primary);
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 2rem;
    }
    
    .modal-header h2 {
        margin: 0;
        font-size: 1.25rem;
        font-weight: 700;
        color: var(--text-main);
        text-align: center;
    }
    
    .modal-body {
        margin-bottom: 1.5rem;
    }
    
    .message {
        text-align: center;
        color: var(--text-main);
        margin: 0 0 1rem 0;
    }
    
    .dependency-list {
        list-style: none;
        padding: 0;
        margin: 0 0 1rem 0;
        background-color: var(--bg-surface-2);
        border-radius: 0.5rem;
        padding: 0.75rem;
    }
    
    .dependency-list li {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.5rem;
        color: var(--text-main);
        font-weight: 500;
    }
    
    .dependency-list li .material-symbols-outlined {
        font-size: 1.25rem;
        color: var(--primary);
    }
    
    .hint {
        text-align: center;
        color: var(--text-muted);
        font-size: 0.875rem;
        margin: 0;
    }
    
    .modal-footer {
        display: flex;
        justify-content: flex-end;
        gap: 0.75rem;
    }
    
    .btn-cancel {
        padding: 0.5rem 1rem;
        border-radius: 0.5rem;
        border: 1px solid var(--border);
        background-color: transparent;
        color: var(--text-main);
        font-weight: 600;
        cursor: pointer;
        transition: background-color 0.2s;
    }
    
    .btn-cancel:hover {
        background-color: rgba(0, 0, 0, 0.05);
    }
    
    .btn-secondary {
        padding: 0.5rem 1rem;
        border-radius: 0.5rem;
        border: 1px solid var(--border);
        background-color: var(--bg-surface-2);
        color: var(--text-main);
        font-weight: 600;
        cursor: pointer;
        transition: background-color 0.2s;
    }
    
    .btn-secondary:hover {
        background-color: var(--border);
    }
    
    .btn-primary {
        padding: 0.5rem 1rem;
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
</style>
