<script context="module">

	let current_tree
	/**
	 * @type {import('@sveltejs/kit').Load}
	 */
	export async function load({ page, fetch, session, context }) {
		const url = `http://localhost:5000/trees/${page.params.slug}`;
        console.log("url", url)
		const res = await fetch(url);
        console.log("res", res)

		if (res.ok) {

			current_tree = await res.json()
			console.log("current_tree", current_tree)
			return {
				props: {
					tree: current_tree
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
{#if current_tree}
<p>id: {current_tree.id}</p>
<p>name: {current_tree.name}</p>
<p>genus: {current_tree.genus}</p>
{/if}