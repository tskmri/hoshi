<script lang="ts">
  import { ChevronRight } from "@lucide/svelte";
  import type { MenuEntry } from "./contextMenu";

  let {
    x,
    y,
    entries,
    onclose,
  }: {
    x: number;
    y: number;
    entries: MenuEntry[];
    onclose: () => void;
  } = $props();

  let root = $state<HTMLDivElement | null>(null);
  let placement = $state("left: 0; top: 0");
  let ready = $state(false);
  let openIndex = $state<number | null>(null);
  let openLeft = $state(false);

  $effect(() => {
    if (!root) return;
    const rect = root.getBoundingClientRect();
    const left = Math.max(8, Math.min(x, window.innerWidth - rect.width - 8));
    const top = Math.max(8, Math.min(y, window.innerHeight - rect.height - 8));
    placement = `left: ${left}px; top: ${top}px`;
    openLeft = x + rect.width * 2 + 16 > window.innerWidth;
    ready = true;
    root.focus({ preventScroll: true });
  });

  async function run(entry: Extract<MenuEntry, { kind: "item" }>) {
    if (entry.disabled || !entry.action) return;
    onclose();
    await entry.action();
  }

  const row =
    "flex w-full items-center justify-between gap-6 rounded-xl px-3 py-1.5 text-left text-sm text-base-content/85 transition-colors hover:bg-base-content/5 hover:text-base-content disabled:pointer-events-none disabled:opacity-40";
</script>

<svelte:window
  onpointerdowncapture={(event) => {
    if (root && !root.contains(event.target as Node)) onclose();
  }}
  onkeydowncapture={(event) => {
    if (event.key === "Escape") {
      event.preventDefault();
      onclose();
    }
  }}
  onblur={onclose}
  onresize={onclose}
  onscrollcapture={onclose}
/>

<div
  bind:this={root}
  role="menu"
  tabindex="-1"
  class="fixed z-50 min-w-56 rounded-2xl border border-base-content/10 bg-base-200/95 p-1.5 shadow-2xl shadow-black/40 backdrop-blur-md"
  style="{placement}; visibility: {ready ? 'visible' : 'hidden'}"
>
  {#each entries as entry, index (index)}
    {#if entry.kind === "separator"}
      <div class="my-1 h-px bg-base-content/10"></div>
    {:else}
      <div
        class="relative"
        role="none"
        onmouseenter={() => (openIndex = index)}
        onmouseleave={() => (openIndex = null)}
      >
        <button
          type="button"
          role="menuitem"
          class="{row} {entry.danger ? 'text-error/90 hover:bg-error/10 hover:text-error' : ''}"
          disabled={entry.disabled}
          onclick={() => run(entry)}
        >
          <span class="truncate">{entry.text}</span>
          {#if entry.children}
            <ChevronRight class="size-3.5 shrink-0 text-base-content/40" />
          {/if}
        </button>

        {#if entry.children && openIndex === index}
          <div class="absolute top-0 {openLeft ? 'right-full pr-1' : 'left-full pl-1'}">
            <div
              role="menu"
              class="min-w-48 rounded-2xl border border-base-content/10 bg-base-200/95 p-1.5 shadow-2xl shadow-black/40 backdrop-blur-md"
            >
              {#each entry.children as child, childIndex (childIndex)}
                {#if child.kind === "separator"}
                  <div class="my-1 h-px bg-base-content/10"></div>
                {:else}
                  <button
                    type="button"
                    role="menuitem"
                    class="{row} {child.danger ? 'text-error/90 hover:bg-error/10 hover:text-error' : ''}"
                    disabled={child.disabled}
                    onclick={() => run(child)}
                  >
                    <span class="truncate">{child.text}</span>
                  </button>
                {/if}
              {/each}
            </div>
          </div>
        {/if}
      </div>
    {/if}
  {/each}
</div>
