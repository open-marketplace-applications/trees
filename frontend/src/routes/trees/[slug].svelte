<script context="module">
	/**
	 * @type {import('@sveltejs/kit').Load}
	 */
	export async function load({ page, fetch, session, context }) {
		const url = `http://localhost:5000/trees/${page.params.slug}`;
        console.log("url", url)
		const res = await fetch(url);
        console.log("res", res)

		if (res.ok) {
			return {
				props: {
					tree: await res.json()
				}
			};
		}

		return {
			status: res.status,
			error: new Error(`Could not load ${url}`)
		};
	}
</script>

<h1>Tree Details</h1>
