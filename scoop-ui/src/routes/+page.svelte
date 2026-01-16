<script>
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';
    import AppCard from '$lib/components/AppCard.svelte';
    import AppDetailModal from '$lib/components/AppDetailModal.svelte';

    const categories = ['All', 'Productivity', 'Development', 'Utilities', 'Design'];
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
    
    // --- Aesthetic Helpers ---
    function getAppInitials(name) {
        if (!name) return '?';
        return name.substring(0, 2).toUpperCase();
    }

    function getAppColor(name) {
        if (!name) return '#64748b';
        let hash = 0;
        for (let i = 0; i < name.length; i++) {
            hash = name.charCodeAt(i) + ((hash << 5) - hash);
        }
        const h = Math.abs(hash) % 360;
        return `hsl(${h}, 70%, 45%)`;
    }
    // -------------------------

    onMount(async () => {
        try {
            // Load 8 random apps
            essentialTools = await invoke('get_random_apps', { count: 8 });
            // Add aesthetic props if missing
            essentialTools = essentialTools.map(t => ({
                ...t,
                bgColor: getAppColor(t.name),
                initials: getAppInitials(t.name)
            }));
            loading = false;
        } catch (e) {
            console.error('Failed to load apps:', e);
            error = e;
            loading = false;
        }
    });

    const productivity = [
        { rank: 1, name: 'Obsidian', desc: 'Knowledge base', icon: 'note_alt', color: '#9333ea', bg: '#f3e8ff', isSymbol: true },
        { rank: 2, name: 'Firefox', desc: 'Web Browser', icon: 'https://lh3.googleusercontent.com/aida-public/AB6AXuAl3bjrG3Jnv__sgjlYL4bKErg0AItHEBlr7isceRW1nj0Ur2NS99fRQQP38_b_YqF6ahIe3_VF4nqxG7tche0r61dyRb_WeJCHjC7YfBvUu-W7X5Up5lXjt3DST99mpop9NjolsYHMTd-It53Q8r0NpwOTkFqHj_5Pt1UO7BX7g3HsIwTWid6AmjPcgwH4keskpcAYZkU_kvTHx0RQfU9FUFyNow1VfzlG02IVu_5tw2hIn6l4v2BNqH4aQ0Z6nuv_N6d-1c7NMVCA', isSymbol: false, color: '#f97316', bg: '#ffedd5' },
        { rank: 3, name: 'Windows Terminal', desc: 'System Tool', icon: 'terminal', isSymbol: true, color: '#0f172a', bg: '#f1f5f9' }
    ];

    // Reactive filtering
    $: filteredTools = selectedCategory === 'All' 
        ? essentialTools 
        : essentialTools; // Placeholder regex/logic for mockup
</script>

<div class="page-container">
    <!-- Header -->
    <header class="page-header">
        <div class="header-left">
            <h2 class="page-title">Discover</h2>
            <p class="page-subtitle">Find the best tools for Windows</p>
        </div>
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
            <div class="hero-content">
                <div class="hero-badge">Editor's Choice</div>
                <h2 class="hero-title">Code anywhere, anytime.</h2>
                <p class="hero-desc">Visual Studio Code is a code editor redefined and optimized for building and debugging modern web and cloud applications.</p>
                
                <div class="hero-app-mini">
                    <div class="hero-mini-icon">
                        <img src="https://upload.wikimedia.org/wikipedia/commons/9/9a/Visual_Studio_Code_1.35_icon.svg" alt="VS Code" />
                    </div>
                    <div>
                        <h3 class="hero-mini-name">Visual Studio Code</h3>
                        <span class="hero-mini-dev">Microsoft Corporation • Free</span>
                    </div>
                    <button class="hero-get-btn">Get</button>
                </div>
            </div>
            <div class="hero-visual" style="background-image: url('https://code.visualstudio.com/assets/home/home-screenshot-win.png')"></div>
            <div class="hero-overlay-gradient"></div>
        </div>
    </div>

    <!-- Essential Grid -->
    <section class="section">
        <div class="section-header">
            <h3 class="section-title">Essential Tools</h3>
        </div>
        
        {#if loading}
            <div class="loading-state">
                <div class="spinner"></div>
            </div>
        {:else if error}
            <div class="error-state">
                <p>Failed to load apps: {error}</p>
            </div>
        {:else if filteredTools.length > 0}
            <div class="grid-4">
                {#each filteredTools as tool}
                    <!-- Inline Card Style -->
                    <div class="app-card" on:click={() => openAppDetail(tool)}>
                        <div class="app-card-icon" style="background-color: {tool.bgColor}">
                            {#if tool.icon && tool.icon.startsWith('http')}
                                <img src={tool.icon} alt={tool.name} />
                            {:else}
                                <span>{tool.initials}</span>
                            {/if}
                        </div>
                        <div class="app-card-content">
                            <h4 class="app-card-title">{tool.name}</h4>
                            <p class="app-card-desc">{tool.description}</p>
                        </div>
                        <div class="app-card-footer">
                            <span class="app-tag">{tool.version}</span>
                        </div>
                    </div>
                {/each}
            </div>
        {:else}
            <div class="empty-state">
                <p>No tools found.</p>
            </div>
        {/if}
    </section>

    <!-- Split Section -->
    <div class="split-layout">
        <!-- List Column -->
        <section class="section col">
            <h3 class="section-title mb-4">Top Productivity</h3>
            <div class="rich-list">
            {#each productivity as item}
                <div class="list-item">
                    <span class="rank">{item.rank}</span>
                    <div class="list-icon" style="background-color: {item.bg}; color: {item.color}">
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
                    <button class="btn-icon-soft">
                        <span class="material-symbols-outlined">download</span>
                    </button>
                </div>
            {/each}
            </div>
        </section>
        
        <!-- New & Updated Card -->
         <section class="section col">
            <h3 class="section-title mb-4">New & Updated</h3>
             <div class="grid-1">
                <div class="update-card">
                    <div class="update-header">
                        <div class="update-icon-circle blue">
                            <span class="material-symbols-outlined">folder_zip</span>
                        </div>
                        <div>
                             <h4 class="update-name">7-Zip</h4>
                             <p class="update-meta">Updated today • v24.01</p>
                        </div>
                        <button class="btn-get-sm">Get</button>
                    </div>
                    <p class="update-desc">7-Zip is a file archiver with a high compression ratio.</p>
                </div>
                
                <div class="update-card">
                    <div class="update-header">
                         <div class="update-icon-circle red">
                            <span class="material-symbols-outlined">movie</span>
                        </div>
                        <div>
                             <h4 class="update-name">FFmpeg</h4>
                             <p class="update-meta">Updated yesterday • v6.1</p>
                        </div>
                        <button class="btn-get-sm">Get</button>
                    </div>
                    <p class="update-desc">A complete, cross-platform solution to record, convert and stream audio and video.</p>
                </div>
             </div>
         </section>
    </div>

    <!-- Footer -->
    <div class="footer">
        <p>Scoop UI © 2026</p>
    </div>
</div>

<!-- App Detail Modal -->
<AppDetailModal 
    app={selectedApp}
    isOpen={isModalOpen}
    onClose={closeModal}
/>

<style>
    :root {
        --primary: #2563eb;
        --text-main: #0f172a;
        --text-muted: #64748b;
        --bg-card: #ffffff;
        --border: #e2e8f0;
    }

    .page-container {
        max-width: 1200px;
        margin: 0 auto;
        padding: 2rem;
        display: flex;
        flex-direction: column;
        gap: 3rem;
    }

    /* Header */
    .page-header {
        display: flex;
        justify-content: space-between;
        align-items: flex-end;
        padding-bottom: 1.5rem;
        border-bottom: 1px solid var(--border);
    }
    
    .page-title {
        font-size: 2rem;
        font-weight: 800;
        margin: 0;
        color: var(--text-main);
        letter-spacing: -0.03em;
    }
    
    .page-subtitle {
        margin: 0.25rem 0 0 0;
        color: var(--text-muted);
        font-size: 0.875rem;
    }

    .categories {
        display: flex;
        gap: 0.5rem;
    }

    .cat-btn {
        height: 2rem;
        padding: 0 1rem;
        border-radius: 999px;
        background: transparent;
        border: 1px solid transparent;
        color: var(--text-muted);
        font-weight: 600;
        font-size: 0.875rem;
        cursor: pointer;
        transition: all 0.2s;
    }

    .cat-btn:hover {
        background-color: #f1f5f9;
        color: var(--text-main);
    }

    .cat-btn.active {
        background-color: #0f172a;
        color: white;
    }

    /* Hero */
    .hero-section {
        width: 100%;
        margin-bottom: 3rem; /* Add spacing to prevent overlap */
    }

    .hero-card {
        border-radius: 1.5rem;
        background-color: #0f172a;
        color: white;
        overflow: hidden;
        display: flex;
        position: relative;
        min-height: 24rem;
        box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 8px 10px -6px rgba(0, 0, 0, 0.1);
    }
    
    .hero-content {
        width: 50%;
        padding: 3rem;
        z-index: 10;
        display: flex;
        flex-direction: column;
        justify-content: center;
    }
    
    .hero-badge {
        display: inline-block;
        background: linear-gradient(to right, #3b82f6, #8b5cf6);
        padding: 0.25rem 0.75rem;
        border-radius: 999px;
        font-size: 0.75rem;
        font-weight: 700;
        text-transform: uppercase;
        letter-spacing: 0.05em;
        margin-bottom: 1.5rem;
        width: fit-content;
    }
    
    .hero-title {
        font-size: 3rem;
        font-weight: 800;
        line-height: 1.1;
        margin: 0 0 1rem 0;
        background: linear-gradient(to bottom right, #ffffff, #94a3b8);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
    }
    
    .hero-desc {
        font-size: 1.125rem;
        color: #cbd5e1;
        line-height: 1.6;
        margin: 0 0 2.5rem 0;
        max-width: 90%;
    }
    
    .hero-app-mini {
        background: rgba(255,255,255,0.1);
        backdrop-filter: blur(12px);
        padding: 1rem;
        border-radius: 1rem;
        display: flex;
        align-items: center;
        gap: 1rem;
        border: 1px solid rgba(255,255,255,0.1);
        width: fit-content;
    }
    
    .hero-mini-icon img {
        width: 3rem;
        height: 3rem;
    }
    
    .hero-mini-name {
        margin: 0;
        font-size: 1rem;
        font-weight: 700;
    }
    
    .hero-mini-dev {
        font-size: 0.75rem;
        color: #94a3b8;
    }
    
    .hero-get-btn {
        background-color: white;
        color: #0f172a;
        border: none;
        padding: 0.5rem 1.25rem;
        border-radius: 999px;
        font-weight: 700;
        cursor: pointer;
        margin-left: 1rem;
        transition: transform 0.1s;
    }
    
    .hero-get-btn:hover {
        transform: scale(1.05);
    }
    
    .hero-visual {
        width: 60%;
        position: absolute;
        right: 0;
        top: 0;
        bottom: 0;
        background-size: cover;
        background-position: left center;
        mask-image: linear-gradient(to right, transparent, black 20%);
        -webkit-mask-image: linear-gradient(to right, transparent, black 20%);
    }

    /* Cards Grid */
    .section-header {
        margin-bottom: 1.5rem;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }
    
    .section-title {
        font-size: 1.25rem;
        font-weight: 700;
        margin: 0;
        color: var(--text-main);
    }
    .mb-4 { margin-bottom: 1rem; }

    .grid-4 {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
        column-gap: 1.25rem;
        row-gap: 3.5rem;
    }
    
    .app-card {
        background: var(--bg-card);
        border: 1px solid var(--border);
        border-radius: 0.875rem;
        padding: 0.875rem;
        transition: all 0.2s;
        cursor: pointer;
        display: flex;
        flex-direction: column;
        gap: 0.625rem;
        height: 100%;
        min-width: 0; /* Prevent flex child overflow */
    }
    
    .app-card:hover {
        transform: translateY(-4px);
        box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
        border-color: #cbd5e1;
    }
    
    .app-card-icon {
        width: 2.5rem;
        height: 2.5rem;
        border-radius: 0.625rem;
        display: flex;
        align-items: center;
        justify-content: center;
        color: white;
        font-weight: 700;
        font-size: 0.875rem;
        box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
        flex-shrink: 0;
    }
    
    .app-card-icon img {
        width: 1.75rem;
        height: 1.75rem;
        object-fit: contain;
    }
    
    .app-card-content {
        flex: 1;
        min-width: 0; /* Prevent text overflow */
    }
    
    .app-card-title {
        margin: 0 0 0.375rem 0;
        font-size: 0.875rem;
        font-weight: 700;
        color: var(--text-main);
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }
    
    .app-card-desc {
        margin: 0;
        font-size: 0.75rem;
        color: var(--text-muted);
        display: -webkit-box;
        -webkit-line-clamp: 2;
        -webkit-box-orient: vertical;
        overflow: hidden;
        line-height: 1.4;
    }
    
    .app-card-footer {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-top: auto;
    }
    
    .app-tag {
        font-size: 0.75rem;
        color: var(--text-muted);
        background: #f1f5f9;
        padding: 0.125rem 0.5rem;
        border-radius: 0.25rem;
        font-family: monospace;
    }

    /* Lists */
    .split-layout {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 2.5rem;
    }
    
    @media (max-width: 900px) {
        .split-layout { grid-template-columns: 1fr; }
    }

    .rich-list {
        display: flex;
        flex-direction: column;
    }
    
    .list-item {
        display: flex;
        align-items: center;
        gap: 1rem;
        padding: 1rem;
        border-bottom: 1px solid var(--border);
        transition: background 0.1s;
    }
    
    .list-item:hover {
        background: #f8fafc;
    }
    
    .rank {
        font-size: 1.25rem;
        font-weight: 900;
        color: #e2e8f0;
        width: 1.5rem;
        text-align: center;
    }
    
    .list-icon {
        width: 3rem;
        height: 3rem;
        border-radius: 0.75rem;
        display: flex;
        align-items: center;
        justify-content: center;
    }
    .list-icon img { width: 1.75rem; height: 1.75rem; }
    
    .list-content {
        flex: 1;
        min-width: 0;
    }
    .list-name { margin: 0; font-weight: 700; font-size: 0.9375rem; }
    .list-desc { margin: 0; font-size: 0.75rem; color: var(--text-muted); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
    
    .btn-icon-soft {
        width: 2.25rem; height: 2.25rem;
        border-radius: 999px;
        background: #f1f5f9;
        color: var(--primary);
        border: none;
        display: flex; align-items: center; justify-content: center;
        cursor: pointer;
        transition: all 0.2s;
    }
    .btn-icon-soft:hover { background: var(--primary); color: white; }

    /* Update Cards */
    .grid-1 { display: flex; flex-direction: column; gap: 1rem; }
    
    .update-card {
        background: var(--bg-card);
        border: 1px solid var(--border);
        border-radius: 1rem;
        padding: 1.25rem;
        transition: all 0.2s;
    }
    
    .update-card:hover {
        transform: translateY(-2px);
        box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.05); /* Match app-card shadow */
        border-color: #cbd5e1;
    }
    
    .update-header {
        display: flex;
        align-items: center;
        gap: 1rem;
        margin-bottom: 0.75rem;
    }
    
    .update-icon-circle {
        width: 3rem; height: 3rem; 
        border-radius: 0.75rem; /* Match app-card-icon radius */
        display: flex; align-items: center; justify-content: center;
        background: white;
        border: 1px solid var(--border);
        flex-shrink: 0;
    }
    .update-icon-circle.blue { color: #3b82f6; background: #eff6ff; border: none; }
    .update-icon-circle.red { color: #ef4444; background: #fef2f2; border: none; }
    
    .update-name { margin: 0; font-weight: 700; font-size: 1rem; color: var(--text-main); }
    .update-meta { margin: 0; font-size: 0.75rem; color: var(--text-muted); }
    
    .update-desc {
        color: var(--text-muted);
        font-size: 0.875rem;
        margin: 0;
        line-height: 1.5;
        display: -webkit-box;
        -webkit-line-clamp: 2;
        -webkit-box-orient: vertical;
        overflow: hidden;
    }
    
    .btn-get-sm {
        margin-left: auto;
        padding: 0.375rem 1rem;
        border-radius: 999px;
        background: transparent;
        border: 1px solid var(--border);
        font-weight: 600;
        font-size: 0.75rem;
        cursor: pointer;
        transition: all 0.2s;
        color: var(--text-main);
    }
    .btn-get-sm:hover { 
        background: var(--text-main); 
        color: white; 
        border-color: var(--text-main); 
    }

    .spinner {
        width: 40px; height: 40px;
        border: 4px solid #f3f3f3;
        border-top: 4px solid var(--primary);
        border-radius: 50%;
        animation: spin 1s linear infinite;
        margin: 4rem auto;
    }
    
    @keyframes spin { 0% { transform: rotate(0deg); } 100% { transform: rotate(360deg); } }

    .footer {
        text-align: center;
        padding-top: 2rem;
        color: var(--text-muted);
        font-size: 0.75rem;
        border-top: 1px solid var(--border);
    }
</style>
