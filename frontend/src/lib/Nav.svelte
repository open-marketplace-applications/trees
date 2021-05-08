<script>
	import Button, { Label } from '@smui/button';
	let lightTheme =
		typeof window === 'undefined' || window.matchMedia('(prefers-color-scheme: light)').matches;
	function switchTheme() {
		lightTheme = !lightTheme;
		let themeLink = document.head.querySelector('#theme');
		if (!themeLink) {
			themeLink = document.createElement('link');
			themeLink.rel = 'stylesheet';
			themeLink.id = 'theme';
		}
		themeLink.href = `/smui${lightTheme ? '' : '-dark'}.css`;
		document.head
			.querySelector('link[href="/smui-dark.css"]')
			.insertAdjacentElement('afterend', themeLink);
	}
</script>

<div class="nav">
	<a href="/">Home</a>
	<a href="/create">Add Tree</a>
	<a href="/trees">Tree List</a>
	<Button style="position: right;" on:click={switchTheme}>
		<Label>{lightTheme ? 'Lights off' : 'Lights on'}</Label>
	</Button>
</div>
