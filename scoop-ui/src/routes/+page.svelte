<script>
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';
    import AppCard from '$lib/components/AppCard.svelte';
    import AppDetailModal from '$lib/components/AppDetailModal.svelte';

    const categories = ['All'];
    let selectedCategory = 'All';

    let essentialTools = [];
    let loading = true;
    let error = null;

    // Modal state
    let isModalOpen = false;
    let selectedApp = null;

    function openAppDetail(app) {
        selectedApp = app;
        isModalOpen = true;
    }

    function closeModal() {
        isModalOpen = false;
        selectedApp = null;
    }

    onMount(async () => {
        try {
            // Load 8 random apps
            essentialTools = await invoke('get_random_apps', { count: 8 });
            loading = false;
        } catch (e) {
            console.error('Failed to load apps:', e);
            error = e;
            loading = false;
        }
    });

    const productivity = [
        { rank: 1, name: 'Obsidian', desc: 'Knowledge base', icon: 'note_alt', color: 'purple', isSymbol: true },
        { rank: 2, name: 'Firefox', desc: 'Web Browser', icon: 'https://lh3.googleusercontent.com/aida-public/AB6AXuAl3bjrG3Jnv__sgjlYL4bKErg0AItHEBlr7isceRW1nj0Ur2NS99fRQQP38_b_YqF6ahIe3_VF4nqxG7tche0r61dyRb_WeJCHjC7YfBvUu-W7X5Up5lXjt3DST99mpop9NjolsYHMTd-It53Q8r0NpwOTkFqHj_5Pt1UO7BX7g3HsIwTWid6AmjPcgwH4keskpcAYZkU_kvTHx0RQfU9FUFyNow1VfzlG02IVu_5tw2hIn6l4v2BNqH4aQ0Z6nuv_N6d-1c7NMVCA', isSymbol: false, color: 'orange' },
        { rank: 3, name: 'Windows Terminal', desc: 'System Tool', icon: 'terminal', isSymbol: true, color: 'slate' }
    ];

    // Reactive filtering
    $: filteredTools = selectedCategory === 'All' 
        ? essentialTools 
        : essentialTools.filter(t => t.category === selectedCategory);
</script>

<div class="page-container">
    <!-- Header -->
    <header class="page-header">
        <h2 class="page-title">Discover</h2>
        <div class="categories">
            {#each categories as cat}
                <button 
                    class="cat-btn" 
                    class:active={selectedCategory === cat}
                    on:click={() => selectedCategory = cat}
                >
                    {cat}
                </button>
            {/each}
        </div>
    </header>

    <!-- Hero Section -->
    <div class="hero-section">
        <div class="hero-card">
            <div class="hero-image" style="background-image: url('https://lh3.googleusercontent.com/aida-public/AB6AXuAwr0Lpkip9tpZPD47gSA1aeET9bVA-Iq_VXxllVkVux4L4NRvFFKEAF12ye-HhCiI2d4hHZtPkTxsWIMfdBRAsJZtvuCR5nWuU_uowHFz2SV3uEwokMODsbzuahcoNUweIvJt4VsWErmB2pnOFF4WEChtELwKC8Gwm1r17fAmiq0e41Xz-hfPC61vdJykmwTacQvqtvZXSBPzd2MCO7Hu5aZbF2bAXNtk7WPIxUVVJnzvnRhCrqnRvwziB_4Llri57haeLycyhkl3a')">
                <div class="hero-overlay"></div>
                <div class="hero-badge">Editor's Choice</div>
            </div>
            <div class="hero-content">
                <div class="hero-app">
                    <div class="hero-icon">
                        <img src="https://lh3.googleusercontent.com/aida-public/AB6AXuD_cQlC5RvNOVnrCsL7Ri7RNwhTZQt5LxXsLi55HCUZ6lfMTddEZuUrVFAvAsjq3dsBOQdyeG1EwW1ExasNfW2YllVPyVliddBIDBSBhI_47jnLGlIMEv4jIbS--DIlkDSgFMQVnWPAIsQ2nqv-7A93PFvJt99TAu55SGGkbWlv4o49xKOGKzPM_tvcJMcgWHHInlmY-3gXKNL94mXCv7A4IUAWxIozjr4Yb8tT3hfbi9QCFuJC3fNWPKEccsW_u8u09hHiLLO_bAlR" alt="VS Code" />
                    </div>
                    <div>
                        <h3 class="hero-app-name">VS Code</h3>
                        <span class="hero-app-dev">Microsoft Corporation</span>
                    </div>
                </div>
                <h2 class="hero-title">Code anywhere, anytime.</h2>
                <p class="hero-desc">Code editing. Redefined. Free. Built on open source. Runs everywhere.</p>
                <div class="mt-auto">
                    <button class="hero-get-btn">
                        <span>Get</span>
                        <span class="material-symbols-outlined icon-sm">download</span>
                    </button>
                </div>
            </div>
        </div>
    </div>

    <!-- Essential Grid -->
    <section class="section">
        <div class="section-header">
            <h3 class="section-title">Essential Tools</h3>
            <a href="/all" class="see-all">See All</a>
        </div>
        
        {#if loading}
            <div class="loading-state">
                <p>Loading apps...</p>
            </div>
        {:else if error}
            <div class="error-state">
                <p>Failed to load apps: {error}</p>
            </div>
        {:else if filteredTools.length > 0}
            <div class="grid-4">
                {#each filteredTools as tool}
                    <AppCard 
                        name={tool.name} 
                        description={tool.description} 
                        icon={tool.icon}
                        isIconSymbol={true}
                        onClick={() => openAppDetail(tool)}
                    />
                {/each}
            </div>
        {:else}
            <div class="empty-state">
                <p>No tools found in this category.</p>
            </div>
        {/if}
    </section>

    <!-- Split Section -->
    <div class="split-layout">
        <!-- List Column -->
        <section class="section col">
            <h3 class="section-title border-b">Top Productivity</h3>
            <div class="list-container">
            {#each productivity as item}
                <div class="list-item">
                    <span class="rank">{item.rank}</span>
                    <div class="list-icon-wrapper {item.color}">
                        {#if item.isSymbol}
                            <span class="material-symbols-outlined">{item.icon}</span>
                        {:else}
                            <img src={item.icon} alt={item.name} />
                        {/if}
                    </div>
                    <div class="list-content">
                        <h4 class="list-name">{item.name}</h4>
                        <p class="list-desc">{item.desc}</p>
                    </div>
                    <button class="get-btn-small">GET</button>
                </div>
            {/each}
            </div>
        </section>
        
        <!-- New & Updated (Simplified as per design card structure) -->
         <section class="section col">
            <h3 class="section-title border-b">New & Updated</h3>
             <div class="grid-2">
                <!-- Using AppCard or similar logic, for brevity just placeholders matching design -->
                <div class="update-card">
                    <div class="flex-between">
                         <div class="update-icon blue"><span class="material-symbols-outlined">folder_zip</span></div>
                         <span class="version-tag">v24.01</span>
                    </div>
                    <div>
                        <h4 class="update-name">7-Zip</h4>
                        <p class="update-desc">High compression ratio file archiver.</p>
                    </div>
                </div>
                <div class="update-card">
                    <div class="flex-between">
                         <div class="update-icon red"><span class="material-symbols-outlined">movie</span></div>
                         <span class="version-tag">v6.1</span>
                    </div>
                    <div>
                        <h4 class="update-name">FFmpeg</h4>
                        <p class="update-desc">Complete, cross-platform solution for video.</p>
                    </div>
                </div>
             </div>
         </section>
    </div>

    <!-- Footer -->
    <div class="footer">
        <p>Scoop UI © 2024</p>
    </div>
</div>

<!-- App Detail Modal -->
<AppDetailModal 
    app={selectedApp}
    isOpen={isModalOpen}
    onClose={closeModal}
/>

<style>
    .page-container {
        max-width: 64rem; /* 5xl */
        margin: 0 auto;
        padding: 2rem;
        display: flex;
        flex-direction: column;
        gap: 2.5rem; /* 10 */
    }

    .page-header {
        display: flex;
        align-items: flex-end;
        justify-content: space-between;
        border-bottom: 1px solid var(--border);
        padding-bottom: 1rem;
    }

    .page-title {
        font-size: 1.875rem; /* 3xl */
        font-weight: 700;
        letter-spacing: -0.025em;
        margin: 0;
    }

    .categories {
        display: flex;
        gap: 0.5rem;
    }

    .cat-btn {
        height: 1.75rem;
        border-radius: 9999px;
        padding: 0 1rem;
        font-size: 0.75rem; /* xs */
        font-weight: 500;
        background: transparent;
        border: none;
        color: var(--text-muted);
        cursor: pointer;
        transition: background-color 0.15s;
    }

    .cat-btn:hover {
        background-color: #f1f5f9;
        color: var(--text-main);
    }

    .cat-btn.active {
        background-color: #f1f5f9;
        color: var(--text-main);
    }

    :global(.dark) .cat-btn:hover, :global(.dark) .cat-btn.active {
        background-color: #1e293b;
        color: #fff;
    }

    /* Hero */
    .hero-card {
        border-radius: 1rem;
        background-color: #ffffff;
        border: 1px solid var(--border);
        overflow: hidden;
        display: flex;
        flex-direction: row; /* md:flex-row */
        height: 20rem; /* Fixed height for visual consistency */
    }

    :global(.dark) .hero-card {
        background-color: var(--bg-subtle);
    }

    .hero-image {
        width: 60%;
        background-size: cover;
        background-position: center;
        position: relative;
    }

    .hero-overlay {
        position: absolute;
        inset: 0;
        background: linear-gradient(to right, transparent, rgba(0,0,0,0.1));
    }

    .hero-badge {
        position: absolute;
        top: 1rem;
        left: 1rem;
        background: rgba(255,255,255,0.9);
        padding: 0.25rem 0.5rem;
        border-radius: 0.25rem;
        font-size: 0.625rem;
        font-weight: 700;
        text-transform: uppercase;
        backdrop-filter: blur(4px);
    }

    .hero-content {
        width: 40%;
        padding: 2rem;
        display: flex;
        flex-direction: column;
        justify-content: center;
    }

    .hero-app {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        margin-bottom: 1rem;
    }

    .hero-icon {
        width: 3rem;
        height: 3rem;
        border-radius: 0.75rem;
        background-color: #f1f5f9;
        display: flex;
        align-items: center;
        justify-content: center;
        overflow: hidden;
    }
    
    .hero-icon img { width: 2rem; height: 2rem; object-fit: contain; }

    .hero-app-name {
        font-size: 1.125rem;
        font-weight: 700;
        margin: 0;
        line-height: 1.2;
    }

    .hero-app-dev {
        font-size: 0.75rem;
        color: var(--text-muted);
    }

    .hero-title {
        font-size: 1.5rem;
        font-weight: 700;
        margin: 0 0 0.5rem 0;
        line-height: 1.2;
    }

    .hero-desc {
        font-size: 0.875rem;
        color: var(--text-muted);
        margin: 0 0 1.5rem 0;
        line-height: 1.5;
    }

    .hero-get-btn {
        background-color: var(--primary);
        color: #fff;
        font-weight: 600;
        padding: 0.5rem 1.5rem;
        border-radius: 9999px;
        border: none;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        box-shadow: 0 10px 15px -3px rgba(37, 99, 235, 0.3);
        cursor: pointer;
        transition: background-color 0.15s;
    }

    .hero-get-btn:hover {
        background-color: #2563eb;
    }

    .icon-sm { font-size: 16px; }

    /* Sections */
    .section-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 1rem;
    }

    .section-title {
        font-size: 1.25rem;
        font-weight: 700;
        margin: 0;
    }

    .see-all {
        font-size: 0.875rem;
        color: var(--primary);
        text-decoration: none;
        font-weight: 500;
    }
    .see-all:hover { text-decoration: underline; }

    .grid-4 {
        display: grid;
        grid-template-columns: repeat(1, minmax(0, 1fr));
        gap: 1rem;
    }
    @media (min-width: 768px) { .grid-4 { grid-template-columns: repeat(2, minmax(0, 1fr)); } }
    @media (min-width: 1024px) { .grid-4 { grid-template-columns: repeat(4, minmax(0, 1fr)); } }

    .split-layout {
        display: grid;
        grid-template-columns: 1fr;
        gap: 2rem;
    }
    @media (min-width: 1024px) {
        .split-layout { grid-template-columns: 1fr 1fr; }
    }

    .border-b { border-bottom: 1px solid var(--border); padding-bottom: 0.5rem; margin-bottom: 1rem; }

    /* List Items */
    .list-container {
        display: flex;
        flex-direction: column;
    }

    .list-item {
        display: flex;
        align-items: center;
        gap: 1rem;
        padding: 0.75rem 0.5rem;
        border-bottom: 1px solid var(--border);
        border-radius: 0.5rem;
        transition: background-color 0.1s;
    }
    
    .list-item:hover { background-color: #f8fafc; }
    :global(.dark) .list-item:hover { background-color: rgba(30,41,59,0.5); }

    .rank { width: 1rem; text-align: center; color: var(--text-muted); font-size: 0.875rem; font-weight: 500; }

    .list-icon-wrapper {
        width: 3rem; height: 3rem; border-radius: 0.75rem;
        display: flex; align-items: center; justify-content: center;
        overflow: hidden;
    }
    .list-icon-wrapper img { width: 2rem; height: 2rem; }
    .list-icon-wrapper.purple { background-color: #f3e8ff; color: #9333ea; }
    .list-icon-wrapper.orange { background-color: #ffedd5; }
    .list-icon-wrapper.slate { background-color: #1e293b; color: #fff; }

    .list-content { flex: 1; display: flex; flex-direction: column; }
    .list-name { margin: 0; font-size: 0.875rem; font-weight: 600; }
    .list-desc { margin: 0; font-size: 0.75rem; color: var(--text-muted); }

    .get-btn-small {
        background-color: #f1f5f9; color: var(--primary); font-weight: 700;
        padding: 0.375rem 1.25rem; border-radius: 9999px; border: none; font-size: 0.75rem;
        cursor: pointer;
    }
    .list-item:hover .get-btn-small { background-color: var(--primary); color: #fff; }

    /* Updates Grid */
    .grid-2 { display: grid; grid-template-columns: 1fr 1fr; gap: 0.75rem; }
    
    .update-card {
        padding: 1rem; border-radius: 0.75rem; background-color: #f8fafc; /* slate-50 */
        border: 1px solid var(--border); display: flex; flex-direction: column; gap: 0.75rem;
        transition: background-color 0.15s; cursor: pointer;
    }
    .update-card:hover { background-color: #fff; }
    :global(.dark) .update-card { background-color: rgba(30,41,59,0.3); }
    :global(.dark) .update-card:hover { background-color: #1e293b; }

    .flex-between { display: flex; justify-content: space-between; align-items: flex-start; }
    .update-icon {
        width: 2.5rem; height: 2.5rem; border-radius: 0.5rem; background: #fff;
        display: flex; align-items: center; justify-content: center; box-shadow: 0 1px 2px 0 rgba(0,0,0,0.05);
    }
    :global(.dark) .update-icon { background: #334155; }
    .update-icon.blue { color: #3b82f6; }
    .update-icon.red { color: #ef4444; }

    .version-tag {
        font-size: 0.625rem; font-weight: 700; color: #16a34a; background-color: #dcfce7;
        padding: 0.125rem 0.5rem; border-radius: 9999px;
    }
    :global(.dark) .version-tag { background-color: rgba(20,83,45,0.3); color: #4ade80; }

    .update-name { margin: 0; font-size: 0.875rem; font-weight: 600; }
    .update-desc { margin: 0.25rem 0 0 0; font-size: 0.75rem; color: var(--text-muted); display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; }


    .footer { padding-bottom: 2.5rem; display: flex; justify-content: center; color: var(--text-muted); font-size: 0.75rem; }
    .empty-state { text-align: center; color: var(--text-muted); padding: 2rem; font-style: italic; }
    
    .loading-state, .error-state {
        text-align: center;
        padding: 3rem 2rem;
        color: var(--text-muted);
    }
    
    .error-state {
        color: #ef4444;
    }
</style>
