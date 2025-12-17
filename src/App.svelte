<script lang="ts">
	// import HelloWorld from '$lib/components/HelloWorld.svelte';
	import AppSideBar from '$lib/components/AppSideBar.svelte';
	import * as Sidebar from '$lib/components/ui/sidebar';
	import WallpaperList from '$lib/components/WallpaperList.svelte';
</script>

<main class="justify-centerp-4 flex min-h-screen items-center select-none">
	<Sidebar.Provider>
		<AppSideBar />
		<Sidebar.Trigger />
		<WallpaperList />
	</Sidebar.Provider>
	<!-- <HelloWorld /> -->
</main>
