<script>
  import '../app.css';
  import Sidebar from '$lib/components/Sidebar.svelte';

  // In a real app we might handle theme toggling here
</script>

<div class="app-layout">
  <Sidebar />
  <main class="main-content">
    <slot />
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
  }
</style>
