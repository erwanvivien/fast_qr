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
* Different possible Shapes to represent modules in a [`crate::QRCode`]
*/
export enum Shape {
/**
* Square Shape
*/
  Square = 0,
/**
* Circle Shape
*/
  Circle = 1,
/**
* RoundedSquare Shape
*/
  RoundedSquare = 2,
/**
* Vertical Shape
*/
  Vertical = 3,
/**
* Horizontal Shape
*/
  Horizontal = 4,
/**
* Diamond Shape
*/
  Diamond = 5,
}
/**
* Different possible image background shapes
*/
export enum ImageBackgroundShape {
/**
* Square shape
*/
  Square = 0,
/**
* Circle shape
*/
  Circle = 1,
/**
* Rounded square shape
*/
  RoundedSquare = 2,
}
/**
* Error Correction Coding has 4 levels
*/
export enum ECL {
/**
* Low, 7%
*/
  L = 0,
/**
* Medium, 15%
*/
  M = 1,
/**
* Quartile, 25%
*/
  Q = 2,
/**
* High, 30%
*/
  H = 3,
}
/**
* Enum containing all possible `QRCode` versions
*/
export enum Version {
/**
* Version n°01
*/
  V01 = 0,
/**
* Version n°02
*/
  V02 = 1,
/**
* Version n°03
*/
  V03 = 2,
/**
* Version n°04
*/
  V04 = 3,
/**
* Version n°05
*/
  V05 = 4,
/**
* Version n°06
*/
  V06 = 5,
/**
* Version n°07
*/
  V07 = 6,
/**
* Version n°08
*/
  V08 = 7,
/**
* Version n°09
*/
  V09 = 8,
/**
* Version n°10
*/
  V10 = 9,
/**
* Version n°11
*/
  V11 = 10,
/**
* Version n°12
*/
  V12 = 11,
/**
* Version n°13
*/
  V13 = 12,
/**
* Version n°14
*/
  V14 = 13,
/**
* Version n°15
*/
  V15 = 14,
/**
* Version n°16
*/
  V16 = 15,
/**
* Version n°17
*/
  V17 = 16,
/**
* Version n°18
*/
  V18 = 17,
/**
* Version n°19
*/
  V19 = 18,
/**
* Version n°20
*/
  V20 = 19,
/**
* Version n°21
*/
  V21 = 20,
/**
* Version n°22
*/
  V22 = 21,
/**
* Version n°23
*/
  V23 = 22,
/**
* Version n°24
*/
  V24 = 23,
/**
* Version n°25
*/
  V25 = 24,
/**
* Version n°26
*/
  V26 = 25,
/**
* Version n°27
*/
  V27 = 26,
/**
* Version n°28
*/
  V28 = 27,
/**
* Version n°29
*/
  V29 = 28,
/**
* Version n°30
*/
  V30 = 29,
/**
* Version n°31
*/
  V31 = 30,
/**
* Version n°32
*/
  V32 = 31,
/**
* Version n°33
*/
  V33 = 32,
/**
* Version n°34
*/
  V34 = 33,
/**
* Version n°35
*/
  V35 = 34,
/**
* Version n°36
*/
  V36 = 35,
/**
* Version n°37
*/
  V37 = 36,
/**
* Version n°38
*/
  V38 = 37,
/**
* Version n°39
*/
  V39 = 38,
/**
* Version n°40
*/
  V40 = 39,
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
* Updates the error correction level of the QRCode (can increase the size of the QRCode)
* @param {number} ecl
* @returns {SvgOptions}
*/
  ecl(ecl: number): SvgOptions;
/**
* Forces the version of the QRCode
* @param {number} version
* @returns {SvgOptions}
*/
  version(version: number): SvgOptions;
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
  readonly svgoptions_ecl: (a: number, b: number) => number;
  readonly svgoptions_version: (a: number, b: number) => number;
  readonly svgoptions_new: () => number;
  readonly qr_svg: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
