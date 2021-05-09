<script>
	import Nav from '../lib/Nav.svelte';
	import '../theme.css';
</script>

<div>
	<div class="container">
		<Nav />
	</div>
	<div class="container"><slot /></div>
</div>
