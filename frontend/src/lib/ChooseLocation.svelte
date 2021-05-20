<script>
	import { onMount } from 'svelte';
	import { browser } from '$app/env';
	import areas from './areas';

    export let position;
    export let onChange;
	let center_marker = null;
	onMount(async () => {
		if (browser) {
			const leaflet = await import('leaflet');

			// let position = [49.291676, 7.373426];

			const map = leaflet
				.map('map')
				.setView(position, 21)
				.on('moveend', function (e) {
					// There event is event object
					// there e.type === 'click'
					// there e.lanlng === L.LatLng on map
					// there e.target.getLatLngs() - your rectangle coordinates
					// but e.target !== rect
                    map.removeLayer(center_marker)
                    position = map.getCenter()
					center_marker = leaflet
						.marker(position)
						.on('click', function (e) {
							// There event is event object
							// there e.type === 'click'
							// there e.lanlng === L.LatLng on map
							// there e.target.getLatLngs() - your rectangle coordinates
							// but e.target !== rect
							// console.info(e);
						})
						.addTo(map)
						.bindPopup('Setze deinen Baum hier!')
						.openPopup();
                    // update parent
                    // onChange(e.target.value)

				});

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

			// Show current tree position
			center_marker = leaflet
				.marker(position)
				.on('click', function (e) {
					// There event is event object
					// there e.type === 'click'
					// there e.lanlng === L.LatLng on map
					// there e.target.getLatLngs() - your rectangle coordinates
					// but e.target !== rect
				})
				.addTo(map)
				.bindPopup('Setze deinen Baum hier!')
				.openPopup();
		}
	});
</script>

<div id="map" />

<style>
	@import 'https://unpkg.com/leaflet@1.7.1/dist/leaflet.css';
	#map {
		height: 240px;
	}
</style>
