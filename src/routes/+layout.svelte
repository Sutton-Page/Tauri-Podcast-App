
<header class="header">
    <nav class="navbar">
        <a href="/" class="nav-link">
            <span class="icon home-icon"></span>
            Home
        </a>
        <a href="/search" class="nav-link">
            <span class="icon search-icon"></span>
            Search
        </a>
    </nav>
</header>

<slot> </slot>


<style>

:global(body) {

    color-scheme: dark;
    background-color: Canvas;
    color: CanvasText;
}

.header {
    background-color: #323232;
    padding: 10px 20px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);
}

.navbar {
    display: flex;
    gap: 20px;
}

.nav-link {
    text-decoration: none;
    color: #ffffff;
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 16px;
}

.nav-link:hover {
    color: #cccccc;
}

/* Icon Styles */
.icon {
    display: inline-block;
    width: 24px;
    height: 24px;
    background-size: cover;
}


</style>