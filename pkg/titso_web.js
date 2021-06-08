
let wasm;

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

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

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

const CLOSURE_DTORS = new FinalizationRegistry(state => {
    wasm.__wbindgen_export_3.get(state.dtor)(state.a, state.b)
});

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_3.get(state.dtor)(a, state.b);
                CLOSURE_DTORS.unregister(state)
            } else {
                state.a = a;
            }
        }
    };
    real.original = state;
    CLOSURE_DTORS.register(real, state, state);
    return real;
}
function __wbg_adapter_24(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h84b2252de8973aee(arg0, arg1);
}

function __wbg_adapter_27(arg0, arg1, arg2) {
    wasm.closure51_externref_shim(arg0, arg1, arg2);
}

function __wbg_adapter_30(arg0, arg1, arg2) {
    wasm.closure62_externref_shim(arg0, arg1, arg2);
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_2.set(idx, obj);
    return idx;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}
/**
*/
export function start() {
    wasm.start();
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function notDefined(what) { return () => { throw new Error(`${what} is not defined`); }; }

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

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('titso_web_bg.wasm', import.meta.url);
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_new_59cb74e423758ede = function() {
        var ret = new Error();
        return ret;
    };
    imports.wbg.__wbg_stack_558ba5917b466edd = function(arg0, arg1) {
        var ret = arg1.stack;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
        try {
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(arg0, arg1);
        }
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        var ret = getStringFromWasm0(arg0, arg1);
        return ret;
    };
    imports.wbg.__wbg_process_2f24d6544ea7b200 = function(arg0) {
        var ret = arg0.process;
        return ret;
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = arg0;
        var ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbg_versions_6164651e75405d4a = function(arg0) {
        var ret = arg0.versions;
        return ret;
    };
    imports.wbg.__wbg_node_4b517d861cbcb3bc = function(arg0) {
        var ret = arg0.node;
        return ret;
    };
    imports.wbg.__wbindgen_is_string = function(arg0) {
        var ret = typeof(arg0) === 'string';
        return ret;
    };
    imports.wbg.__wbg_crypto_98fc271021c7d2ad = function(arg0) {
        var ret = arg0.crypto;
        return ret;
    };
    imports.wbg.__wbg_msCrypto_a2cdb043d2bfe57f = function(arg0) {
        var ret = arg0.msCrypto;
        return ret;
    };
    imports.wbg.__wbg_newwithlength_a8d1dbcbe703a5c6 = function(arg0) {
        var ret = new Uint8Array(arg0 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_modulerequire_3440a4bcf44437db = function() { return handleError(function (arg0, arg1) {
        var ret = module.require(getStringFromWasm0(arg0, arg1));
        return ret;
    }, arguments) };
    imports.wbg.__wbg_randomFillSync_64cc7d048f228ca8 = function() { return handleError(function (arg0, arg1, arg2) {
        arg0.randomFillSync(getArrayU8FromWasm0(arg1, arg2));
    }, arguments) };
    imports.wbg.__wbg_subarray_901ede8318da52a6 = function(arg0, arg1, arg2) {
        var ret = arg0.subarray(arg1 >>> 0, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_getRandomValues_98117e9a7e993920 = function() { return handleError(function (arg0, arg1) {
        arg0.getRandomValues(arg1);
    }, arguments) };
    imports.wbg.__wbg_close_e2d0920ba3974c3b = function(arg0) {
        arg0.close();
    };
    imports.wbg.__wbg_self_bb69a836a72ec6e9 = function() { return handleError(function () {
        var ret = self.self;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_window_3304fc4b414c9693 = function() { return handleError(function () {
        var ret = window.window;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_globalThis_e0d21cabc6630763 = function() { return handleError(function () {
        var ret = globalThis.globalThis;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_global_8463719227271676 = function() { return handleError(function () {
        var ret = global.global;
        return ret;
    }, arguments) };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        var ret = arg0 === undefined;
        return ret;
    };
    imports.wbg.__wbg_newnoargs_9fdd8f3961dd1bee = function(arg0, arg1) {
        var ret = new Function(getStringFromWasm0(arg0, arg1));
        return ret;
    };
    imports.wbg.__wbg_call_ba36642bd901572b = function() { return handleError(function (arg0, arg1) {
        var ret = arg0.call(arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_new_515b65a8e7699d00 = function() {
        var ret = new Array();
        return ret;
    };
    imports.wbg.__wbg_push_b7f68478f81d358b = function(arg0, arg1) {
        var ret = arg0.push(arg1);
        return ret;
    };
    imports.wbg.__wbg_new_edbe38a4e21329dd = function() {
        var ret = new Object();
        return ret;
    };
    imports.wbg.__wbg_set_73349fc4814e0fc6 = function() { return handleError(function (arg0, arg1, arg2) {
        var ret = Reflect.set(arg0, arg1, arg2);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_resolve_cae3d8f752f5db88 = function(arg0) {
        var ret = Promise.resolve(arg0);
        return ret;
    };
    imports.wbg.__wbg_then_c2361a9d5c9a4fcb = function(arg0, arg1) {
        var ret = arg0.then(arg1);
        return ret;
    };
    imports.wbg.__wbg_then_6c9a4bf55755f9b8 = function(arg0, arg1, arg2) {
        var ret = arg0.then(arg1, arg2);
        return ret;
    };
    imports.wbg.__wbindgen_memory = function() {
        var ret = wasm.memory;
        return ret;
    };
    imports.wbg.__wbg_buffer_9e184d6f785de5ed = function(arg0) {
        var ret = arg0.buffer;
        return ret;
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_e57ad1f2ce812c03 = function(arg0, arg1, arg2) {
        var ret = new Uint8Array(arg0, arg1 >>> 0, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_new_e8101319e4cf95fc = function(arg0) {
        var ret = new Uint8Array(arg0);
        return ret;
    };
    imports.wbg.__wbg_set_e8ae7b27314e8b98 = function(arg0, arg1, arg2) {
        arg0.set(arg1, arg2 >>> 0);
    };
    imports.wbg.__wbg_length_2d56cb37075fcfb1 = function(arg0) {
        var ret = arg0.length;
        return ret;
    };
    imports.wbg.__wbg_document_5aff8cd83ef968f5 = function(arg0) {
        var ret = arg0.document;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_instanceof_HtmlElement_835072e813858ac0 = function(arg0) {
        var ret = arg0 instanceof HTMLElement;
        return ret;
    };
    imports.wbg.__wbg_instanceof_HtmlInputElement_df6fbc606ba24e20 = function(arg0) {
        var ret = arg0 instanceof HTMLInputElement;
        return ret;
    };
    imports.wbg.__wbg_instanceof_HtmlTextAreaElement_244fe1b35f3576f5 = function(arg0) {
        var ret = arg0 instanceof HTMLTextAreaElement;
        return ret;
    };
    imports.wbg.__wbg_instanceof_HtmlButtonElement_da6b54269a93893e = function(arg0) {
        var ret = arg0 instanceof HTMLButtonElement;
        return ret;
    };
    imports.wbg.__wbg_instanceof_HtmlAnchorElement_fb8c991ea5f60f22 = function(arg0) {
        var ret = arg0 instanceof HTMLAnchorElement;
        return ret;
    };
    imports.wbg.__wbg_indexedDB_9747d1198ca79df8 = function() { return handleError(function (arg0) {
        var ret = arg0.indexedDB;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    }, arguments) };
    imports.wbg.__wbg_open_9ace4089a034cd64 = function() { return handleError(function (arg0, arg1, arg2) {
        var ret = arg0.open(getStringFromWasm0(arg1, arg2));
        return ret;
    }, arguments) };
    imports.wbg.__wbg_setonupgradeneeded_2302b142979ab0da = function(arg0, arg1) {
        arg0.onupgradeneeded = arg1;
    };
    imports.wbg.__wbg_setonsuccess_3f96f99ee181f23e = function(arg0, arg1) {
        arg0.onsuccess = arg1;
    };
    imports.wbg.__wbg_setonerror_ad0ba5e117be3b58 = function(arg0, arg1) {
        arg0.onerror = arg1;
    };
    imports.wbg.__wbg_error_8b87a600d89439ea = function() { return handleError(function (arg0) {
        var ret = arg0.error;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    }, arguments) };
    imports.wbg.__wbg_result_a2136eaa9b6a8091 = function() { return handleError(function (arg0) {
        var ret = arg0.result;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_addEventListener_6d9a78a5d277bdaf = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        arg0.addEventListener(getStringFromWasm0(arg1, arg2), arg3, arg4);
    }, arguments) };
    imports.wbg.__wbg_setdisabled_1f38a7ba42cfe924 = function(arg0, arg1) {
        arg0.disabled = arg1 !== 0;
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = arg0.original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        var ret = false;
        return ret;
    };
    imports.wbg.__wbg_querySelector_be2ba71c84a94384 = function() { return handleError(function (arg0, arg1, arg2) {
        var ret = arg0.querySelector(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    }, arguments) };
    imports.wbg.__wbg_objectStore_021a1b68448ceb4a = function() { return handleError(function (arg0, arg1, arg2) {
        var ret = arg0.objectStore(getStringFromWasm0(arg1, arg2));
        return ret;
    }, arguments) };
    imports.wbg.__wbg_openCursor_8cc27f66a1e615b3 = function() { return handleError(function (arg0, arg1) {
        var ret = arg0.openCursor(arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_openCursor_5b140cc0e2727252 = function() { return handleError(function (arg0) {
        var ret = arg0.openCursor();
        return ret;
    }, arguments) };
    imports.wbg.__wbg_newwithu8arraysequence_f8f2f7ae16d4884b = function() { return handleError(function (arg0, arg1, arg2) {
        var ret = new File(arg0, getStringFromWasm0(arg1, arg2));
        return ret;
    }, arguments) };
    imports.wbg.__wbg_createObjectURL_a97d76bcc0a4968c = function() { return handleError(function (arg0, arg1) {
        var ret = URL.createObjectURL(arg1);
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    }, arguments) };
    imports.wbg.__wbg_open_666e62eebf9fef92 = function() { return handleError(function (arg0, arg1, arg2) {
        var ret = arg0.open(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    }, arguments) };
    imports.wbg.__wbg_alert_d011c949f36fa720 = function() { return handleError(function (arg0, arg1, arg2) {
        arg0.alert(getStringFromWasm0(arg1, arg2));
    }, arguments) };
    imports.wbg.__wbg_key_3df98554773477fa = function() { return handleError(function (arg0) {
        var ret = arg0.key;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_value_95f8b65f8d354dac = function() { return handleError(function (arg0) {
        var ret = arg0.value;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_continue_f8df112ef1252fce = function() { return handleError(function (arg0) {
        arg0.continue();
    }, arguments) };
    imports.wbg.__wbg_get_e1ff9456eedf8b36 = function() { return handleError(function (arg0, arg1) {
        var ret = arg0.get(arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_files_a44d8f0fdedd825f = function(arg0) {
        var ret = arg0.files;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_get_3bf3d94d9366367b = function(arg0, arg1) {
        var ret = arg0[arg1 >>> 0];
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_arrayBuffer_ce658a27baec9674 = function(arg0) {
        var ret = arg0.arrayBuffer();
        return ret;
    };
    imports.wbg.__wbg_put_5c628d6fe18be216 = function() { return handleError(function (arg0, arg1, arg2) {
        var ret = arg0.put(arg1, arg2);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_click_a22518ab89bfd9b8 = function(arg0) {
        arg0.click();
    };
    imports.wbg.__wbg_setvalue_65a652cfd99c8a4a = function(arg0, arg1, arg2) {
        arg0.value = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_sethidden_4e6127e185ecf2df = function(arg0, arg1) {
        arg0.hidden = arg1 !== 0;
    };
    imports.wbg.__wbg_style_25309daade79abb3 = function(arg0) {
        var ret = arg0.style;
        return ret;
    };
    imports.wbg.__wbg_setProperty_dccccce3a52c26db = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        arg0.setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_setvalue_b1b2f2945b1cb6ef = function(arg0, arg1, arg2) {
        arg0.value = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_instanceof_KeyboardEvent_61a0135987c0e26f = function(arg0) {
        var ret = arg0 instanceof KeyboardEvent;
        return ret;
    };
    imports.wbg.__wbg_key_6827d862c9cc3928 = function(arg0, arg1) {
        var ret = arg1.key;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_value_f4c762446c572119 = function(arg0, arg1) {
        var ret = arg1.value;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_setchecked_dc7daac77dc0e73e = function(arg0, arg1) {
        arg0.checked = arg1 !== 0;
    };
    imports.wbg.__wbg_setvalueAsNumber_84bb389cbb16676a = function(arg0, arg1) {
        arg0.valueAsNumber = arg1;
    };
    imports.wbg.__wbg_type_8154bde586a1b6a5 = function(arg0, arg1) {
        var ret = arg1.type;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_settype_bd7de9ca642dc3b2 = function(arg0, arg1, arg2) {
        arg0.type = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_select_29835af4faa80004 = function(arg0) {
        arg0.select();
    };
    imports.wbg.__wbg_delete_b36d0839032d1ea1 = function() { return handleError(function (arg0, arg1) {
        var ret = arg0.delete(arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_checked_dc000202a8fa9328 = function(arg0) {
        var ret = arg0.checked;
        return ret;
    };
    imports.wbg.__wbg_valueAsNumber_c5f9b4474bf3921c = function(arg0) {
        var ret = arg0.valueAsNumber;
        return ret;
    };
    imports.wbg.__wbg_value_d8dfe9a459c6ea2a = function(arg0, arg1) {
        var ret = arg1.value;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_setreadOnly_32fdbe1ee2751bef = function(arg0, arg1) {
        arg0.readOnly = arg1 !== 0;
    };
    imports.wbg.__wbg_preventDefault_7c4d18eb2bb1a26a = function(arg0) {
        arg0.preventDefault();
    };
    imports.wbg.__wbg_objectStoreNames_0f8a780cd7a6d518 = function(arg0) {
        var ret = arg0.objectStoreNames;
        return ret;
    };
    imports.wbg.__wbg_contains_580219ba80dcefc5 = function(arg0, arg1, arg2) {
        var ret = arg0.contains(getStringFromWasm0(arg1, arg2));
        return ret;
    };
    imports.wbg.__wbg_createObjectStore_3c72635cad3457fd = function() { return handleError(function (arg0, arg1, arg2) {
        var ret = arg0.createObjectStore(getStringFromWasm0(arg1, arg2));
        return ret;
    }, arguments) };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        var ret = debugString(arg1);
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg_instanceof_Window_11e25482011fc506 = function(arg0) {
        var ret = arg0 instanceof Window;
        return ret;
    };
    imports.wbg.__wbg_transaction_93e331534592706d = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        var ret = arg0.transaction(getStringFromWasm0(arg1, arg2), arg3);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_debug_50dabfdc7da1bce4 = typeof console.debug == 'function' ? console.debug : notDefined('console.debug');
    imports.wbg.__wbg_error_d95afd6217cfd219 = typeof console.error == 'function' ? console.error : notDefined('console.error');
    imports.wbg.__wbg_info_ddbff0efdb9cff60 = typeof console.info == 'function' ? console.info : notDefined('console.info');
    imports.wbg.__wbg_log_9a99fb1af846153b = typeof console.log == 'function' ? console.log : notDefined('console.log');
    imports.wbg.__wbg_warn_b39e749f1dc02058 = typeof console.warn == 'function' ? console.warn : notDefined('console.warn');
    imports.wbg.__wbindgen_closure_wrapper1210 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 49, __wbg_adapter_24);
        return ret;
    };
    imports.wbg.__wbindgen_closure_wrapper1220 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 52, __wbg_adapter_27);
        return ret;
    };
    imports.wbg.__wbindgen_closure_wrapper1528 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 63, __wbg_adapter_30);
        return ret;
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_export_2;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
        ;
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }



    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    wasm.__wbindgen_start();
    return wasm;
}

export default init;

