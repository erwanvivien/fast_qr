
let wasm;

const cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8Memory0 = new Uint8Array();

function getUint8Memory0() {
    if (cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedInt32Memory0 = new Int32Array();

function getInt32Memory0() {
    if (cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}
/**
* Generate a QR code from a string. All parameters are automatically set.
* @param {string} content
* @returns {Uint8Array}
*/
export function qr(content) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(content, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.qr(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var v1 = getArrayU8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_free(r0, r1 * 1);
        return v1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

let cachedFloat64Memory0 = new Float64Array();

function getFloat64Memory0() {
    if (cachedFloat64Memory0.byteLength === 0) {
        cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64Memory0;
}

function passArrayF64ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 8);
    getFloat64Memory0().set(arg, ptr / 8);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}
/**
* Generate a QR code from a string. All parameters are automatically set.
* @param {string} content
* @param {SvgOptions} options
* @returns {string}
*/
export function qr_svg(content, options) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(content, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(options, SvgOptions);
        var ptr1 = options.ptr;
        options.ptr = 0;
        wasm.qr_svg(retptr, ptr0, len0, ptr1);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* Different possible Shapes to represent modules in a QRCode
*/
export const Shape = Object.freeze({
/**
* Square Shape
*/
Square:0,"0":"Square",
/**
* Circle Shape
*/
Circle:1,"1":"Circle",
/**
* RoundedSquare Shape
*/
RoundedSquare:2,"2":"RoundedSquare",
/**
* Vertical Shape
*/
Vertical:3,"3":"Vertical",
/**
* Horizontal Shape
*/
Horizontal:4,"4":"Horizontal",
/**
* Diamond Shape
*/
Diamond:5,"5":"Diamond", });
/**
* Different possible image background shapes
*/
export const ImageBackgroundShape = Object.freeze({
/**
* Square shape
*/
Square:0,"0":"Square",
/**
* Circle shape
*/
Circle:1,"1":"Circle",
/**
* Rounded square shape
*/
RoundedSquare:2,"2":"RoundedSquare", });
/**
* Configuration for the SVG output.
*/
export class SvgOptions {

    static __wrap(ptr) {
        const obj = Object.create(SvgOptions.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_svgoptions_free(ptr);
    }
    /**
    * Updates the shape of the QRCode modules.
    * @param {number} shape
    * @returns {SvgOptions}
    */
    shape(shape) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.svgoptions_shape(ptr, shape);
        return SvgOptions.__wrap(ret);
    }
    /**
    * Updates the module color of the QRCode. Tales a string in the format `#RRGGBB[AA]`.
    * @param {string} module_color
    * @returns {SvgOptions}
    */
    module_color(module_color) {
        const ptr = this.__destroy_into_raw();
        const ptr0 = passStringToWasm0(module_color, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.svgoptions_module_color(ptr, ptr0, len0);
        return SvgOptions.__wrap(ret);
    }
    /**
    * Updates the margin of the QRCode.
    * @param {number} margin
    * @returns {SvgOptions}
    */
    margin(margin) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.svgoptions_margin(ptr, margin);
        return SvgOptions.__wrap(ret);
    }
    /**
    * Updates the background color of the QRCode. Tales a string in the format `#RRGGBB[AA]`.
    * @param {string} background_color
    * @returns {SvgOptions}
    */
    background_color(background_color) {
        const ptr = this.__destroy_into_raw();
        const ptr0 = passStringToWasm0(background_color, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.svgoptions_background_color(ptr, ptr0, len0);
        return SvgOptions.__wrap(ret);
    }
    /**
    * Updates the image of the QRCode. Takes base64 or a url.
    * @param {string} image
    * @returns {SvgOptions}
    */
    image(image) {
        const ptr = this.__destroy_into_raw();
        const ptr0 = passStringToWasm0(image, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.svgoptions_image(ptr, ptr0, len0);
        return SvgOptions.__wrap(ret);
    }
    /**
    * Updates the background color of the image. Takes a string in the format `#RRGGBB[AA]`.
    * @param {string} image_background_color
    * @returns {SvgOptions}
    */
    image_background_color(image_background_color) {
        const ptr = this.__destroy_into_raw();
        const ptr0 = passStringToWasm0(image_background_color, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.svgoptions_image_background_color(ptr, ptr0, len0);
        return SvgOptions.__wrap(ret);
    }
    /**
    * Updates the shape of the image background. Takes an convert::ImageBackgroundShape.
    * @param {number} image_background_shape
    * @returns {SvgOptions}
    */
    image_background_shape(image_background_shape) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.svgoptions_image_background_shape(ptr, image_background_shape);
        return SvgOptions.__wrap(ret);
    }
    /**
    * Updates the size of the image. Takes a size and a gap (unit being module size).
    * @param {number} size
    * @param {number} gap
    * @returns {SvgOptions}
    */
    image_size(size, gap) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.svgoptions_image_size(ptr, size, gap);
        return SvgOptions.__wrap(ret);
    }
    /**
    * Updates the position of the image. Takes an array [x, y] (unit being module size).
    * @param {Float64Array} image_position
    * @returns {SvgOptions}
    */
    image_position(image_position) {
        const ptr = this.__destroy_into_raw();
        const ptr0 = passArrayF64ToWasm0(image_position, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.svgoptions_image_position(ptr, ptr0, len0);
        return SvgOptions.__wrap(ret);
    }
    /**
    * Creates a new SvgOptions object.
    */
    constructor() {
        const ret = wasm.svgoptions_new();
        return SvgOptions.__wrap(ret);
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function getImports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function initMemory(imports, maybe_memory) {

}

function finalizeInit(instance, module) {
    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    cachedFloat64Memory0 = new Float64Array();
    cachedInt32Memory0 = new Int32Array();
    cachedUint8Memory0 = new Uint8Array();


    return wasm;
}

function initSync(module) {
    const imports = getImports();

    initMemory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return finalizeInit(instance, module);
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('fast_qr_bg.wasm', import.meta.url);
    }
    const imports = getImports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    initMemory(imports);

    const { instance, module } = await load(await input, imports);

    return finalizeInit(instance, module);
}

export { initSync }
export default init;
