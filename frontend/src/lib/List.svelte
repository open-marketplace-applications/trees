<script>
	import { onMount } from 'svelte';

    let trees = [];
	onMount(async () => {
		const returnValue = await fetch(`http://localhost:5000/trees`);
        console.log("returnValue", returnValue)
		const response = await returnValue.json();
        console.log("response", response)
        trees = response
	});
</script>

{#each trees as tree}

{#if tree}
<li><a target="_blank" href="http://localhost:5000/trees/{tree.id}">
    {tree.name},  {tree.genus}
</a></li>
{/if}
{/each}


