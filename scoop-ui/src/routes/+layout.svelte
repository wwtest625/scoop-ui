<script>
  import '../app.css';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import { onMount } from 'svelte';
  import { initializeData, appLoadingStore } from '$lib/stores';

  let loading = true;

  // subscribe to store
  $: loading = $appLoadingStore;

  onMount(() => {
    initializeData();
  });
</script>

<div class="app-layout">
  <Sidebar />
  <main class="main-content">
    {#if loading}
      <!-- Loading Overlay -->
      <div class="loading-overlay">
        <div class="loading-content">
             <!-- You can use a logo here if you have one -->
             <span class="material-symbols-outlined logo-icon">terminal</span>
             <h1 class="loading-title">Scoop UI</h1>
             <div class="loader">
                <div class="loader-bar"></div>
             </div>
             <p class="loading-text">Initializing Scoop...</p>
        </div>
      </div>
    {:else}
      <slot />
    {/if}
  </main>
</div>

<style>
  .app-layout {
    display: flex;
    width: 100vw;
    height: 100vh;
    overflow: hidden;
    background-color: var(--bg-light);
    color: var(--text-main);
  }

  .main-content {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    position: relative;
    display: flex;
    flex-direction: column;
  }
  
  :global(.dark) .app-layout {
      background-color: var(--bg-subtle); /* Ensure dark mode bg matches */
  }

  /* Loading Overlay */
  .loading-overlay {
      position: absolute;
      top: 0; left: 0; right: 0; bottom: 0;
      background-color: var(--bg-light);
      display: flex;
      align-items: center;
      justify-content: center;
      z-index: 50;
  }
  
  :global(.dark) .loading-overlay {
      background-color: #0f172a;
  }

  .loading-content {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 1.5rem;
  }

  .logo-icon {
      font-size: 4rem;
      color: var(--primary);
  }

  .loading-title {
      font-size: 2rem;
      font-weight: 800;
      margin: 0;
      color: var(--text-main);
  }

  .loading-text {
      color: var(--text-muted);
      font-size: 0.875rem;
      margin: 0;
  }

  .loader {
      width: 200px;
      height: 4px;
      background-color: var(--border);
      border-radius: 9999px;
      overflow: hidden;
      position: relative;
  }

  .loader-bar {
      position: absolute;
      top: 0; bottom: 0; left: 0;
      width: 50%;
      background-color: var(--primary);
      border-radius: 9999px;
      animation: indeterminate 1.5s infinite ease-in-out;
  }

  @keyframes indeterminate {
      0% { left: -50%; width: 50%; }
      50% { left: 25%; width: 50%; }
      100% { left: 100%; width: 50%; }
  }
</style>
