<script lang="ts">
	// import HelloWorld from '$lib/components/HelloWorld.svelte';
	import AppSideBar from '$lib/components/AppSideBar.svelte';
	import * as Sidebar from '$lib/components/ui/sidebar';
</script>

<main
	class="flex min-h-screen select-none items-center justify-center bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500 p-4"
>
	<Sidebar.Provider>
		<AppSideBar />
		<Sidebar.Trigger />
	</Sidebar.Provider>
	<!-- <HelloWorld /> -->
</main>
