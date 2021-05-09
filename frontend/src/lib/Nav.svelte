<script>
	import { user } from './stores.js'

	import { ToggleMode, Container, Button, Logo } from '../design-system/index';
	import UserMenu from './UserMenu.svelte'
</script>

<div class="nav">
	<Container class="container">
    <div class="left">
      <Logo />
    </div>
		<div class="menu">
			<Button link="/">Home</Button>
			<Button link="/create">Add Tree</Button>
		</div>
    <div class="right">
      <ToggleMode />
      {#if $user.name === ''}
        <Button class="cta-btn" link="/signin">Sign in</Button>
      {:else}
        <Button link="/">Sign out</Button>
      {/if}
    </div>
  </Container>
	
</div>

<style>
  .nav {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: var(--border);
    background-color: var(--color-layout-50);
    z-index: 1;
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
  }

  .menu {
    display: flex;
    position: absolute;
    left: 50%;
    transform: translateX(-50%);
  }

  .menu .btn {
    margin: 0 10px !important;
  }

  .left {
    width: 50px;
  }

  .nav :global(.container, .right) {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
</style>