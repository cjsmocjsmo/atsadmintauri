<script>
	import { page } from '$app/stores';
</script>

<header>
	<nav>
		<ul>
			
			<li aria-current={$page.url.pathname === '/estimates' ? 'estimates' : undefined}>
				<a href="/estimates">Estimates</a>
			</li>
			<li aria-current={$page.url.pathname.startsWith('/reviews') ? 'page' : undefined}>
				<a href="/reviews">Reviews</a>
			</li>
			<li aria-current={$page.url.pathname === '/' ? 'Logout' : undefined}>
				<a href="/">Logout</a>
			</li>
		</ul>
		
	</nav>
	<!-- <li>
				<button>SyncDB</button>
			</li> -->
</header>

<style>
	header {
		display: flex;
		justify-content: space-between;
	}

	nav {
		display: flex;
		align-items: space-evenly;
		justify-content: center;
		margin-right: auto;
		margin-left: auto;
		margin-top: 2.5em;
	}

	ul {
		position: relative;
		padding: 0;
		margin: 0;
		height: 3em;
		display: flex;
		justify-content: center;
		align-items: center;
		list-style: none;
		background: var(--background);
		background-size: contain;
	}

	li {
		position: relative;
		height: 100%;
	}

	nav a {
		display: flex;
		height: 100%;
		align-items: center;
		padding: 0 0.5rem;
		color: var(--color-text);
		font-weight: 700;
		font-size: 1.5rem;
		text-transform: uppercase;
		letter-spacing: 0.1em;
		text-decoration: none;
		transition: color 0.2s linear;
	}

	a:hover {
		color: var(--color-theme-1);
	}
</style>
