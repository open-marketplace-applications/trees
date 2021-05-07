<script>
	import LeafletMap from '$lib/LeafletMap.svelte';
	import { createLoadObserver } from '../lib/util.js';
	import { browser } from '$app/env';

    console.log('loaded??');
	const onload = createLoadObserver(() => {
		console.log('loaded!!!');

		if (browser) {
			console.log('navigator: ', navigator);
			console.log('navigator geolocation: ', navigator.geolocation);
			if (navigator.geolocation) {
				console.log('navigator geolocation: ', navigator.geolocation);
				navigator.geolocation.getCurrentPosition((position) => {
					console.log('Geolocation latitude: ', position.coords.latitude);
					console.log('Geolocation longitude: ', position.coords.longitude);
				});
			} else {
				console.log('Geolocation is not supported by this browser.');
			}
		}
	});
</script>

<main>
	<LeafletMap />
</main>
