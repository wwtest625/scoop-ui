<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    
    export let show = false;
    export let title = "确认";
    export let message = "";
    export let confirmText = "确认";
    export let cancelText = "取消";
    export let type: 'info' | 'warning' | 'danger' = 'info';
    
    const dispatch = createEventDispatcher();
    
    function handleConfirm() {
        dispatch('confirm');
    }
    
    function handleCancel() {
        dispatch('cancel');
    }
</script>

{#if show}
    <div class="modal-backdrop" on:click|self={handleCancel}>
        <div class="modal">
            <div class="modal-header">
                <div class="icon-wrapper" class:warning={type === 'warning'} class:danger={type === 'danger'}>
                    {#if type === 'warning'}
                        <span class="material-symbols-outlined">warning</span>
                    {:else if type === 'danger'}
                        <span class="material-symbols-outlined">error</span>
                    {:else}
                        <span class="material-symbols-outlined">info</span>
                    {/if}
                </div>
                <h2>{title}</h2>
            </div>
            
            <div class="modal-body">
                <p>{message}</p>
            </div>
            
            <div class="modal-footer">
                <button class="btn-cancel" on:click={handleCancel}>{cancelText}</button>
                <button 
                    class="btn-confirm" 
                    class:danger={type === 'danger'}
                    on:click={handleConfirm}
                >
                    {confirmText}
                </button>
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
        max-width: 400px;
        box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
    }
    
    .modal-header {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
        margin-bottom: 1rem;
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
    
    .icon-wrapper.warning {
        background-color: rgba(245, 158, 11, 0.1);
        color: #f59e0b;
    }
    
    .icon-wrapper.danger {
        background-color: rgba(239, 68, 68, 0.1);
        color: #ef4444;
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
    
    .modal-body p {
        text-align: center;
        color: var(--text-main);
        margin: 0;
        line-height: 1.5;
    }
    
    .modal-footer {
        display: flex;
        justify-content: flex-end;
        gap: 0.75rem;
    }
    
    .btn-cancel {
        padding: 0.5rem 1.5rem;
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
    
    .btn-confirm {
        padding: 0.5rem 1.5rem;
        border-radius: 0.5rem;
        border: none;
        background-color: var(--primary);
        color: white;
        font-weight: 600;
        cursor: pointer;
        transition: opacity 0.2s;
    }
    
    .btn-confirm:hover {
        opacity: 0.9;
    }
    
    .btn-confirm.danger {
        background-color: #ef4444;
    }
</style>
