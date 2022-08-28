/* tslint:disable */
/* eslint-disable */
/**
* Generate a QR code from a string. All parameters are automatically set.
* @param {string} content
* @returns {Uint8Array}
*/
export function qr(content: string): Uint8Array;

type Shape =
  | "Square"
  | "Circle"
  | "RoundedSquare"
  | "Vertical"
  | "Horizontal"
  | "Diamond"
;

/**
 * Optionnal params to generate an SVG
 */
export type QrSvgOptions = {
  /**
   * Shape is for every module (pixel)
   * Default: "Square"
   */
  shape: Shape;
  /**
   * A QRCode needs margins of at least 4 in order to be read.
   * If QRCode is not readable, try increasing margins
   * Default: 4
   */
  margin: number;
  /**
   * The color of the background of the SVG, any string will be used as if valid
   * Default: "#FFF"
   */
  background_color: string;
  /**
   * The color of every module (pixel), any string will be used as if valid
   * Default: "#000"
   */
  module_color: string;
};

/**
* Generate a QR code from a string. All parameters are automatically set.
* @param {string} content
* @returns {Uint8Array}
*/
export function qr_svg(content: string, options?: QrSvgOptions): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly qr: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* Synchronously compiles the given `bytes` and instantiates the WebAssembly module.
*
* @param {BufferSource} bytes
*
* @returns {InitOutput}
*/
export function initSync(bytes: BufferSource): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
