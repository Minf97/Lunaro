<script lang="ts">
	import * as Popover from '$lib/components/ui/popover';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator';
	import { invoke } from '@tauri-apps/api/core';
	import { listen } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';

	let open = $state(false);
	let showClearDialog = $state(false);

	// 监听后端 Cmd+Shift+S 快捷键事件
	onMount(() => {
		const unlisten = listen('settings:open', () => {
			open = true;
		});

		return () => {
			unlisten.then((fn) => fn());
		};
	});

	// 打开壁纸目录
	async function openWallpaperDir() {
		try {
			const dir = await invoke<string>('get_wallpaper_dir');
			await invoke('open_folder', { path: dir });
		} catch (error) {
			console.error('Failed to open folder:', error);
			alert(`Failed to open folder: ${error}`);
		}
	}

	// 清除缓存
	async function clearCache() {
		showClearDialog = true;
	}

	// 确认清除缓存
	async function confirmClearCache() {
		try {
			const dir = await invoke<string>('get_wallpaper_dir');
			const result = await invoke<string>('delete_folder_contents', { path: dir });
			alert(result);
			showClearDialog = false;
			open = false;
		} catch (error) {
			console.error('Failed to clear cache:', error);
			alert(`Failed to clear cache: ${error}`);
		}
	}
</script>

<Popover.Root bind:open>
	<Popover.Trigger>
		<Button variant="ghost">设置</Button>
	</Popover.Trigger>
	<Popover.Content class="w-64">
		<div class="space-y-2">
			<h3 class="text-sm font-medium">设置</h3>

			<div class="space-y-1">
				<Button variant="ghost" class="w-full justify-start" onclick={openWallpaperDir}>
					📁 打开壁纸目录
				</Button>
				<Button variant="ghost" class="w-full justify-start" onclick={clearCache}>
					🗑️ 清除缓存
				</Button>
			</div>

			<Separator />

			<div class="space-y-2">
				<h4 class="text-muted-foreground text-xs font-medium">快捷键</h4>
				<div class="space-y-1 text-xs">
					<div class="flex items-center justify-between">
						<span>设置中心</span>
						<kbd
							class="bg-muted pointer-events-none inline-flex h-5 items-center gap-1 rounded border px-1.5 font-mono text-[10px] font-medium opacity-100 select-none"
						>
							⌘ ⇧ S
						</kbd>
					</div>
					<div class="flex items-center justify-between">
						<span>刷新列表</span>
						<kbd
							class="bg-muted pointer-events-none inline-flex h-5 items-center gap-1 rounded border px-1.5 font-mono text-[10px] font-medium opacity-100 select-none"
						>
							⌘ R
						</kbd>
					</div>
					<div class="flex items-center justify-between">
						<span>切换侧边栏</span>
						<kbd
							class="bg-muted pointer-events-none inline-flex h-5 items-center gap-1 rounded border px-1.5 font-mono text-[10px] font-medium opacity-100 select-none"
						>
							⌘ B
						</kbd>
					</div>
				</div>
			</div>
		</div>
	</Popover.Content>
</Popover.Root>

<Dialog.Root bind:open={showClearDialog}>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>清除缓存</Dialog.Title>
			<Dialog.Description>确定要清除所有缓存的壁纸吗？此操作无法撤销。</Dialog.Description>
		</Dialog.Header>
		<Dialog.Footer>
			<Button variant="outline" onclick={() => (showClearDialog = false)}>取消</Button>
			<Button variant="destructive" onclick={confirmClearCache}>确认清除</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
