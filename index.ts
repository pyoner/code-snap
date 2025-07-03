import type { SnapshotConfig } from "./types";
// @ts-ignore
import codeSnapWasmModule from "./code_snap_bg.wasm";
// @ts-ignore
import { codesnap as fn, initSync } from "./code_snap";

initSync({ module: codeSnapWasmModule });

const codesnap = (code: string, language: string, config?: SnapshotConfig) =>
  fn(code, language, config ? JSON.stringify(config) : null);
export { codesnap };
