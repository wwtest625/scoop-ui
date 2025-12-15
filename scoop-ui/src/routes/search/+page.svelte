<script>
    import AppCard from '$lib/components/AppCard.svelte';
    import { page } from '$app/stores';
    import { goto } from '$app/navigation';

    const trending = ['Trending', 'VS Code', 'Google Chrome', 'NodeJS', '7zip', 'Python', 'Firefox'];
    
    // Mock data for search suggestions
    const suggestedApps = [
        { name: 'Visual Studio Code', desc: 'Code editing. Redefined.', icon: 'code', isSymbol: true, gradient: 'from-blue-500 to-purple-600', rating: 4.9, cat: 'Development' },
        { name: 'Windows Terminal', desc: 'Modern command line tools', icon: 'terminal', isSymbol: true, gradient: 'from-yellow-400 to-orange-500', rating: 4.8, cat: 'System' },
        { name: '7-Zip', desc: 'High compression ratio archiver', icon: 'folder_zip', isSymbol: true, gradient: 'from-green-500 to-emerald-700', rating: 4.7, cat: 'Utility' },
        { name: 'VLC Media Player', desc: 'Open source multimedia player', icon: 'play_circle', isSymbol: true, gradient: 'from-red-500 to-pink-600', rating: 4.9, cat: 'Video' },
        { name: 'Discord', desc: 'Talk, chat, and hang out', icon: 'webhook', isSymbol: true, gradient: 'from-indigo-500 to-purple-800', rating: 4.6, cat: 'Social' },
        { name: 'Postman', desc: 'API Platform for developers', icon: 'science', isSymbol: true, gradient: 'from-teal-400 to-cyan-600', rating: 4.8, cat: 'DevTools' }
    ];

    let query = $page.url.searchParams.get('q') || '';

    // Update input when URL changes
    $: query = $page.url.searchParams.get('q') || '';

    function handleSearch(e) {
        if (e.key === 'Enter') {
            // @ts-ignore
            const val = e.target.value.trim();
            if (val) {
                goto(`/search?q=${encodeURIComponent(val)}`);
            }
        }
    }
</script>

<div class="search-page">
    <!-- Heading -->
    <header class="header">
        <h1 class="title">Search</h1>
        <p class="subtitle">Find apps, utilities, and developer tools.</p>
    </header>

    <!-- Search Input -->
    <div class="search-box-wrapper">
        <div class="search-icon">
            <span class="material-symbols-outlined icon-lg">search</span>
        </div>
        <input 
            class="main-search-input" 
            placeholder="Games, Utilities, Development..." 
            type="text" 
            bind:value={query}
            on:keydown={handleSearch}
            autofocus
        />
    </div>

    <!-- Discovery Chips -->
    <section class="section">
        <h2 class="section-title">Discover</h2>
        <div class="chips-container">
            {#each trending as chip, i}
                <button class="chip">
                    {#if i === 0}
                        <span class="material-symbols-outlined chip-icon">trending_up</span>
                    {/if}
                    {chip}
                </button>
            {/each}
        </div>
    </section>

    <!-- Suggested Apps -->
    <section class="section">
        <div class="section-header">
            <h2 class="section-title">Suggested Apps</h2>
            <button class="see-all">See All</button>
        </div>
        
        <div class="grid-3">
            {#each suggestedApps as app}
                <!-- 
                   Note: The design has specific gradients for icon backgrounds. 
                   Our AppCard is generic, but adaptable. 
                   For now using AppCard as base but ideally we might enhance AppCard to support custom icon backgrounds 
                   or just use the default.
                   To stay true to design, I'll inline the card here or update AppCard later?
                   Actually AppCard is close enough, let's use it.
                   The design here is slightly richer (rating, category). 
                   I will stick to AppCard for reusability as per instructions to be efficient, 
                   but if user wanted pixel perfect match I would create a 'RichAppCard'.
                   Given "Project Forking" context, I'll stick to a robust AppCard.
                   Wait, I can just use the generic AppCard for now to prove structure.
                -->
                <AppCard 
                    name={app.name} 
                    description={app.desc} 
                    icon={app.icon} 
                    isIconSymbol={app.isSymbol}
                />
            {/each}
        </div>
    </section>
</div>

<style>
    .search-page {
        max-width: 1200px;
        margin: 0 auto;
        padding: 2rem 1.5rem; /* px-6 py-8 */
        display: flex;
        flex-direction: column;
        gap: 2.5rem; /* 10 */
    }

    @media (min-width: 768px) {
        .search-page { padding: 3rem 3rem; }
    }

    .header {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .title {
        font-size: 2.25rem; /* 4xl */
        font-weight: 900; /* black */
        letter-spacing: -0.05em;
        margin: 0;
    }
    @media (min-width: 768px) { .title { font-size: 3rem; } }

    .subtitle {
        font-size: 1.125rem; /* lg */
        color: var(--text-muted);
        margin: 0;
    }

    .search-box-wrapper {
        position: relative;
    }

    .search-icon {
        position: absolute;
        top: 0; bottom: 0; left: 0;
        padding-left: 1rem;
        display: flex;
        align-items: center;
        pointer-events: none;
    }
    @media (min-width: 768px) { .search-icon { padding-left: 1.5rem; } }

    .icon-lg { font-size: 28px; color: #94a3b8; }

    .main-search-input {
        width: 100%;
        height: 3.5rem; /* 14 */
        padding-left: 3.5rem;
        padding-right: 1.5rem;
        border-radius: 1rem; /* 2xl */
        background-color: #ffffff;
        border: 1px solid var(--border); /* ring-1 slate-200 */
        font-size: 1.125rem; /* lg */
        color: var(--text-main);
        outline: none;
        box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.02);
        transition: all 0.2s;
    }
    @media (min-width: 768px) {
        .main-search-input { height: 4rem; padding-left: 4rem; }
    }

    :global(.dark) .main-search-input {
        background-color: #1e2936; /* card-dark */
        border-color: #334155;
    }

    .main-search-input:focus {
        border-color: var(--primary);
        box-shadow: 0 0 0 1px var(--primary), 0 0 15px rgba(25, 127, 230, 0.3);
    }

    .section {
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .section-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .section-title {
        font-size: 1.25rem; /* xl */
        font-weight: 700;
        margin: 0;
    }

    .chips-container {
        display: flex;
        flex-wrap: wrap;
        gap: 0.75rem;
    }

    .chip {
        display: flex;
        align-items: center;
        padding: 0.625rem 1.25rem;
        border-radius: 9999px;
        background-color: #f1f5f9; /* slate-100 */
        color: #334155; /* slate-700 */
        font-weight: 500;
        font-size: 0.875rem; /* sm */
        border: none;
        cursor: pointer;
        transition: background-color 0.15s;
    }

    :global(.dark) .chip {
        background-color: #1e293b;
        color: #cbd5e1;
    }

    .chip:hover { background-color: #e2e8f0; }
    :global(.dark) .chip:hover { background-color: #334155; }

    .chip-icon { font-size: 18px; margin-right: 0.5rem; color: var(--text-muted); }

    .see-all {
        color: var(--primary);
        font-size: 0.875rem;
        font-weight: 600;
        background: none;
        border: none;
        cursor: pointer;
    }
    .see-all:hover { text-decoration: underline; }

    .grid-3 {
        display: grid;
        grid-template-columns: repeat(1, minmax(0, 1fr));
        gap: 1.25rem;
    }
    @media (min-width: 768px) { .grid-3 { grid-template-columns: repeat(2, minmax(0, 1fr)); } }
    @media (min-width: 1280px) { .grid-3 { grid-template-columns: repeat(3, minmax(0, 1fr)); } }
</style>
