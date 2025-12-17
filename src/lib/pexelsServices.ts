const PEXELS_API_KEY = import.meta.env.VITE_PEXELS_API_KEY;
const BASE_URL = 'https://api.pexels.com/v1';

export interface PexelsPhotoSrc {
	original: string;
	large2x: string;
	large: string;
	medium: string;
	small: string;
	portrait: string;
	landscape: string;
	tiny: string;
}

export interface PexelsPhoto {
	id: number;
	width: number;
	height: number;
	url: string;
	photographer: string;
	photographer_url: string;
	photographer_id: number;
	avg_color: string;
	src: PexelsPhotoSrc;
	liked: boolean;
	alt: string;
}

export interface PexelsCuratedResponse {
	page: number;
	per_page: number;
	photos: PexelsPhoto[];
	total_results: number;
	next_page?: string;
	prev_page?: string;
}

export type PexelsSearchResponse = PexelsCuratedResponse;

export async function fetchCuratedWallpapers(
	page = 1,
	perPage = 30
): Promise<PexelsCuratedResponse> {
	const response = await fetch(`${BASE_URL}/curated?page=${page}&per_page=${perPage}`, {
		headers: {
			Authorization: PEXELS_API_KEY
		}
	});

	return (await response.json()) as PexelsCuratedResponse;
}

export async function fetchSearchWallpapers(
	query: string,
	page = 1,
	perPage = 30
): Promise<PexelsSearchResponse> {
	const response = await fetch(
		`${BASE_URL}/search?query=${encodeURIComponent(query)}&page=${page}&per_page=${perPage}`,
		{
			headers: {
				Authorization: PEXELS_API_KEY
			}
		}
	);

	return (await response.json()) as PexelsSearchResponse;
}
