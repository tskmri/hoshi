<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { Copy, Minus, Square, X } from "@lucide/svelte";
  import icon from "../../src-tauri/icons/32x32.png";
  import { isMac } from "./platform";

  let { title = "Hoshi" }: { title?: string } = $props();

  const win = getCurrentWindow();
  let maximized = $state(false);
  let fullscreen = $state(false);

  type ResizeEdge =
    | "East"
    | "North"
    | "NorthEast"
    | "NorthWest"
    | "South"
    | "SouthEast"
    | "SouthWest"
    | "West";

  const edges: [ResizeEdge, string][] = [
    ["North", "inset-x-0 top-0 h-1 cursor-ns-resize"],
    ["South", "inset-x-0 bottom-0 h-1 cursor-ns-resize"],
    ["West", "inset-y-0 left-0 w-1 cursor-ew-resize"],
    ["East", "inset-y-0 right-0 w-1 cursor-ew-resize"],
    ["NorthWest", "left-0 top-0 size-2 cursor-nwse-resize"],
    ["NorthEast", "right-0 top-0 size-2 cursor-nesw-resize"],
    ["SouthWest", "bottom-0 left-0 size-2 cursor-nesw-resize"],
    ["SouthEast", "bottom-0 right-0 size-2 cursor-nwse-resize"],
  ];

  async function refreshWindowState() {
    try {
      maximized = await win.isMaximized();
      fullscreen = await win.isFullscreen();
    } catch {}
  }

  $effect(() => {
    refreshWindowState();
    const unlisten = win.onResized(() => refreshWindowState());
    return () => {
      unlisten.then((fn) => fn());
    };
  });
</script>

{#if !fullscreen}
  <div
    class="relative flex h-10 shrink-0 select-none items-center bg-base-100 text-base-content"
    data-tauri-drag-region
  >
    {#if isMac}
      <div class="flex shrink-0 items-center gap-2 pl-4 pr-2">
        <button
          type="button"
          class="size-3 rounded-full bg-[#ff5f57] ring-1 ring-black/10 transition hover:brightness-110"
          title="Close"
          aria-label="Close"
          onclick={() => win.close()}
        ></button>
        <button
          type="button"
          class="size-3 rounded-full bg-[#febc2e] ring-1 ring-black/10 transition hover:brightness-110"
          title="Minimize"
          aria-label="Minimize"
          onclick={() => win.minimize()}
        ></button>
        <button
          type="button"
          class="size-3 rounded-full bg-[#28c840] ring-1 ring-black/10 transition hover:brightness-110"
          title={maximized ? "Restore" : "Maximize"}
          aria-label={maximized ? "Restore" : "Maximize"}
          onclick={() => win.toggleMaximize()}
        ></button>
      </div>
      <div class="flex min-w-0 flex-1 items-center gap-2 px-2" data-tauri-drag-region>
        <img src={icon} alt="" class="size-[18px] shrink-0 rounded-[4px]" draggable="false" />
        <span class="truncate text-xs font-medium text-base-content/70" data-tauri-drag-region>
          {title}
        </span>
      </div>
      <div class="w-4 shrink-0"></div>
    {:else}
      <div class="flex min-w-0 flex-1 items-center gap-2 pl-3" data-tauri-drag-region>
        <img src={icon} alt="" class="size-[18px] shrink-0 rounded-[4px]" draggable="false" />
        <span class="truncate text-xs font-medium text-base-content/70" data-tauri-drag-region>
          {title}
        </span>
      </div>
      <div class="flex h-full shrink-0 items-stretch">
        <button
          type="button"
          class="flex w-11 items-center justify-center text-base-content/60 transition-colors hover:bg-base-300 hover:text-base-content"
          title="Minimize"
          aria-label="Minimize"
          onclick={() => win.minimize()}
        >
          <Minus class="size-4" />
        </button>
        <button
          type="button"
          class="flex w-11 items-center justify-center text-base-content/60 transition-colors hover:bg-base-300 hover:text-base-content"
          title={maximized ? "Restore" : "Maximize"}
          aria-label={maximized ? "Restore" : "Maximize"}
          onclick={() => win.toggleMaximize()}
        >
          {#if maximized}
            <Copy class="size-3.5" />
          {:else}
            <Square class="size-3.5" />
          {/if}
        </button>
        <button
          type="button"
          class="flex w-11 items-center justify-center text-base-content/60 transition-colors hover:bg-[#e81123] hover:text-white"
          title="Close"
          aria-label="Close"
          onclick={() => win.close()}
        >
          <X class="size-4" />
        </button>
      </div>
    {/if}
  </div>

  <div class="pointer-events-none fixed inset-0 z-40">
    {#each edges as [direction, className] (direction)}
      <div
        class="pointer-events-auto absolute {className}"
        role="presentation"
        onpointerdown={(event) => {
          if (event.button !== 0) return;
          event.preventDefault();
          win.startResizeDragging(direction);
        }}
      ></div>
    {/each}
  </div>
{/if}
