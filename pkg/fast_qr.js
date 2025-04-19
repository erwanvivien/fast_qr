let wasm;

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

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
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

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
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}
/**
 * Generate a QR code from a string. All parameters are automatically set.
 * @param {string} content
 * @returns {Uint8Array}
 */
export function qr(content) {
    const ptr0 = passStringToWasm0(content, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.qr(ptr0, len0);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}

let cachedFloat64ArrayMemory0 = null;

function getFloat64ArrayMemory0() {
    if (cachedFloat64ArrayMemory0 === null || cachedFloat64ArrayMemory0.byteLength === 0) {
        cachedFloat64ArrayMemory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64ArrayMemory0;
}

function passArrayF64ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 8, 8) >>> 0;
    getFloat64ArrayMemory0().set(arg, ptr / 8);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}
/**
 * Generate a QR code from a string. All parameters are automatically set.
 * @param {string} content
 * @param {SvgOptions} options
 * @returns {string}
 */
export function qr_svg(content, options) {
    let deferred3_0;
    let deferred3_1;
    try {
        const ptr0 = passStringToWasm0(content, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(options, SvgOptions);
        var ptr1 = options.__destroy_into_raw();
        const ret = wasm.qr_svg(ptr0, len0, ptr1);
        deferred3_0 = ret[0];
        deferred3_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
    }
}

/**
 * Error Correction Coding has 4 levels
 * @enum {0 | 1 | 2 | 3}
 */
export const ECL = Object.freeze({
    /**
     * Low, 7%
     */
    L: 0, "0": "L",
    /**
     * Medium, 15%
     */
    M: 1, "1": "M",
    /**
     * Quartile, 25%
     */
    Q: 2, "2": "Q",
    /**
     * High, 30%
     */
    H: 3, "3": "H",
});
/**
 * Different possible image background shapes
 * @enum {0 | 1 | 2}
 */
export const ImageBackgroundShape = Object.freeze({
    /**
     * Square shape
     */
    Square: 0, "0": "Square",
    /**
     * Circle shape
     */
    Circle: 1, "1": "Circle",
    /**
     * Rounded square shape
     */
    RoundedSquare: 2, "2": "RoundedSquare",
});
/**
 * Different possible Shapes to represent modules in a [`crate::QRCode`]
 * @enum {0 | 1 | 2 | 3 | 4 | 5}
 */
export const Shape = Object.freeze({
    /**
     * Square Shape
     */
    Square: 0, "0": "Square",
    /**
     * Circle Shape
     */
    Circle: 1, "1": "Circle",
    /**
     * RoundedSquare Shape
     */
    RoundedSquare: 2, "2": "RoundedSquare",
    /**
     * Vertical Shape
     */
    Vertical: 3, "3": "Vertical",
    /**
     * Horizontal Shape
     */
    Horizontal: 4, "4": "Horizontal",
    /**
     * Diamond Shape
     */
    Diamond: 5, "5": "Diamond",
});
/**
 * Enum containing all possible `QRCode` versions
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39}
 */
export const Version = Object.freeze({
    /**
     * Version n°01
     */
    V01: 0, "0": "V01",
    /**
     * Version n°02
     */
    V02: 1, "1": "V02",
    /**
     * Version n°03
     */
    V03: 2, "2": "V03",
    /**
     * Version n°04
     */
    V04: 3, "3": "V04",
    /**
     * Version n°05
     */
    V05: 4, "4": "V05",
    /**
     * Version n°06
     */
    V06: 5, "5": "V06",
    /**
     * Version n°07
     */
    V07: 6, "6": "V07",
    /**
     * Version n°08
     */
    V08: 7, "7": "V08",
    /**
     * Version n°09
     */
    V09: 8, "8": "V09",
    /**
     * Version n°10
     */
    V10: 9, "9": "V10",
    /**
     * Version n°11
     */
    V11: 10, "10": "V11",
    /**
     * Version n°12
     */
    V12: 11, "11": "V12",
    /**
     * Version n°13
     */
    V13: 12, "12": "V13",
    /**
     * Version n°14
     */
    V14: 13, "13": "V14",
    /**
     * Version n°15
     */
    V15: 14, "14": "V15",
    /**
     * Version n°16
     */
    V16: 15, "15": "V16",
    /**
     * Version n°17
     */
    V17: 16, "16": "V17",
    /**
     * Version n°18
     */
    V18: 17, "17": "V18",
    /**
     * Version n°19
     */
    V19: 18, "18": "V19",
    /**
     * Version n°20
     */
    V20: 19, "19": "V20",
    /**
     * Version n°21
     */
    V21: 20, "20": "V21",
    /**
     * Version n°22
     */
    V22: 21, "21": "V22",
    /**
     * Version n°23
     */
    V23: 22, "22": "V23",
    /**
     * Version n°24
     */
    V24: 23, "23": "V24",
    /**
     * Version n°25
     */
    V25: 24, "24": "V25",
    /**
     * Version n°26
     */
    V26: 25, "25": "V26",
    /**
     * Version n°27
     */
    V27: 26, "26": "V27",
    /**
     * Version n°28
     */
    V28: 27, "27": "V28",
    /**
     * Version n°29
     */
    V29: 28, "28": "V29",
    /**
     * Version n°30
     */
    V30: 29, "29": "V30",
    /**
     * Version n°31
     */
    V31: 30, "30": "V31",
    /**
     * Version n°32
     */
    V32: 31, "31": "V32",
    /**
     * Version n°33
     */
    V33: 32, "32": "V33",
    /**
     * Version n°34
     */
    V34: 33, "33": "V34",
    /**
     * Version n°35
     */
    V35: 34, "34": "V35",
    /**
     * Version n°36
     */
    V36: 35, "35": "V36",
    /**
     * Version n°37
     */
    V37: 36, "36": "V37",
    /**
     * Version n°38
     */
    V38: 37, "37": "V38",
    /**
     * Version n°39
     */
    V39: 38, "38": "V39",
    /**
     * Version n°40
     */
    V40: 39, "39": "V40",
});

const SvgOptionsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_svgoptions_free(ptr >>> 0, 1));
/**
 * Configuration for the SVG output.
 */
export class SvgOptions {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SvgOptions.prototype);
        obj.__wbg_ptr = ptr;
        SvgOptionsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SvgOptionsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_svgoptions_free(ptr, 0);
    }
    /**
     * Updates the shape of the QRCode modules.
     * @param {Shape} shape
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
     * @param {ImageBackgroundShape} image_background_shape
     * @returns {SvgOptions}
     */
    image_background_shape(image_background_shape) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.svgoptions_image_background_shape(ptr, image_background_shape);
        return SvgOptions.__wrap(ret);
    }
    /**
     * Updates the size of the image. (unit being module size).
     * @param {number} size
     * @returns {SvgOptions}
     */
    image_size(size) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.svgoptions_image_size(ptr, size);
        return SvgOptions.__wrap(ret);
    }
    /**
     * Updates the gap between background color and the image. (unit being module size).
     * @param {number} gap
     * @returns {SvgOptions}
     */
    image_gap(gap) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.svgoptions_image_gap(ptr, gap);
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
     * Updates the error correction level of the QRCode (can increase the size of the QRCode)
     * @param {ECL} ecl
     * @returns {SvgOptions}
     */
    ecl(ecl) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.svgoptions_ecl(ptr, ecl);
        return SvgOptions.__wrap(ret);
    }
    /**
     * Forces the version of the QRCode
     * @param {Version} version
     * @returns {SvgOptions}
     */
    version(version) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.svgoptions_version(ptr, version);
        return SvgOptions.__wrap(ret);
    }
    /**
     * Creates a new SvgOptions object.
     */
    constructor() {
        const ret = wasm.svgoptions_new();
        this.__wbg_ptr = ret >>> 0;
        SvgOptionsFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

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

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_export_0;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
        ;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedFloat64ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('fast_qr_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
