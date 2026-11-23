<script lang="ts">
  import type { Snippet } from "svelte";

  let { title, compact = false, children }: { title: string; compact?: boolean; children: Snippet } = $props();
</script>

{#if compact}
  <section class="flex flex-col gap-3">
    <h3 class="text-[10px] font-medium uppercase tracking-[0.2em] text-base-content/45">{title}</h3>
    {@render children()}
  </section>
{:else}
  <div class="card rounded-[1.75rem] border border-base-content/10 bg-base-200/40">
    <div class="card-body gap-4">
      <h2 class="card-title text-[10px] font-medium uppercase tracking-[0.2em] text-base-content/45">{title}</h2>
      {@render children()}
    </div>
  </div>
{/if}
