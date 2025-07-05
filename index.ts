import type { SnapshotConfig } from "./types";
// @ts-ignore
import codeSnapWasmModule from "./codesnap_bg.wasm";
// @ts-ignore
import { codesnap as fn, initSync } from "./codesnap";

initSync({ module: codeSnapWasmModule });

export type Config = Partial<SnapshotConfig>;

export type ImageData = {
  width: number;
  height: number;
  readonly data: Uint8Array;
};

export const codesnap = (
  code: string,
  language: string,
  config?: Config,
): ImageData =>
  fn(
    code,
    language,
    JSON.stringify({
      theme: "candy",
      background: "#000000",
      scale_factor: 1,
      ...config,
    } satisfies Config),
  );
