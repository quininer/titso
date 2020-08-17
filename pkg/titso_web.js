
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
function __wbg_adapter_20(arg0, arg1, arg2) {
    wasm.closure340_externref_shim(arg0, arg1, arg2);
}

function __wbg_adapter_23(arg0, arg1, arg2) {
    wasm.closure205_externref_shim(arg0, arg1, arg2);
}

function __wbg_adapter_26(arg0, arg1) {
    wasm._dyn_core_11e4cce1c6ea371b___ops__function__FnMut_____Output______as_wasm_bindgen_8404defc4f0a9479___closure__WasmClosure___describe__invoke______(arg0, arg1);
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
    imports.wbg.__wbg_self_1b7a39e3a92c949c = handleError(function() {
        var ret = self.self;
        return ret;
    });
    imports.wbg.__wbg_require_604837428532a733 = function(arg0, arg1) {
        var ret = require(getStringFromWasm0(arg0, arg1));
        return ret;
    };
    imports.wbg.__wbg_crypto_968f1772287e2df0 = function(arg0) {
        var ret = arg0.crypto;
        return ret;
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        var ret = arg0 === undefined;
        return ret;
    };
    imports.wbg.__wbg_getRandomValues_a3d34b4fee3c2869 = function(arg0) {
        var ret = arg0.getRandomValues;
        return ret;
    };
    imports.wbg.__wbg_getRandomValues_f5e14ab7ac8e995d = function(arg0, arg1, arg2) {
        arg0.getRandomValues(getArrayU8FromWasm0(arg1, arg2));
    };
    imports.wbg.__wbg_randomFillSync_d5bd2d655fdf256a = function(arg0, arg1, arg2) {
        arg0.randomFillSync(getArrayU8FromWasm0(arg1, arg2));
    };
    imports.wbg.__wbg_new_8172f4fed77fdb7c = function() {
        var ret = new Object();
        return ret;
    };
    imports.wbg.__wbg_close_f143d62c258f91a2 = function(arg0) {
        arg0.close();
    };
    imports.wbg.__wbg_new_17534eac4df3cd22 = function() {
        var ret = new Array();
        return ret;
    };
    imports.wbg.__wbg_push_7114ccbf1c58e41f = function(arg0, arg1) {
        var ret = arg0.push(arg1);
        return ret;
    };
    imports.wbg.__wbg_call_e9f0ce4da840ab94 = handleError(function(arg0, arg1) {
        var ret = arg0.call(arg1);
        return ret;
    });
    imports.wbg.__wbg_resolve_4df26938859b92e3 = function(arg0) {
        var ret = Promise.resolve(arg0);
        return ret;
    };
    imports.wbg.__wbg_then_ffb6e71f7a6735ad = function(arg0, arg1) {
        var ret = arg0.then(arg1);
        return ret;
    };
    imports.wbg.__wbg_then_021fcdc7f0350b58 = function(arg0, arg1, arg2) {
        var ret = arg0.then(arg1, arg2);
        return ret;
    };
    imports.wbg.__wbg_self_179e8c2a5a4c73a3 = handleError(function() {
        var ret = self.self;
        return ret;
    });
    imports.wbg.__wbg_window_492cfe63a6e41dfa = handleError(function() {
        var ret = window.window;
        return ret;
    });
    imports.wbg.__wbg_globalThis_8ebfea75c2dd63ee = handleError(function() {
        var ret = globalThis.globalThis;
        return ret;
    });
    imports.wbg.__wbg_global_62ea2619f58bf94d = handleError(function() {
        var ret = global.global;
        return ret;
    });
    imports.wbg.__wbg_newnoargs_e2fdfe2af14a2323 = function(arg0, arg1) {
        var ret = new Function(getStringFromWasm0(arg0, arg1));
        return ret;
    };
    imports.wbg.__wbindgen_memory = function() {
        var ret = wasm.memory;
        return ret;
    };
    imports.wbg.__wbg_buffer_88f603259d7a7b82 = function(arg0) {
        var ret = arg0.buffer;
        return ret;
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_a048d126789a272b = function(arg0, arg1, arg2) {
        var ret = new Uint8Array(arg0, arg1 >>> 0, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_new_85d8a1fc4384acef = function(arg0) {
        var ret = new Uint8Array(arg0);
        return ret;
    };
    imports.wbg.__wbg_set_478951586c457484 = function(arg0, arg1, arg2) {
        arg0.set(arg1, arg2 >>> 0);
    };
    imports.wbg.__wbg_length_2e98733d73dac355 = function(arg0) {
        var ret = arg0.length;
        return ret;
    };
    imports.wbg.__wbg_set_afe54b1eeb1aa77c = handleError(function(arg0, arg1, arg2) {
        var ret = Reflect.set(arg0, arg1, arg2);
        return ret;
    });
    imports.wbg.__wbg_addEventListener_27eb43df29303d67 = handleError(function(arg0, arg1, arg2, arg3, arg4) {
        arg0.addEventListener(getStringFromWasm0(arg1, arg2), arg3, arg4);
    });
    imports.wbg.__wbg_instanceof_HtmlElement_773e85b6bd68ae2d = function(arg0) {
        var ret = arg0 instanceof HTMLElement;
        return ret;
    };
    imports.wbg.__wbg_instanceof_KeyboardEvent_4d896f2a1fbf25c3 = function(arg0) {
        var ret = arg0 instanceof KeyboardEvent;
        return ret;
    };
    imports.wbg.__wbg_instanceof_HtmlInputElement_aae90057bd26cb78 = function(arg0) {
        var ret = arg0 instanceof HTMLInputElement;
        return ret;
    };
    imports.wbg.__wbg_instanceof_HtmlAnchorElement_8b74b7fd180f11b7 = function(arg0) {
        var ret = arg0 instanceof HTMLAnchorElement;
        return ret;
    };
    imports.wbg.__wbg_instanceof_HtmlButtonElement_f5c73c981d727655 = function(arg0) {
        var ret = arg0 instanceof HTMLButtonElement;
        return ret;
    };
    imports.wbg.__wbg_instanceof_HtmlTextAreaElement_2be5c0dd95f91e2f = function(arg0) {
        var ret = arg0 instanceof HTMLTextAreaElement;
        return ret;
    };
    imports.wbg.__wbg_click_23279f650dd3e83b = function(arg0) {
        arg0.click();
    };
    imports.wbg.__wbg_querySelector_e0528b8b8b25e9be = handleError(function(arg0, arg1, arg2) {
        var ret = arg0.querySelector(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    });
    imports.wbg.__wbg_preventDefault_7670dc6ff59bc226 = function(arg0) {
        arg0.preventDefault();
    };
    imports.wbg.__wbg_key_0b3d2c7a78af4571 = function(arg0, arg1) {
        var ret = arg1.key;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
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
    imports.wbg.__wbg_result_94ee1c1db21ddb63 = handleError(function(arg0) {
        var ret = arg0.result;
        return ret;
    });
    imports.wbg.__wbg_objectStoreNames_21817c1dcae9fd74 = function(arg0) {
        var ret = arg0.objectStoreNames;
        return ret;
    };
    imports.wbg.__wbg_contains_40ab2a6beed28d06 = function(arg0, arg1, arg2) {
        var ret = arg0.contains(getStringFromWasm0(arg1, arg2));
        return ret;
    };
    imports.wbg.__wbg_createObjectStore_e7fa7ae38e7c91dc = handleError(function(arg0, arg1, arg2) {
        var ret = arg0.createObjectStore(getStringFromWasm0(arg1, arg2));
        return ret;
    });
    imports.wbg.__wbg_style_ae2bb40204a83a34 = function(arg0) {
        var ret = arg0.style;
        return ret;
    };
    imports.wbg.__wbg_setProperty_4a05a7c81066031f = handleError(function(arg0, arg1, arg2, arg3, arg4) {
        arg0.setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    });
    imports.wbg.__wbg_setvalue_dc3cce23da13c2e9 = function(arg0, arg1, arg2) {
        arg0.value = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_checked_b48f7fd8d5b7330a = function(arg0) {
        var ret = arg0.checked;
        return ret;
    };
    imports.wbg.__wbg_sethidden_52638818d4866675 = function(arg0, arg1) {
        arg0.hidden = arg1 !== 0;
    };
    imports.wbg.__wbg_setreadOnly_2743cdd0fa988945 = function(arg0, arg1) {
        arg0.readOnly = arg1 !== 0;
    };
    imports.wbg.__wbg_type_dc8fd3b44b26155b = function(arg0, arg1) {
        var ret = arg1.type;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_settype_d3d5022d7fd1ef0d = function(arg0, arg1, arg2) {
        arg0.type = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_select_f8a585a1639a0ed7 = function(arg0) {
        arg0.select();
    };
    imports.wbg.__wbg_setvalue_fc815a91d9a4dec3 = function(arg0, arg1, arg2) {
        arg0.value = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_key_795ee74ba74dcdec = handleError(function(arg0) {
        var ret = arg0.key;
        return ret;
    });
    imports.wbg.__wbg_value_f8f6bdf7fbd02b0d = handleError(function(arg0) {
        var ret = arg0.value;
        return ret;
    });
    imports.wbg.__wbg_continue_c603cffbfc191c8c = handleError(function(arg0) {
        arg0.continue();
    });
    imports.wbg.__wbg_value_6d2605b80cdcbc03 = function(arg0, arg1) {
        var ret = arg1.value;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_alert_b8cc97eba2487980 = handleError(function(arg0, arg1, arg2) {
        arg0.alert(getStringFromWasm0(arg1, arg2));
    });
    imports.wbg.__wbg_objectStore_3a75ed354ae5c417 = handleError(function(arg0, arg1, arg2) {
        var ret = arg0.objectStore(getStringFromWasm0(arg1, arg2));
        return ret;
    });
    imports.wbg.__wbg_delete_ebdc2140cdb8208f = handleError(function(arg0, arg1) {
        var ret = arg0.delete(arg1);
        return ret;
    });
    imports.wbg.__wbg_setonsuccess_614caec5c13522fa = function(arg0, arg1) {
        arg0.onsuccess = arg1;
    };
    imports.wbg.__wbg_setonerror_13b4bbb71281298c = function(arg0, arg1) {
        arg0.onerror = arg1;
    };
    imports.wbg.__wbg_error_d801d33d501cc2ae = handleError(function(arg0) {
        var ret = arg0.error;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    });
    imports.wbg.__wbg_valueAsNumber_8d01fa81f95d3fa1 = function(arg0) {
        var ret = arg0.valueAsNumber;
        return ret;
    };
    imports.wbg.__wbg_value_036b553531ffb781 = function(arg0, arg1) {
        var ret = arg1.value;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_files_0f166b0ea94b6fee = function(arg0) {
        var ret = arg0.files;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_get_57006e54cc3a0582 = function(arg0, arg1) {
        var ret = arg0[arg1 >>> 0];
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_arrayBuffer_e9198c501f71094f = function(arg0) {
        var ret = arg0.arrayBuffer();
        return ret;
    };
    imports.wbg.__wbg_newwithu8arraysequence_189fa50372679ab6 = handleError(function(arg0, arg1, arg2) {
        var ret = new File(arg0, getStringFromWasm0(arg1, arg2));
        return ret;
    });
    imports.wbg.__wbg_createObjectURL_56f69e60ba5ed5a6 = handleError(function(arg0, arg1) {
        var ret = URL.createObjectURL(arg1);
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    });
    imports.wbg.__wbg_open_f0cbbf59d2c6fa42 = handleError(function(arg0, arg1, arg2) {
        var ret = arg0.open(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    });
    imports.wbg.__wbg_openCursor_5d207dba1ec8923a = handleError(function(arg0, arg1) {
        var ret = arg0.openCursor(arg1);
        return ret;
    });
    imports.wbg.__wbg_openCursor_0581ed85bf387a0b = handleError(function(arg0) {
        var ret = arg0.openCursor();
        return ret;
    });
    imports.wbg.__wbg_document_d3b6d86af1c5d199 = function(arg0) {
        var ret = arg0.document;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_indexedDB_6624a39cf12ad868 = handleError(function(arg0) {
        var ret = arg0.indexedDB;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    });
    imports.wbg.__wbg_open_5183524500cf3d25 = handleError(function(arg0, arg1, arg2) {
        var ret = arg0.open(getStringFromWasm0(arg1, arg2));
        return ret;
    });
    imports.wbg.__wbg_setonupgradeneeded_49a5c9018920d388 = function(arg0, arg1) {
        arg0.onupgradeneeded = arg1;
    };
    imports.wbg.__wbg_setdisabled_dfb251d11f5a8bb1 = function(arg0, arg1) {
        arg0.disabled = arg1 !== 0;
    };
    imports.wbg.__wbg_get_da540323a23c1d23 = handleError(function(arg0, arg1) {
        var ret = arg0.get(arg1);
        return ret;
    });
    imports.wbg.__wbg_put_7df6e52e6e0a8896 = handleError(function(arg0, arg1, arg2) {
        var ret = arg0.put(arg1, arg2);
        return ret;
    });
    imports.wbg.__wbg_setchecked_4c76d21b2d916833 = function(arg0, arg1) {
        arg0.checked = arg1 !== 0;
    };
    imports.wbg.__wbg_setvalueAsNumber_af57d858b39d9e32 = function(arg0, arg1) {
        arg0.valueAsNumber = arg1;
    };
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
    imports.wbg.__wbg_instanceof_Window_e8f84259147dce74 = function(arg0) {
        var ret = arg0 instanceof Window;
        return ret;
    };
    imports.wbg.__wbg_debug_ef2b78738889619f = typeof console.debug == 'function' ? console.debug : notDefined('console.debug');
    imports.wbg.__wbg_error_7dcc755846c00ef7 = typeof console.error == 'function' ? console.error : notDefined('console.error');
    imports.wbg.__wbg_info_43f70b84e943346e = typeof console.info == 'function' ? console.info : notDefined('console.info');
    imports.wbg.__wbg_log_61ea781bd002cc41 = typeof console.log == 'function' ? console.log : notDefined('console.log');
    imports.wbg.__wbg_warn_502e53bc79de489a = typeof console.warn == 'function' ? console.warn : notDefined('console.warn');
    imports.wbg.__wbg_transaction_112e07f255b683a6 = handleError(function(arg0, arg1, arg2, arg3) {
        var ret = arg0.transaction(getStringFromWasm0(arg1, arg2), arg3);
        return ret;
    });
    imports.wbg.__wbindgen_closure_wrapper1525 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 206, __wbg_adapter_23);
        return ret;
    };
    imports.wbg.__wbindgen_closure_wrapper1527 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 209, __wbg_adapter_26);
        return ret;
    };
    imports.wbg.__wbindgen_closure_wrapper1628 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 341, __wbg_adapter_20);
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

