import * as wasm from './titso_web_bg.wasm';

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

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

const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

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
    wasm.closure120_externref_shim(arg0, arg1, arg2);
}

function __wbg_adapter_23(arg0, arg1, arg2) {
    wasm.closure325_externref_shim(arg0, arg1, arg2);
}

function __wbg_adapter_26(arg0, arg1) {
    wasm._dyn_core_11e4cce1c6ea371b___ops__function__FnMut_____Output______as_wasm_bindgen_f16b82f13c07af1___closure__WasmClosure___describe__invoke______(arg0, arg1);
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

export const __wbg_new_59cb74e423758ede = function() {
    var ret = new Error();
    return ret;
};

export const __wbg_stack_558ba5917b466edd = function(arg0, arg1) {
    var ret = arg1.stack;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export const __wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
    try {
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(arg0, arg1);
    }
};

export const __wbindgen_string_new = function(arg0, arg1) {
    var ret = getStringFromWasm0(arg0, arg1);
    return ret;
};

export const __wbg_self_1b7a39e3a92c949c = handleError(function() {
    var ret = self.self;
    return ret;
});

export const __wbg_require_604837428532a733 = function(arg0, arg1) {
    var ret = require(getStringFromWasm0(arg0, arg1));
    return ret;
};

export const __wbg_crypto_968f1772287e2df0 = function(arg0) {
    var ret = arg0.crypto;
    return ret;
};

export const __wbindgen_is_undefined = function(arg0) {
    var ret = arg0 === undefined;
    return ret;
};

export const __wbg_getRandomValues_a3d34b4fee3c2869 = function(arg0) {
    var ret = arg0.getRandomValues;
    return ret;
};

export const __wbg_getRandomValues_f5e14ab7ac8e995d = function(arg0, arg1, arg2) {
    arg0.getRandomValues(getArrayU8FromWasm0(arg1, arg2));
};

export const __wbg_randomFillSync_d5bd2d655fdf256a = function(arg0, arg1, arg2) {
    arg0.randomFillSync(getArrayU8FromWasm0(arg1, arg2));
};

export const __wbg_new_8172f4fed77fdb7c = function() {
    var ret = new Object();
    return ret;
};

export const __wbg_new_17534eac4df3cd22 = function() {
    var ret = new Array();
    return ret;
};

export const __wbg_push_7114ccbf1c58e41f = function(arg0, arg1) {
    var ret = arg0.push(arg1);
    return ret;
};

export const __wbg_byteLength_4024fc066aa7659d = function(arg0) {
    var ret = arg0.byteLength;
    return ret;
};

export const __wbg_call_e9f0ce4da840ab94 = handleError(function(arg0, arg1) {
    var ret = arg0.call(arg1);
    return ret;
});

export const __wbg_resolve_4df26938859b92e3 = function(arg0) {
    var ret = Promise.resolve(arg0);
    return ret;
};

export const __wbg_then_ffb6e71f7a6735ad = function(arg0, arg1) {
    var ret = arg0.then(arg1);
    return ret;
};

export const __wbg_self_179e8c2a5a4c73a3 = handleError(function() {
    var ret = self.self;
    return ret;
});

export const __wbg_window_492cfe63a6e41dfa = handleError(function() {
    var ret = window.window;
    return ret;
});

export const __wbg_globalThis_8ebfea75c2dd63ee = handleError(function() {
    var ret = globalThis.globalThis;
    return ret;
});

export const __wbg_global_62ea2619f58bf94d = handleError(function() {
    var ret = global.global;
    return ret;
});

export const __wbg_newnoargs_e2fdfe2af14a2323 = function(arg0, arg1) {
    var ret = new Function(getStringFromWasm0(arg0, arg1));
    return ret;
};

export const __wbindgen_memory = function() {
    var ret = wasm.memory;
    return ret;
};

export const __wbg_buffer_88f603259d7a7b82 = function(arg0) {
    var ret = arg0.buffer;
    return ret;
};

export const __wbg_newwithbyteoffsetandlength_a048d126789a272b = function(arg0, arg1, arg2) {
    var ret = new Uint8Array(arg0, arg1 >>> 0, arg2 >>> 0);
    return ret;
};

export const __wbg_new_85d8a1fc4384acef = function(arg0) {
    var ret = new Uint8Array(arg0);
    return ret;
};

export const __wbg_set_478951586c457484 = function(arg0, arg1, arg2) {
    arg0.set(arg1, arg2 >>> 0);
};

export const __wbg_length_2e98733d73dac355 = function(arg0) {
    var ret = arg0.length;
    return ret;
};

export const __wbg_byteLength_eaa4a2fa4e78c5ae = function(arg0) {
    var ret = arg0.byteLength;
    return ret;
};

export const __wbg_set_afe54b1eeb1aa77c = handleError(function(arg0, arg1, arg2) {
    var ret = Reflect.set(arg0, arg1, arg2);
    return ret;
});

export const __wbg_target_9d8c7027b8164233 = function(arg0) {
    var ret = arg0.target;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export const __wbg_instanceof_IdbRequest_b09c6f42093b9e91 = function(arg0) {
    var ret = arg0 instanceof IDBRequest;
    return ret;
};

export const __wbg_result_94ee1c1db21ddb63 = handleError(function(arg0) {
    var ret = arg0.result;
    return ret;
});

export const __wbg_objectStoreNames_21817c1dcae9fd74 = function(arg0) {
    var ret = arg0.objectStoreNames;
    return ret;
};

export const __wbg_length_c7d783c6f529f712 = function(arg0) {
    var ret = arg0.length;
    return ret;
};

export const __wbg_version_1c7420256ebc4ecb = function(arg0) {
    var ret = arg0.version;
    return ret;
};

export const __wbg_createObjectStore_e7fa7ae38e7c91dc = handleError(function(arg0, arg1, arg2) {
    var ret = arg0.createObjectStore(getStringFromWasm0(arg1, arg2));
    return ret;
});

export const __wbg_instanceof_IdbDatabase_755c903a284da531 = function(arg0) {
    var ret = arg0 instanceof IDBDatabase;
    return ret;
};

export const __wbg_indexedDB_6624a39cf12ad868 = handleError(function(arg0) {
    var ret = arg0.indexedDB;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
});

export const __wbg_open_5183524500cf3d25 = handleError(function(arg0, arg1, arg2) {
    var ret = arg0.open(getStringFromWasm0(arg1, arg2));
    return ret;
});

export const __wbg_open_55860ad246a8deb1 = handleError(function(arg0, arg1, arg2, arg3) {
    var ret = arg0.open(getStringFromWasm0(arg1, arg2), arg3 >>> 0);
    return ret;
});

export const __wbg_setonupgradeneeded_49a5c9018920d388 = function(arg0, arg1) {
    arg0.onupgradeneeded = arg1;
};

export const __wbg_setonsuccess_614caec5c13522fa = function(arg0, arg1) {
    arg0.onsuccess = arg1;
};

export const __wbg_bound_619a6a3bce79bd49 = handleError(function(arg0, arg1) {
    var ret = IDBKeyRange.bound(arg0, arg1);
    return ret;
});

export const __wbg_delete_ebdc2140cdb8208f = handleError(function(arg0, arg1) {
    var ret = arg0.delete(arg1);
    return ret;
});

export const __wbg_put_7df6e52e6e0a8896 = handleError(function(arg0, arg1, arg2) {
    var ret = arg0.put(arg1, arg2);
    return ret;
});

export const __wbg_setoncomplete_5b1258b21437f4e8 = function(arg0, arg1) {
    arg0.oncomplete = arg1;
};

export const __wbg_setonerror_f1f902c6482ce20e = function(arg0, arg1) {
    arg0.onerror = arg1;
};

export const __wbg_transaction_8302a2efa3050f90 = handleError(function(arg0, arg1, arg2) {
    var ret = arg0.transaction(getStringFromWasm0(arg1, arg2));
    return ret;
});

export const __wbg_objectStore_3a75ed354ae5c417 = handleError(function(arg0, arg1, arg2) {
    var ret = arg0.objectStore(getStringFromWasm0(arg1, arg2));
    return ret;
});

export const __wbg_openCursor_0581ed85bf387a0b = handleError(function(arg0) {
    var ret = arg0.openCursor();
    return ret;
});

export const __wbg_key_795ee74ba74dcdec = handleError(function(arg0) {
    var ret = arg0.key;
    return ret;
});

export const __wbg_value_f8f6bdf7fbd02b0d = handleError(function(arg0) {
    var ret = arg0.value;
    return ret;
});

export const __wbg_continue_c603cffbfc191c8c = handleError(function(arg0) {
    arg0.continue();
});

export const __wbg_close_f143d62c258f91a2 = function(arg0) {
    arg0.close();
};

export const __wbg_value_6d2605b80cdcbc03 = function(arg0, arg1) {
    var ret = arg1.value;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export const __wbg_alert_b8cc97eba2487980 = handleError(function(arg0, arg1, arg2) {
    arg0.alert(getStringFromWasm0(arg1, arg2));
});

export const __wbg_setvalue_dc3cce23da13c2e9 = function(arg0, arg1, arg2) {
    arg0.value = getStringFromWasm0(arg1, arg2);
};

export const __wbg_sethidden_52638818d4866675 = function(arg0, arg1) {
    arg0.hidden = arg1 !== 0;
};

export const __wbg_focus_f5a9f4b6a353d1d1 = handleError(function(arg0) {
    arg0.focus();
});

export const __wbg_setchecked_4c76d21b2d916833 = function(arg0, arg1) {
    arg0.checked = arg1 !== 0;
};

export const __wbg_setvalueAsNumber_af57d858b39d9e32 = function(arg0, arg1) {
    arg0.valueAsNumber = arg1;
};

export const __wbg_setvalue_fc815a91d9a4dec3 = function(arg0, arg1, arg2) {
    arg0.value = getStringFromWasm0(arg1, arg2);
};

export const __wbg_checked_b48f7fd8d5b7330a = function(arg0) {
    var ret = arg0.checked;
    return ret;
};

export const __wbg_valueAsNumber_8d01fa81f95d3fa1 = function(arg0) {
    var ret = arg0.valueAsNumber;
    return ret;
};

export const __wbg_value_036b553531ffb781 = function(arg0, arg1) {
    var ret = arg1.value;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export const __wbg_type_dc8fd3b44b26155b = function(arg0, arg1) {
    var ret = arg1.type;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export const __wbg_settype_d3d5022d7fd1ef0d = function(arg0, arg1, arg2) {
    arg0.type = getStringFromWasm0(arg1, arg2);
};

export const __wbg_select_f8a585a1639a0ed7 = function(arg0) {
    arg0.select();
};

export const __wbg_instanceof_HtmlElement_773e85b6bd68ae2d = function(arg0) {
    var ret = arg0 instanceof HTMLElement;
    return ret;
};

export const __wbg_instanceof_HtmlInputElement_aae90057bd26cb78 = function(arg0) {
    var ret = arg0 instanceof HTMLInputElement;
    return ret;
};

export const __wbg_instanceof_HtmlButtonElement_f5c73c981d727655 = function(arg0) {
    var ret = arg0 instanceof HTMLButtonElement;
    return ret;
};

export const __wbg_instanceof_HtmlTextAreaElement_2be5c0dd95f91e2f = function(arg0) {
    var ret = arg0 instanceof HTMLTextAreaElement;
    return ret;
};

export const __wbg_document_d3b6d86af1c5d199 = function(arg0) {
    var ret = arg0.document;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export const __wbg_addEventListener_27eb43df29303d67 = handleError(function(arg0, arg1, arg2, arg3, arg4) {
    arg0.addEventListener(getStringFromWasm0(arg1, arg2), arg3, arg4);
});

export const __wbindgen_cb_drop = function(arg0) {
    const obj = arg0.original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return true;
    }
    var ret = false;
    return ret;
};

export const __wbg_instanceof_KeyboardEvent_4d896f2a1fbf25c3 = function(arg0) {
    var ret = arg0 instanceof KeyboardEvent;
    return ret;
};

export const __wbg_querySelector_e0528b8b8b25e9be = handleError(function(arg0, arg1, arg2) {
    var ret = arg0.querySelector(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
});

export const __wbg_key_0b3d2c7a78af4571 = function(arg0, arg1) {
    var ret = arg1.key;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export const __wbindgen_debug_string = function(arg0, arg1) {
    var ret = debugString(arg1);
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export const __wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

export const __wbg_instanceof_Window_e8f84259147dce74 = function(arg0) {
    var ret = arg0 instanceof Window;
    return ret;
};

export const __wbg_transaction_ea6d3c7923959ad8 = handleError(function(arg0, arg1, arg2) {
    var ret = arg0.transaction(arg1, arg2);
    return ret;
});

export const __wbg_debug_ef2b78738889619f = typeof console.debug == 'function' ? console.debug : notDefined('console.debug');

export const __wbg_error_7dcc755846c00ef7 = typeof console.error == 'function' ? console.error : notDefined('console.error');

export const __wbg_info_43f70b84e943346e = typeof console.info == 'function' ? console.info : notDefined('console.info');

export const __wbg_log_61ea781bd002cc41 = typeof console.log == 'function' ? console.log : notDefined('console.log');

export const __wbg_warn_502e53bc79de489a = typeof console.warn == 'function' ? console.warn : notDefined('console.warn');

export const __wbindgen_closure_wrapper1132 = function(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 121, __wbg_adapter_20);
    return ret;
};

export const __wbindgen_closure_wrapper1709 = function(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 326, __wbg_adapter_23);
    return ret;
};

export const __wbindgen_closure_wrapper1131 = function(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 123, __wbg_adapter_26);
    return ret;
};

export const __wbindgen_init_externref_table = function() {
    const table = wasm.__wbindgen_export_2;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};

