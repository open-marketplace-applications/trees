<script>
	import { onMount } from 'svelte';
	import { variables } from '$lib/variables';

    let trees = [];
	onMount(async () => {
		const returnValue = await fetch(variables.apiPath +`/trees`);
        console.log("returnValue", returnValue)
		const response = await returnValue.json();
        console.log("response", response)
        trees = response
	});
</script>

{#each trees as tree}

{#if tree}
<li><a href="/trees/new{tree.id}">
    {tree.name},  {tree.genus}
</a></li>
{/if}
{/each}


