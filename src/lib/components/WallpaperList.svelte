<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { fetchCuratedWallpapers, type PexelsPhoto } from '$lib/pexelsServices';
	//    import { savePicture } from '$lib/db/queries';

	let wallpapers = $state<PexelsPhoto[]>([]);
	let loading = $state(true);

	$inspect(wallpapers);

	onMount(() => {
		// 初始加载壁纸
		(async () => {
			try {
				const { photos } = await fetchCuratedWallpapers(1, 30);

				// 保存到数据库
				//    for (const photo of photos) {
				//      await savePicture({
				//        pexelsId: photo.id,
				//        photographer: photo.photographer,
				//        photographerUrl: photo.photographer_url,
				//        originalUrl: photo.src.large,
				//        thumbnailUrl: photo.src.medium,
				//        width: photo.width,
				//        height: photo.height,
				//      });
				//    }

				wallpapers = photos;
			} catch (error) {
				console.error('Failed to load wallpapers:', error);
			} finally {
				loading = false;
			}
		})();

		// 监听 Cmd+R 快捷键刷新壁纸列表
		const handleKeyPress = (e: KeyboardEvent) => {
			if ((e.metaKey || e.ctrlKey) && e.key === 'r') {
				e.preventDefault();
				refreshWallpapers();
			}
		};

		window.addEventListener('keydown', handleKeyPress);

		return () => {
			window.removeEventListener('keydown', handleKeyPress);
		};
	});

	// 刷新壁纸列表
	async function refreshWallpapers() {
		loading = true;
		try {
			const { photos } = await fetchCuratedWallpapers(1, 30);
			wallpapers = photos;
		} catch (error) {
			console.error('Failed to refresh wallpapers:', error);
		} finally {
			loading = false;
		}
	}

	async function setWallpaper(imageUrl: string) {
		try {
			// 1. 获取壁纸目录
			const wallpaperDir = await invoke<string>('get_wallpaper_dir');

			// TODO: 避免重复下载，如果这个名字已经有了，那么就不下载了。
			// 从 URL 中提取文件名
			const filename = imageUrl.split('/').pop() || `wallpaper_${Date.now()}.jpg`;
			const localPath = `${wallpaperDir}/${filename}`;

			// 2. 下载图片到本地
			console.log('⬇️ Downloading image from:', imageUrl);
			const response = await fetch(imageUrl);
			if (!response.ok) {
				throw new Error(`HTTP error! status: ${response.status}`);
			}
			const blob = await response.blob();
			const arrayBuffer = await blob.arrayBuffer();
			const bytes = new Uint8Array(arrayBuffer);
			console.log('✅ Image downloaded, size:', bytes.length, 'bytes');

			// 3. 保存图片到本地
			await invoke('write_binary', { path: localPath, contents: Array.from(bytes) });

			// 4. 设置为壁纸
			const result = await invoke('set_wallpaper_macos', { imagePath: localPath });
			console.log('✅ Wallpaper set result:', result);

			// TODO: shadcn 弹窗提示成功
			alert('Wallpaper set successfully!');
		} catch (error) {
			console.error('❌ Failed to set wallpaper:', error);
			alert(`Failed to set wallpaper: ${error}`);
		}
	}
</script>

<div class="w-full">
	{#if loading}
		<p>Loading wallpapers...</p>
	{:else}
		<div class="grid">
			{#each wallpapers as wallpaper (wallpaper.id)}
				<div class="card">
					<img src={wallpaper.src.medium} alt={wallpaper.photographer} />
					<div class="info">
						<p>{wallpaper.photographer}</p>
						<button onclick={() => setWallpaper(wallpaper.src.original)}> Set as Wallpaper </button>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 1rem;
		padding: 1rem;
	}

	.card {
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
	}

	.card img {
		width: 100%;
		height: 200px;
		object-fit: cover;
	}

	.info {
		padding: 0.5rem;
	}

	button {
		margin-top: 0.5rem;
		padding: 0.5rem 1rem;
		background: #007bff;
		color: white;
		border: none;
		border-radius: 4px;
		cursor: pointer;
	}
</style>
