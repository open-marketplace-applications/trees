<script>
	import { onMount } from 'svelte';
	import { browser } from '$app/env';
	import areas from './areas';
	import { variables } from '$lib/variables';

	let trees = [];
	onMount(async () => {
		if (browser) {
			const returnValue = await fetch(variables.apiPath + `/trees`);
			console.log('returnValue', returnValue);
			const response = await returnValue.json();
			console.log('response', response);
			trees = response;

			const leaflet = await import('leaflet');

			let position = [49.291676, 7.373426];

			let sd;
			const map = leaflet.map('map').setView(position, 21);

			leaflet
				.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
					attribution:
						'© <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
				})
				.addTo(map);

			areas.forEach((area) => {
				var rect = leaflet
					.polygon(area.edges, { color: 'blue', weight: 1 })
					.on('click', function (e) {
						// There event is event object
						// there e.type === 'click'
						// there e.lanlng === L.LatLng on map
						// there e.target.getLatLngs() - your rectangle coordinates
						// but e.target !== rect
						console.info(e);
					})
					.addTo(map);
			});

			// 49.31822574308543, 7.33893700717749
			trees.forEach((tree) => {
				console.info("tree", tree)
				leaflet.marker([parseFloat(tree.lat), parseFloat(tree.lng)]).addTo(map).bindPopup(`${tree.name} - ${tree.genus}`).openPopup();
			});
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
