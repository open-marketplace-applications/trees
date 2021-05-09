<script>
	import { onMount } from 'svelte';
	import { browser } from '$app/env';

	onMount(async () => {
		if (browser) {
			const leaflet = await import('leaflet');

			let position = [49.291676, 7.373426];

			let area = [
				[49.291634, 7.373488],
				[49.291760, 7.374464],
				[49.291676, 7.374586],
				[49.291382, 7.373609]
			];
			const map = leaflet.map('map').setView(position, 21);

			leaflet
				.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
					attribution:
						'© <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
				})
				.addTo(map);

			var rect = leaflet.polygon(area, { color: 'blue', weight: 1 })
				.on('click', function (e) {
					// There event is event object
					// there e.type === 'click'
					// there e.lanlng === L.LatLng on map
					// there e.target.getLatLngs() - your rectangle coordinates
					// but e.target !== rect
					console.info(e);
				})
				.addTo(map);
			// 49.31822574308543, 7.33893700717749
			leaflet.marker([49.291582, 7.374009]).addTo(map).bindPopup('Baum #1').openPopup();
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
