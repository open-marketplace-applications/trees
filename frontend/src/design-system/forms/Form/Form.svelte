<script lang="ts">
  export let onSubmit: () => void
</script>

<div class="form-container">
  <form on:submit|preventDefault={onSubmit}>
    <slot />
  </form>
</div>

<style>
  .form-container {
    max-width: 500px;
    width: 100%;
    margin: 0 auto;
    padding: 0;
    border: none;
    border-radius: var(--radius);
  }

  :global(h1) {
    margin-bottom: var(--space-lg) !important;
  }

  @media (min-width: 720px) {
    .form-container {
      border: var(--border);
      padding: var(--space-xxl);
      background-color: var(--color-layout);
    }
  }
</style>
