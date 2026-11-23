<script lang="ts">
  import {
    BookOpen,
    ChartLine,
    LibraryBig,
    PanelLeftClose,
    PanelLeftOpen,
    Settings,
  } from "@lucide/svelte";
  import { saveShellConfig, shellConfig } from "./shellConfig.svelte";
  import type { AppView } from "./types";

  let {
    view,
    showToggle = true,
    onNavigate,
  }: {
    view: AppView;
    showToggle?: boolean;
    onNavigate: (view: AppView) => void;
  } = $props();

  const destinations = [
    ["books", "Books", LibraryBig],
    ["dictionary", "Dictionary", BookOpen],
    ["statistics", "Statistics", ChartLine],
  ] as const;

  // The content panel is inset by 12px (main's p-3), so the visible left
  // gutter is the rail plus that gap. Centre the icons in the whole gutter.
  const list = "flex w-full flex-col items-center gap-1 pl-3";

  const item =
    "tooltip tooltip-right flex size-11 items-center justify-center rounded-full p-0 transition-colors hover:bg-base-content/5";

  function toggleHomeBar() {
    // The sidebar is only collapsible from Books; Settings always shows it.
    if (view !== "books") return;
    shellConfig.homeBarCollapsed = !shellConfig.homeBarCollapsed;
    saveShellConfig();
  }
</script>

<aside class="flex w-18 shrink-0 flex-col">
  <ul class="{list} flex-1 pt-2">
    {#each destinations as [id, label, Icon] (id)}
      <li>
        <button
          class={item}
          data-tip={label}
          onclick={() => onNavigate(id)}
        >
          <Icon class="size-6 {view === id ? '' : 'opacity-55'}" />
        </button>
      </li>
    {/each}
  </ul>
  <ul class="{list} pb-2">
    {#if showToggle}
      <li>
        <button
          class="{item} {shellConfig.homeBarCollapsed ? '' : 'opacity-55'}"
          data-tip={shellConfig.homeBarCollapsed ? "Show Home Bar" : "Hide Home Bar"}
          onclick={toggleHomeBar}
        >
          {#if shellConfig.homeBarCollapsed}
            <PanelLeftOpen class="size-6" />
          {:else}
            <PanelLeftClose class="size-6" />
          {/if}
        </button>
      </li>
    {/if}
    <li>
      <button
        class={item}
        data-tip="Settings"
        onclick={() => onNavigate("settings")}
      >
        <Settings class="size-6 {view === 'settings' ? '' : 'opacity-55'}" />
      </button>
    </li>
  </ul>
</aside>
