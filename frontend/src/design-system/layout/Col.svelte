<script lang="ts">
  export let align: string = 'flex-start'
  export let direction: string = 'column'
  export let justify: string = 'flex-start'
</script>

<div
  style="align-items: {align}; justify-content: {justify}; flex-direction: {direction}"
  class={`col ${$$props.class}`}
>
  <slot />
</div>

<style>
  .col {
    flex: 1;
    width: 100%;
    min-width: 100%;
    display: flex;
  }

  @media (min-width: 620px) {
    .col {
      margin: 0 var(--space-sm);
      flex: calc(50% - 24px);
      width: auto;
      min-width: auto;
    }
  }

  @media (min-width: 980px) {
    .col {
      margin: 0 var(--space-md);
      width: auto;
      min-width: auto;
    }
  }
</style>
