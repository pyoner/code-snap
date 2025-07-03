import type { SnapshotConfig } from "./types";
// @ts-ignore
import codeSnapWasmModule from "./code_snap_bg.wasm";
// @ts-ignore
import { codesnap as fn, initSync } from "./code_snap";

initSync({ module: codeSnapWasmModule });

export type Config = Partial<SnapshotConfig>;

export const codesnap = (code: string, language: string, config?: Config) =>
  fn(
    code,
    language,
    config
      ? JSON.stringify({
          theme: "candy",
          background: "#000000",
          scale_factor: 1,
          ...config,
        } satisfies Config)
      : null,
  );
