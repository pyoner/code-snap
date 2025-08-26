import type { SnapshotConfig } from "./types";
// @ts-ignore
import initWasm, { codesnap as fn } from "./code_snap";

await initWasm();

export type Config = Partial<SnapshotConfig>;

export type ImageData = {
  width: number;
  height: number;
  readonly data: Uint8Array;
  free: () => void;
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
