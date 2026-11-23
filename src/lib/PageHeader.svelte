<script lang="ts">
  import type { Snippet } from "svelte";

  let { title, subtitle, children, actions }: { title: string; subtitle?: Snippet; children?: Snippet; actions?: Snippet } = $props();
</script>

<header class="@container navbar min-h-0 gap-2 border-b border-base-content/10 bg-base-100/75 px-6 py-4 backdrop-blur-md">
  <div class="flex items-baseline gap-2.5">
    <h1 class="shrink-0 text-[22px] font-medium leading-8 tracking-tight">{title}</h1>
    {#if subtitle}
      <span class="truncate text-sm tabular-nums text-base-content/60">{@render subtitle()}</span>
    {/if}
  </div>
  {@render children?.()}
  {#if actions}
    <div class="ml-auto flex shrink-100 items-center gap-2 whitespace-nowrap">
      {@render actions()}
    </div>
  {/if}
</header>
