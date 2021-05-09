<script>
	import { user } from './stores.js'
  import { goto } from '$app/navigation';
  import { fly } from 'svelte/transition';
	import { ToggleMode, Container, Button, Logo } from '../design-system/index';
	import UserMenu from './UserMenu.svelte'

  const signOut = () => {
    goto('/signin')
    user.set({ name: '', avatarUrl: '' })
    console.log(user)
  }

</script>

<div class="nav">
	<Container class="container">
    <div class="left">
      <Logo />
    </div>
		<div class="menu">
			<Button link="/">Map</Button>
			<Button link="/trees">Trees</Button>
			<Button link="/create">Add Tree</Button>
		</div>
    <div class="right">
      {#if $user.name === ''}
        <Button color='secondary' link="/signin">Sign in</Button>
      {:else}
        <Button type="button" onClick={signOut}>Sign out</Button>
      {/if}
      <ToggleMode />
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