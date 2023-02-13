/* tslint:disable */
/* eslint-disable */
/**
* Generate a QR code from a string. All parameters are automatically set.
* @param {string} content
* @returns {Uint8Array}
*/
export function qr(content: string): Uint8Array;
/**
* Generate a QR code from a string. All parameters are automatically set.
* @param {string} content
* @param {SvgOptions} options
* @returns {string}
*/
export function qr_svg(content: string, options: SvgOptions): string;
/**
* Different possible Shapes to represent modules in a QRCode
*/
export enum Shape {
/**
* Square Shape
*/
  Square,
/**
* Circle Shape
*/
  Circle,
/**
* RoundedSquare Shape
*/
  RoundedSquare,
/**
* Vertical Shape
*/
  Vertical,
/**
* Horizontal Shape
*/
  Horizontal,
/**
* Diamond Shape
*/
  Diamond,
}
/**
* Different possible image background shapes
*/
export enum ImageBackgroundShape {
/**
* Square shape
*/
  Square,
/**
* Circle shape
*/
  Circle,
/**
* Rounded square shape
*/
  RoundedSquare,
}
/**
* Configuration for the SVG output.
*/
export class SvgOptions {
  free(): void;
/**
* Updates the shape of the QRCode modules.
* @param {number} shape
* @returns {SvgOptions}
*/
  shape(shape: number): SvgOptions;
/**
* Updates the module color of the QRCode. Tales a string in the format `#RRGGBB[AA]`.
* @param {string} module_color
* @returns {SvgOptions}
*/
  module_color(module_color: string): SvgOptions;
/**
* Updates the margin of the QRCode.
* @param {number} margin
* @returns {SvgOptions}
*/
  margin(margin: number): SvgOptions;
/**
* Updates the background color of the QRCode. Tales a string in the format `#RRGGBB[AA]`.
* @param {string} background_color
* @returns {SvgOptions}
*/
  background_color(background_color: string): SvgOptions;
/**
* Updates the image of the QRCode. Takes base64 or a url.
* @param {string} image
* @returns {SvgOptions}
*/
  image(image: string): SvgOptions;
/**
* Updates the background color of the image. Takes a string in the format `#RRGGBB[AA]`.
* @param {string} image_background_color
* @returns {SvgOptions}
*/
  image_background_color(image_background_color: string): SvgOptions;
/**
* Updates the shape of the image background. Takes an convert::ImageBackgroundShape.
* @param {number} image_background_shape
* @returns {SvgOptions}
*/
  image_background_shape(image_background_shape: number): SvgOptions;
/**
* Updates the size of the image. Takes a size and a gap (unit being module size).
* @param {number} size
* @param {number} gap
* @returns {SvgOptions}
*/
  image_size(size: number, gap: number): SvgOptions;
/**
* Updates the position of the image. Takes an array [x, y] (unit being module size).
* @param {Float64Array} image_position
* @returns {SvgOptions}
*/
  image_position(image_position: Float64Array): SvgOptions;
/**
* Creates a new SvgOptions object.
*/
  constructor();
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly qr: (a: number, b: number, c: number) => void;
  readonly __wbg_svgoptions_free: (a: number) => void;
  readonly svgoptions_shape: (a: number, b: number) => number;
  readonly svgoptions_module_color: (a: number, b: number, c: number) => number;
  readonly svgoptions_margin: (a: number, b: number) => number;
  readonly svgoptions_background_color: (a: number, b: number, c: number) => number;
  readonly svgoptions_image: (a: number, b: number, c: number) => number;
  readonly svgoptions_image_background_color: (a: number, b: number, c: number) => number;
  readonly svgoptions_image_background_shape: (a: number, b: number) => number;
  readonly svgoptions_image_size: (a: number, b: number, c: number) => number;
  readonly svgoptions_image_position: (a: number, b: number, c: number) => number;
  readonly svgoptions_new: () => number;
  readonly qr_svg: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
