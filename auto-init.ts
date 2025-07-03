// @ts-ignore
import codeSnapWasmModule from "./code_snap_bg.wasm";
// @ts-ignore
import { codesnap, initSync } from "./code_snap";

initSync({ module: codeSnapWasmModule });
export { codesnap };
