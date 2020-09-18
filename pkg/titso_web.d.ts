/* tslint:disable */
/* eslint-disable */
/**
*/
export function start(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly start: () => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core_63dfd7407e7118b6___ops__function__FnMut_____Output______as_wasm_bindgen_6632896c218b51ea___closure__WasmClosure___describe__invoke______: (a: number, b: number) => void;
  readonly _dyn_for__a__core_63dfd7407e7118b6___ops__function__FnMut____a______Output______as_wasm_bindgen_6632896c218b51ea___closure__WasmClosure___describe__invoke___web_sys_c2c02385922442f2___features__gen_Event__Event_____: (a: number, b: number, c: number) => void;
  readonly _dyn_core_63dfd7407e7118b6___ops__function__FnMut_______Output______as_wasm_bindgen_6632896c218b51ea___closure__WasmClosure___describe__invoke___wasm_bindgen_6632896c218b51ea___JsValue_____: (a: number, b: number, c: number) => void;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        