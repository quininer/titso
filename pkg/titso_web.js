
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

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_20(arg0, arg1) {
    wasm._dyn_core_63dfd7407e7118b6___ops__function__FnMut_____Output______as_wasm_bindgen_6632896c218b51ea___closure__WasmClosure___describe__invoke______(arg0, arg1);
}

function __wbg_adapter_23(arg0, arg1, arg2) {
    wasm.closure57_externref_shim(arg0, arg1, arg2);
}

function __wbg_adapter_26(arg0, arg1, arg2) {
    wasm.closure67_externref_shim(arg0, arg1, arg2);
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_2.set(idx, obj);
    return idx;
}

function handleError(f) {
    return function () {
        try {
            return f.apply(this, arguments);

        } catch (e) {
            const idx = addToExternrefTable0(e);
            wasm.__wbindgen_exn_store(idx);
        }
    };
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
        input = import.meta.url.replace(/\.js$/, '_bg.wasm');
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
    imports.wbg.__wbg_self_a5f0fe5564782787 = handleError(function() {
        var ret = self.self;
        return ret;
    });
    imports.wbg.__wbg_require_29e58b5f6f133563 = handleError(function(arg0, arg1, arg2) {
        var ret = arg0.require(getStringFromWasm0(arg1, arg2));
        return ret;
    });
    imports.wbg.__wbg_crypto_d91429ea1a087f70 = function(arg0) {
        var ret = arg0.crypto;
        return ret;
    };
    imports.wbg.__wbg_msCrypto_c8be2bb4fc7d8cd3 = function(arg0) {
        var ret = arg0.msCrypto;
        return ret;
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        var ret = arg0 === undefined;
        return ret;
    };
    imports.wbg.__wbg_static_accessor_MODULE_7f278c5446c126c8 = function() {
        var ret = module;
        return ret;
    };
    imports.wbg.__wbg_getRandomValues_11115a852729f4e8 = handleError(function(arg0, arg1, arg2) {
        arg0.getRandomValues(getArrayU8FromWasm0(arg1, arg2));
    });
    imports.wbg.__wbg_randomFillSync_a2d002fc3b8e30f7 = handleError(function(arg0, arg1, arg2) {
        arg0.randomFillSync(getArrayU8FromWasm0(arg1, arg2));
    });
    imports.wbg.__wbg_new_3e06d4f36713e4cb = function() {
        var ret = new Object();
        return ret;
    };
    imports.wbg.__wbg_close_e718e777bdd51f95 = function(arg0) {
        arg0.close();
    };
    imports.wbg.__wbg_self_07b2f89e82ceb76d = handleError(function() {
        var ret = self.self;
        return ret;
    });
    imports.wbg.__wbg_window_ba85d88572adc0dc = handleError(function() {
        var ret = window.window;
        return ret;
    });
    imports.wbg.__wbg_globalThis_b9277fc37e201fe5 = handleError(function() {
        var ret = globalThis.globalThis;
        return ret;
    });
    imports.wbg.__wbg_global_e16303fe83e1d57f = handleError(function() {
        var ret = global.global;
        return ret;
    });
    imports.wbg.__wbg_newnoargs_f3b8a801d5d4b079 = function(arg0, arg1) {
        var ret = new Function(getStringFromWasm0(arg0, arg1));
        return ret;
    };
    imports.wbg.__wbg_call_8e95613cc6524977 = handleError(function(arg0, arg1) {
        var ret = arg0.call(arg1);
        return ret;
    });
    imports.wbg.__wbg_new_e13110f81ae347cf = function() {
        var ret = new Array();
        return ret;
    };
    imports.wbg.__wbg_push_b46eeec52d2b03bb = function(arg0, arg1) {
        var ret = arg0.push(arg1);
        return ret;
    };
    imports.wbg.__wbg_set_304f2ec1a3ab3b79 = handleError(function(arg0, arg1, arg2) {
        var ret = Reflect.set(arg0, arg1, arg2);
        return ret;
    });
    imports.wbg.__wbg_resolve_2529512c3bb73938 = function(arg0) {
        var ret = Promise.resolve(arg0);
        return ret;
    };
    imports.wbg.__wbg_then_4a7a614abbbe6d81 = function(arg0, arg1) {
        var ret = arg0.then(arg1);
        return ret;
    };
    imports.wbg.__wbg_then_3b7ac098cfda2fa5 = function(arg0, arg1, arg2) {
        var ret = arg0.then(arg1, arg2);
        return ret;
    };
    imports.wbg.__wbindgen_memory = function() {
        var ret = wasm.memory;
        return ret;
    };
    imports.wbg.__wbg_buffer_49131c283a06686f = function(arg0) {
        var ret = arg0.buffer;
        return ret;
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_c0f38401daad5a22 = function(arg0, arg1, arg2) {
        var ret = new Uint8Array(arg0, arg1 >>> 0, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_new_9b295d24cf1d706f = function(arg0) {
        var ret = new Uint8Array(arg0);
        return ret;
    };
    imports.wbg.__wbg_set_3bb960a9975f3cd2 = function(arg0, arg1, arg2) {
        arg0.set(arg1, arg2 >>> 0);
    };
    imports.wbg.__wbg_length_2b13641a9d906653 = function(arg0) {
        var ret = arg0.length;
        return ret;
    };
    imports.wbg.__wbg_document_6cc8d0b87c0a99b9 = function(arg0) {
        var ret = arg0.document;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_instanceof_HtmlElement_9cd64b297a10eb1e = function(arg0) {
        var ret = arg0 instanceof HTMLElement;
        return ret;
    };
    imports.wbg.__wbg_instanceof_HtmlInputElement_aaef9fb14eceaa9b = function(arg0) {
        var ret = arg0 instanceof HTMLInputElement;
        return ret;
    };
    imports.wbg.__wbg_instanceof_HtmlTextAreaElement_6c876047bbe08f92 = function(arg0) {
        var ret = arg0 instanceof HTMLTextAreaElement;
        return ret;
    };
    imports.wbg.__wbg_instanceof_HtmlButtonElement_645b6f9d0d172e00 = function(arg0) {
        var ret = arg0 instanceof HTMLButtonElement;
        return ret;
    };
    imports.wbg.__wbg_instanceof_HtmlAnchorElement_d9e9e82f92646521 = function(arg0) {
        var ret = arg0 instanceof HTMLAnchorElement;
        return ret;
    };
    imports.wbg.__wbg_indexedDB_e8c06c6b6d20e442 = handleError(function(arg0) {
        var ret = arg0.indexedDB;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    });
    imports.wbg.__wbg_open_affb30c9eece38df = handleError(function(arg0, arg1, arg2) {
        var ret = arg0.open(getStringFromWasm0(arg1, arg2));
        return ret;
    });
    imports.wbg.__wbg_setonupgradeneeded_f23a8702db7a97e0 = function(arg0, arg1) {
        arg0.onupgradeneeded = arg1;
    };
    imports.wbg.__wbg_setonsuccess_f98f30972e88488d = function(arg0, arg1) {
        arg0.onsuccess = arg1;
    };
    imports.wbg.__wbg_setonerror_4a5dc453f97e6402 = function(arg0, arg1) {
        arg0.onerror = arg1;
    };
    imports.wbg.__wbg_error_05fd670065af9e70 = handleError(function(arg0) {
        var ret = arg0.error;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    });
    imports.wbg.__wbg_result_a13c3411d4796cd5 = handleError(function(arg0) {
        var ret = arg0.result;
        return ret;
    });
    imports.wbg.__wbg_addEventListener_f0baf69c9c7425c9 = handleError(function(arg0, arg1, arg2, arg3, arg4) {
        arg0.addEventListener(getStringFromWasm0(arg1, arg2), arg3, arg4);
    });
    imports.wbg.__wbg_setdisabled_99145ea35d651bfe = function(arg0, arg1) {
        arg0.disabled = arg1 !== 0;
    };
    imports.wbg.__wbg_querySelector_69fd5cd784bcc892 = handleError(function(arg0, arg1, arg2) {
        var ret = arg0.querySelector(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    });
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = arg0.original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        var ret = false;
        return ret;
    };
    imports.wbg.__wbg_objectStore_089f0d1d943d4d1e = handleError(function(arg0, arg1, arg2) {
        var ret = arg0.objectStore(getStringFromWasm0(arg1, arg2));
        return ret;
    });
    imports.wbg.__wbg_openCursor_529ad72fe211a1ea = handleError(function(arg0, arg1) {
        var ret = arg0.openCursor(arg1);
        return ret;
    });
    imports.wbg.__wbg_openCursor_73b89b4b96f01ccd = handleError(function(arg0) {
        var ret = arg0.openCursor();
        return ret;
    });
    imports.wbg.__wbg_alert_af5c272d926011d5 = handleError(function(arg0, arg1, arg2) {
        arg0.alert(getStringFromWasm0(arg1, arg2));
    });
    imports.wbg.__wbg_newwithu8arraysequence_fda653c226cde342 = handleError(function(arg0, arg1, arg2) {
        var ret = new File(arg0, getStringFromWasm0(arg1, arg2));
        return ret;
    });
    imports.wbg.__wbg_createObjectURL_356d0e93fee8a070 = handleError(function(arg0, arg1) {
        var ret = URL.createObjectURL(arg1);
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    });
    imports.wbg.__wbg_open_016e1ce96cc27f6a = handleError(function(arg0, arg1, arg2) {
        var ret = arg0.open(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    });
    imports.wbg.__wbg_key_a121db69fa6f6b0f = handleError(function(arg0) {
        var ret = arg0.key;
        return ret;
    });
    imports.wbg.__wbg_value_7f632e51a53c7897 = handleError(function(arg0) {
        var ret = arg0.value;
        return ret;
    });
    imports.wbg.__wbg_continue_f608f9a7cf9c739a = handleError(function(arg0) {
        arg0.continue();
    });
    imports.wbg.__wbg_get_04ed71603e88012b = handleError(function(arg0, arg1) {
        var ret = arg0.get(arg1);
        return ret;
    });
    imports.wbg.__wbg_files_ecbda252a7b3abd7 = function(arg0) {
        var ret = arg0.files;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_get_bf9bd877034efef8 = function(arg0, arg1) {
        var ret = arg0[arg1 >>> 0];
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_arrayBuffer_8152406b83585e19 = function(arg0) {
        var ret = arg0.arrayBuffer();
        return ret;
    };
    imports.wbg.__wbg_put_0e0b20aed9866a03 = handleError(function(arg0, arg1, arg2) {
        var ret = arg0.put(arg1, arg2);
        return ret;
    });
    imports.wbg.__wbg_click_1e787c2777ec0972 = function(arg0) {
        arg0.click();
    };
    imports.wbg.__wbg_setvalue_839acf17e43a847f = function(arg0, arg1, arg2) {
        arg0.value = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_sethidden_912c0ab43cd6ed09 = function(arg0, arg1) {
        arg0.hidden = arg1 !== 0;
    };
    imports.wbg.__wbg_style_9a41d46c005f7596 = function(arg0) {
        var ret = arg0.style;
        return ret;
    };
    imports.wbg.__wbg_setProperty_42eabadfcd7d6199 = handleError(function(arg0, arg1, arg2, arg3, arg4) {
        arg0.setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    });
    imports.wbg.__wbg_setvalue_1012134a2989f3ee = function(arg0, arg1, arg2) {
        arg0.value = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_instanceof_KeyboardEvent_f3e957ac7e5a3f7d = function(arg0) {
        var ret = arg0 instanceof KeyboardEvent;
        return ret;
    };
    imports.wbg.__wbg_key_590d4d2a765d1b58 = function(arg0, arg1) {
        var ret = arg1.key;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_value_bff6f7ef104e077a = function(arg0, arg1) {
        var ret = arg1.value;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_setchecked_1a89c058f5ac906a = function(arg0, arg1) {
        arg0.checked = arg1 !== 0;
    };
    imports.wbg.__wbg_setvalueAsNumber_e055cd6dc6cf028b = function(arg0, arg1) {
        arg0.valueAsNumber = arg1;
    };
    imports.wbg.__wbg_type_c26c07fdd821d8e7 = function(arg0, arg1) {
        var ret = arg1.type;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_settype_4f9e0a597bc38efe = function(arg0, arg1, arg2) {
        arg0.type = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_select_64b382d394e30924 = function(arg0) {
        arg0.select();
    };
    imports.wbg.__wbg_delete_8984e815985d3404 = handleError(function(arg0, arg1) {
        var ret = arg0.delete(arg1);
        return ret;
    });
    imports.wbg.__wbg_checked_bd3a45386afc949e = function(arg0) {
        var ret = arg0.checked;
        return ret;
    };
    imports.wbg.__wbg_valueAsNumber_0b0d9c814c72fa90 = function(arg0) {
        var ret = arg0.valueAsNumber;
        return ret;
    };
    imports.wbg.__wbg_value_91d41b8dbd0b2f0b = function(arg0, arg1) {
        var ret = arg1.value;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_setreadOnly_2c2f871781064c26 = function(arg0, arg1) {
        arg0.readOnly = arg1 !== 0;
    };
    imports.wbg.__wbg_preventDefault_93d06688748bfc14 = function(arg0) {
        arg0.preventDefault();
    };
    imports.wbg.__wbg_objectStoreNames_184ef193940cd5ed = function(arg0) {
        var ret = arg0.objectStoreNames;
        return ret;
    };
    imports.wbg.__wbg_contains_9a5f3512a339d9a2 = function(arg0, arg1, arg2) {
        var ret = arg0.contains(getStringFromWasm0(arg1, arg2));
        return ret;
    };
    imports.wbg.__wbg_createObjectStore_a40a77ecea42673f = handleError(function(arg0, arg1, arg2) {
        var ret = arg0.createObjectStore(getStringFromWasm0(arg1, arg2));
        return ret;
    });
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
    imports.wbg.__wbg_instanceof_Window_adf3196bdc02b386 = function(arg0) {
        var ret = arg0 instanceof Window;
        return ret;
    };
    imports.wbg.__wbg_transaction_b3441d4f50996395 = handleError(function(arg0, arg1, arg2, arg3) {
        var ret = arg0.transaction(getStringFromWasm0(arg1, arg2), arg3);
        return ret;
    });
    imports.wbg.__wbg_debug_b443de592faba09f = typeof console.debug == 'function' ? console.debug : notDefined('console.debug');
    imports.wbg.__wbg_error_7f083efc6bc6752c = typeof console.error == 'function' ? console.error : notDefined('console.error');
    imports.wbg.__wbg_info_6d4a86f0fd590270 = typeof console.info == 'function' ? console.info : notDefined('console.info');
    imports.wbg.__wbg_log_3bafd82835c6de6d = typeof console.log == 'function' ? console.log : notDefined('console.log');
    imports.wbg.__wbg_warn_d05e82888b7fad05 = typeof console.warn == 'function' ? console.warn : notDefined('console.warn');
    imports.wbg.__wbindgen_closure_wrapper1213 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 55, __wbg_adapter_20);
        return ret;
    };
    imports.wbg.__wbindgen_closure_wrapper1224 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 58, __wbg_adapter_23);
        return ret;
    };
    imports.wbg.__wbindgen_closure_wrapper1533 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 68, __wbg_adapter_26);
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

