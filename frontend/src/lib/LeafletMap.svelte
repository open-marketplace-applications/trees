<script>
	import { onMount } from 'svelte';
	import { browser } from '$app/env';

	onMount(async () => {
		if (browser) {
			const leaflet = await import('leaflet');

			const map = leaflet.map('map').setView([49.31, 7.33], 21);

			leaflet
				.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
					attribution:
						'© <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
				})
				.addTo(map);

            // 49.31822574308543, 7.33893700717749
			leaflet
				.marker([49.31, 7.33])
				.addTo(map)
				.bindPopup('Baum #1')
				.openPopup();
		}
	});
</script>

<main>
	<div id="map" />
</main>

<style>
	@import 'https://unpkg.com/leaflet@1.7.1/dist/leaflet.css';
	main #map {
		height: 800px;
	}
</style>
