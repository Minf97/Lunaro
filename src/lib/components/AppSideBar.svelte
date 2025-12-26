<script lang="ts">
	import * as Sidebar from '$lib/components/ui/sidebar';
	import Search from '$lib/components/Search.svelte';
	import SettingsPopover from '$lib/components/SettingsPopover.svelte';
</script>

<Sidebar.Root>
	<Sidebar.Header class="px-2">
		<Search />
	</Sidebar.Header>
	<Sidebar.Content class="px-2 py-1"></Sidebar.Content>
	<Sidebar.Footer>
		<SettingsPopover />
	</Sidebar.Footer>
</Sidebar.Root>
