import { readerConfig, type ThemeName } from "./readerConfig.svelte";

const media = window.matchMedia("(prefers-color-scheme: dark)");
const os = $state({ dark: media.matches });
media.addEventListener("change", (e) => {
  os.dark = e.matches;
});

function schemeOf(theme: ThemeName): "light" | "dark" | null {
  if (theme === "Light") return "light";
  if (theme === "Dark") return "dark";
  return null;
}

function customTheme() {
  return readerConfig.customThemes[readerConfig.customTheme];
}

export function chromeOverride(): "light" | "dark" | null {
  if (readerConfig.theme === "Custom") return schemeOf(customTheme().uiTheme);
  return schemeOf(readerConfig.theme);
}

export function resolvedScheme(): "light" | "dark" {
  return chromeOverride() ?? (os.dark ? "dark" : "light");
}

export function readerBackground(): string {
  if (readerConfig.theme === "Custom") return customTheme().backgroundColor;
  return resolvedScheme() === "dark" ? "#121212" : "#ffffff";
}

export function readerTextColor(): string | null {
  if (readerConfig.theme === "Custom") return customTheme().textColor;
  return resolvedScheme() === "dark" ? "#d8d9de" : null;
}

export function infoColor(): string | null {
  return readerConfig.theme === "Custom" ? customTheme().infoColor : null;
}

export function chromeTheme(): "hoshi-light" | "hoshi-dark" {
  return resolvedScheme() === "dark" ? "hoshi-dark" : "hoshi-light";
}
