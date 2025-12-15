<script lang="ts">
	import * as InputGroup from '$lib/components/ui/input-group';
	import * as Command from '$lib/components/ui/command';
	import SearchIcon from '@lucide/svelte/icons/search';

    let value = $state("");
    $inspect(value);
</script>

<div class="grid w-full max-w-sm gap-6">
	<Command.Root class="border rounded-lg">
		<!-- <InputGroup.Root>
			<InputGroup.Input placeholder="✨看看最近最热" class="!border-none" />
			<InputGroup.Addon>
				<SearchIcon />
			</InputGroup.Addon>
		</InputGroup.Root> -->
		<Command.Input placeholder="✨看看最近最热" bind:value={value} />

		<!-- 联想 -->
		{#if value}
			<Command.List>
				<Command.Empty>No results found.</Command.Empty>
			</Command.List>
		{/if}
	</Command.Root>
</div>
