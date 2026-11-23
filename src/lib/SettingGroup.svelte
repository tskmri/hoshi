<script lang="ts">
  import type { Snippet } from "svelte";

  let { title = "", children }: { title?: string; children: Snippet } = $props();
</script>

<section class="flex flex-col gap-2">
  {#if title}
    <h3 class="text-[10px] font-medium uppercase tracking-[0.2em] text-base-content/45">{title}</h3>
  {/if}
  <div class="card rounded-2xl border border-base-content/10 bg-base-100/40">
    <div class="card-body gap-3 p-3">
      {@render children()}
    </div>
  </div>
</section>
