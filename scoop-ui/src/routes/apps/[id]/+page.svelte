<script>
    import { page } from '$app/stores';

    const appId = $page.params.id;

    // Mock Data (Hardcoded for demo, ideally fetched via ID)
    const appData = {
        name: 'Google Chrome',
        desc: 'Fast, secure, and customizable browser.',
        longDesc: [
            "Get more done with the new Google Chrome. A more simple, secure, and faster web browser than ever, with Google's smarts built-in.",
            "Designed for Windows 10 and 11, Chrome brings you personalized news articles, quick links to your favorite sites, downloads, and Google Search and Google Translate built-in.",
            "Browse fast on your computer. Chrome treats every tab as a separate process, so if one crashes, the others stay open. The V8 engine processes JavaScript with lightning speed."
        ],
        icon: 'https://lh3.googleusercontent.com/aida-public/AB6AXuC9TX6iIl1Pg2qqmc8fFA2wVIZSD1zh61igthY8cvZwg7Fw1s5QREpe7qpBwBkPs6nLQwYmBdCHE7_RVHH7Ck-LUoicFVpOcDqS-_zSttljVjEh11xuz9y72dyNOad_3hQBJSQa37FNewjMqXzrrM-bFepqBmYw_ONeXRTKjkMGae4nQRJdTarH4TkSoaIaI3zSZoYCKXUHteGxGAM_RHl2Wsai5RDgY-bhtY0ygZqDiqjf2TY-7hH9M_v1CUeg7peCZ9-yFtA38MC_',
        version: '121.0.6167.140',
        updateTime: '2 days ago',
        developer: 'Google LLC',
        size: '105.4 MB',
        category: 'Utilities',
        compatibility: 'Windows 10/11',
        license: 'Freeware',
        bucket: 'extras',
        website: 'google.com',
        rating: 4.5,
        ratingCount: '12K',
        rank: '#1',
        age: '4+',
        screenshots: [
            'https://lh3.googleusercontent.com/aida-public/AB6AXuB0BFeM0ntuvbPbm4MRb21SE-MQK31ULvZkMZHHRcAT3LmmqnipXmV8r8Nd5sd9ta5NjGBxOazZiCbA6LBhXA1fZTBKZlQS-EhIrdpd3ZMgsSD6trAhy-OxzQcj5dGMRbqijNYUJAsrWGVwku25VLH3eA97Z6cPjjkEhqTNSJ5mqo2z4JVDBzkq3lH1itRGvtl3UVZYo99jxjbqLvACjaZwCrF0TPnYdwgd0kiwV2h5vsqclZ-JUYhXjxD8LRfOLvDQXF4AjKMbsJwW',
            'https://lh3.googleusercontent.com/aida-public/AB6AXuAudycdO8zEBtFNETWnddoU776yWAL97XnkJMIbR69-wwZF3uO7WR1bIAL5kb6cXM6Kh63zb4LQmyw_UBhZGCEKfPYFuEL31bO_5Q_0kTuoZixxp8SvvyobpTOzzgDGmW6inWkpCGwz2CEgyWgEZ9Q4aWJXi2lYkq77ca3mQgc6-HZ_hLL5Te6fzAyZxQvNaRf7C59Cxumq-07OFZe3RYybb1m5MX0MshqSJ1cGvvV7m8NFyPz6iBcJvi3kWvsoENCr6YD0kYqHwa8d',
            'https://lh3.googleusercontent.com/aida-public/AB6AXuCS1gFz0nqojIhy_vos4bxFl-mYi2FRmHLoWybZB7QkONK8EQNVo7uYLKgqV46qhYlYdH05JXW_gtCAr9_TKxaRbfi9ZEsO9qb6SMK08Rah0vauU6o3FIFO7G0XbrwTuJJmPsK18-gZdZLwqzRRha9cgPw8l_cSQrSlyw5SMCJBGOfGo6rrRunBD5pMtM5Se0mPCpAHpNW5hlT1GHMBpOhuS7oZWyATDMVdJ4Fyxu1Dhj9m0u-nrzOGnGMT4KZgD2Mgi1RSbh88ly3h'
        ]
    };
</script>

<div class="details-page">
    <!-- Breadcrumbs -->
    <nav class="breadcrumbs">
        <a href="/" class="crumb-link">Home</a>
        <span class="crumb-sep">/</span>
        <a href="/search" class="crumb-link">{appData.category}</a>
        <span class="crumb-sep">/</span>
        <span class="crumb-current">{appData.name}</span>
    </nav>

    <!-- Header / Hero -->
    <div class="hero">
        <div class="app-icon-large">
            <img src={appData.icon} alt={appData.name} class="icon-img" />
        </div>
        
        <div class="hero-info">
            <h1 class="hero-title">{appData.name}</h1>
            <p class="hero-desc">{appData.desc}</p>
            
            <div class="hero-actions">
                <button class="btn-install">
                    <span>INSTALL</span>
                    <span class="material-symbols-outlined icon-sm">download</span>
                </button>
                <button class="btn-share">
                    <span class="material-symbols-outlined">ios_share</span>
                </button>
            </div>
        </div>
    </div>

    <!-- Info Strip -->
    <div class="info-strip">
        <div class="strip-item">
            <div class="strip-label">4.5 Stars</div>
            <div class="strip-value large">{appData.rating}</div>
            <div class="stars">
                <span class="material-symbols-outlined star-icon">star</span>
                <span class="material-symbols-outlined star-icon">star</span>
                <span class="material-symbols-outlined star-icon">star</span>
                <span class="material-symbols-outlined star-icon">star</span>
                <span class="material-symbols-outlined star-icon">star_half</span>
            </div>
            <div class="strip-sub">{appData.ratingCount} Ratings</div>
        </div>
        <div class="strip-sep"></div>
        <div class="strip-item">
            <div class="strip-label">Rank</div>
            <div class="strip-value large">{appData.rank}</div>
            <div class="strip-sub">{appData.category}</div>
        </div>
        <div class="strip-sep"></div>
        <div class="strip-item">
            <div class="strip-label">Age</div>
            <div class="strip-value large">{appData.age}</div>
            <div class="strip-sub">Years Old</div>
        </div>
        <div class="strip-sep"></div>
        <div class="strip-item">
            <div class="strip-label">Version</div>
            <div class="strip-value large">121.0</div>
            <div class="strip-sub">Latest</div>
        </div>
        <div class="strip-sep"></div>
        <div class="strip-item">
            <div class="strip-label">Developer</div>
             <div class="dev-icon-box">
                <span class="material-symbols-outlined dev-icon">person</span>
             </div>
            <div class="strip-sub">Google LLC</div>
        </div>
    </div>

    <!-- Scoop Command -->
    <div class="command-box">
        <h3 class="command-title">
            <span class="material-symbols-outlined command-icon">terminal</span>
            Quick Install
        </h3>
        <div class="command-code-box">
            <code class="command-code">
                <span class="prompt">$</span> scoop install googlechrome
            </code>
            <button class="btn-copy" title="Copy">
                <span class="material-symbols-outlined">content_copy</span>
            </button>
        </div>
        <p class="command-note">Requires Scoop installed on Windows PowerShell.</p>
    </div>

    <!-- What's New -->
    <section class="section">
        <div class="section-header">
            <h2 class="section-title">What's New</h2>
            <button class="link-btn">Version History</button>
        </div>
        <div class="card whats-new">
            <div class="whats-new-header">
                <span class="version-tag">Version {appData.version}</span>
                <span class="update-time">{appData.updateTime}</span>
            </div>
            <p class="whats-new-text">
                This update includes important security fixes and performance improvements.
                <br/><br/>
                • <b>Energy Saver Mode:</b> Chrome now consumes less battery when your device is unplugged.<br/>
                • <b>Memory Saver:</b> Frees up memory from inactive tabs so active pages stay smooth.<br/>
                • <b>Stability:</b> Fixed several crashes related to PDF rendering.
            </p>
        </div>
    </section>

    <!-- Preview -->
    <section class="section">
        <h2 class="section-title">Preview</h2>
        <div class="screenshots-container">
            {#each appData.screenshots as shot}
                <div class="screenshot-wrapper">
                    <img src={shot} alt="Screenshot" class="screenshot" />
                </div>
            {/each}
        </div>
    </section>

    <!-- Description & Info Grid -->
    <div class="grid-layout">
        <div class="left-col">
            <h2 class="section-sub-title">Description</h2>
            <div class="description-text">
                {#each appData.longDesc as para}
                    <p>{para}</p>
                {/each}
            </div>
        </div>
        <div class="right-col">
             <h2 class="section-sub-title">Information</h2>
             <dl class="info-list">
                <div class="info-row">
                    <dt>Provider</dt>
                    <dd>{appData.developer}</dd>
                </div>
                <div class="info-row">
                    <dt>Size</dt>
                    <dd>{appData.size}</dd>
                </div>
                <div class="info-row">
                    <dt>Category</dt>
                    <dd><a href="#" class="link">{appData.category}</a></dd>
                </div>
                <div class="info-row">
                    <dt>Compatibility</dt>
                    <dd>{appData.compatibility}</dd>
                </div>
                <div class="info-row">
                    <dt>License</dt>
                    <dd>{appData.license}</dd>
                </div>
                <div class="info-row">
                    <dt>Bucket</dt>
                    <dd><span class="bucket-badge">{appData.bucket}</span></dd>
                </div>
                <div class="info-row">
                    <dt>Website</dt>
                    <dd><a href="#" class="link">{appData.website}</a></dd>
                </div>
             </dl>
        </div>
    </div>
</div>

<style>
    .details-page {
        max-width: 980px;
        margin: 0 auto;
        padding: 2rem 1.5rem 6rem 1.5rem;
    }

    /* Breadcrumbs */
    .breadcrumbs { display: flex; align-items: center; gap: 0.5rem; margin-bottom: 2rem; font-size: 0.875rem; color: var(--text-muted); }
    .crumb-link { text-decoration: none; color: inherit; }
    .crumb-link:hover { color: var(--primary); text-decoration: underline; }
    .crumb-sep { color: #cbd5e1; }
    .crumb-current { font-weight: 600; color: var(--text-main); }

    /* Hero */
    .hero { display: flex; gap: 2rem; margin-bottom: 2.5rem; align-items: flex-start; }
    @media (max-width: 768px) { .hero { flex-direction: column; } }

    .app-icon-large {
        width: 10rem; height: 10rem; flex-shrink: 0;
        border-radius: 2rem;
        background-color: #ffffff;
        box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
        overflow: hidden;
        padding: 1rem;
        border: 1px solid rgba(0,0,0,0.05);
    }
    :global(.dark) .app-icon-large { background-color: var(--surface-dark); border-color: rgba(255,255,255,0.1); }
    .icon-img { width: 100%; height: 100%; object-fit: contain; }

    .hero-info { flex: 1; display: flex; flex-direction: column; padding-top: 0.5rem; }
    .hero-title { font-size: 2.25rem; font-weight: 700; color: var(--text-main); margin: 0 0 0.5rem 0; line-height: 1.1; }
    .hero-desc { font-size: 1.125rem; color: var(--text-muted); margin: 0 0 1.5rem 0; }
    
    .hero-actions { display: flex; gap: 1rem; align-items: center; }
    .btn-install {
        background-color: var(--primary); color: white; border: none;
        border-radius: 9999px; padding: 0.625rem 2rem; font-weight: 700; font-size: 0.875rem;
        display: flex; align-items: center; gap: 0.5rem;
        box-shadow: 0 4px 6px -1px rgba(25, 127, 230, 0.3);
        cursor: pointer; transition: transform 0.1s;
    }
    .btn-install:active { transform: scale(0.95); }
    
    .btn-share {
        background-color: #f1f5f9; color: var(--primary); border: none;
        border-radius: 9999px; padding: 0.625rem; display: flex; align-items: center; justify-content: center;
        cursor: pointer;
    }
    :global(.dark) .btn-share { background-color: var(--surface-dark); }
    .btn-share:hover { background-color: #e2e8f0; }

    /* Info Strip */
    .info-strip {
        display: flex; overflow-x: auto; padding: 1rem 0;
        border-top: 1px solid var(--border); border-bottom: 1px solid var(--border);
        margin-bottom: 2.5rem;
    }
    .strip-item {
        flex: 1; display: flex; flex-direction: column; align-items: center; gap: 0.25rem; min-width: 6rem;
    }
    .strip-label { font-size: 0.75rem; font-weight: 600; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.05em; }
    .strip-value { font-weight: 700; color: #475569; }
    :global(.dark) .strip-value { color: #cbd5e1; }
    .strip-value.large { font-size: 1.25rem; }
    .strip-sub { font-size: 0.625rem; color: var(--text-muted); }
    .strip-sep { width: 1px; background-color: var(--border); }
    
    .stars { display: flex; color: #cbd5e1; }
    .star-icon { font-size: 16px; }
    .dev-icon-box { background-color: #e2e8f0; border-radius: 0.5rem; padding: 0.125rem; }
    :global(.dark) .dev-icon-box { background-color: #334155; }
    .dev-icon { font-size: 20px; color: #64748b; display: block; }

    /* Command Box */
    .command-box {
        background-color: #f8fafc; border-radius: 0.75rem; padding: 1.25rem;
        border: 1px solid var(--border); margin-bottom: 2.5rem;
    }
    :global(.dark) .command-box { background-color: var(--surface-dark); }
    .command-title { font-size: 0.875rem; font-weight: 700; margin: 0 0 0.75rem 0; display: flex; align-items: center; gap: 0.5rem; text-transform: uppercase; letter-spacing: 0.025em; }
    .command-icon { color: var(--primary); font-size: 18px; }

    .command-code-box {
        display: flex; align-items: center; gap: 0.5rem;
        background-color: #0f172a; border-radius: 0.5rem; padding: 0.75rem;
        color: #fff;
    }
    :global(.dark) .command-code-box { background-color: #000; }
    .command-code { font-family: monospace; font-size: 0.875rem; color: #4ade80; flex: 1; overflow-x: auto; }
    .prompt { color: #64748b; user-select: none; margin-right: 0.5rem; }
    .btn-copy { background: none; border: none; color: #94a3b8; cursor: pointer; padding: 0.25rem; }
    .btn-copy:hover { color: #fff; }
    
    .command-note { font-size: 0.75rem; color: var(--text-muted); margin: 0.5rem 0 0 0.25rem; }

    /* Section */
    .section { margin-bottom: 3rem; }
    .section-header { display: flex; justify-content: space-between; align-items: baseline; margin-bottom: 1rem; }
    .section-title { font-size: 1.5rem; font-weight: 700; margin: 0; }
    .link-btn { background: none; border: none; color: var(--primary); font-weight: 500; cursor: pointer; font-size: 0.875rem; }
    .link-btn:hover { text-decoration: underline; }

    .card { background-color: #ffffff; border-radius: 1rem; border: 1px solid var(--border); padding: 1.5rem; }
    :global(.dark) .card { background-color: var(--surface-dark); }

    .whats-new-header { display: flex; justify-content: space-between; margin-bottom: 0.75rem; }
    .version-tag { font-weight: 500; color: var(--text-muted); font-size: 0.875rem; }
    .update-time { color: var(--text-muted); font-size: 0.875rem; }
    .whats-new-text { font-size: 1rem; line-height: 1.6; color: #334155; }
    :global(.dark) .whats-new-text { color: #cbd5e1; }

    .screenshots-container { display: flex; gap: 1.25rem; overflow-x: auto; padding-bottom: 1rem; }
    .screenshot-wrapper {
        flex-shrink: 0; width: 400px; aspect-ratio: 16/10;
        border-radius: 0.75rem; overflow: hidden;
        border: 1px solid var(--border);
        background-color: #e2e8f0;
    }
    .screenshot { width: 100%; height: 100%; object-fit: cover; }

    /* Grid Layout */
    .grid-layout { display: grid; gap: 3rem; grid-template-columns: 1fr; }
    @media(min-width: 1024px) { .grid-layout { grid-template-columns: 2fr 1fr; } }
    
    .section-sub-title { font-size: 1.25rem; font-weight: 700; margin-bottom: 1rem; }
    .description-text p { margin-bottom: 1rem; line-height: 1.6; color: #475569; }
    :global(.dark) .description-text p { color: #cbd5e1; }

    .info-list { display: flex; flex-direction: column; gap: 1rem; font-size: 0.875rem; }
    .info-row { display: flex; justify-content: space-between; border-bottom: 1px solid var(--border); padding-bottom: 0.75rem; }
    .info-row dt { color: var(--text-muted); }
    .info-row dd { font-weight: 500; text-align: right; }
    .link { color: var(--primary); text-decoration: none; }
    .link:hover { text-decoration: underline; }
    .bucket-badge { background-color: #f1f5f9; padding: 0.125rem 0.5rem; border-radius: 0.25rem; font-family: monospace; }
    :global(.dark) .bucket-badge { background-color: #1e293b; }
</style>
