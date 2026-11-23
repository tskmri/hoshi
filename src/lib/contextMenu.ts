export type MenuEntry =
  | { kind: "separator" }
  | {
      kind: "item";
      text: string;
      disabled?: boolean;
      danger?: boolean;
      action?: () => unknown;
      children?: MenuEntry[];
    };
