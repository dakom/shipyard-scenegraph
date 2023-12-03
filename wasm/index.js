(function () {
    'use strict';

    let wasm;

    const heap = new Array(128).fill(undefined);

    heap.push(undefined, null, true, false);

    function getObject(idx) { return heap[idx]; }

    let heap_next = heap.length;

    function dropObject(idx) {
        if (idx < 132) return;
        heap[idx] = heap_next;
        heap_next = idx;
    }

    function takeObject(idx) {
        const ret = getObject(idx);
        dropObject(idx);
        return ret;
    }

    function addHeapObject(obj) {
        if (heap_next === heap.length) heap.push(heap.length + 1);
        const idx = heap_next;
        heap_next = heap[idx];

        heap[idx] = obj;
        return idx;
    }

    let WASM_VECTOR_LEN = 0;

    let cachedUint8Memory0 = null;

    function getUint8Memory0() {
        if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
            cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
        }
        return cachedUint8Memory0;
    }

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
            getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
            WASM_VECTOR_LEN = buf.length;
            return ptr;
        }

        let len = arg.length;
        let ptr = malloc(len, 1) >>> 0;

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
            ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
            const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
            const ret = encodeString(arg, view);

            offset += ret.written;
        }

        WASM_VECTOR_LEN = offset;
        return ptr;
    }

    function isLikeNone(x) {
        return x === undefined || x === null;
    }

    let cachedInt32Memory0 = null;

    function getInt32Memory0() {
        if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
            cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
        }
        return cachedInt32Memory0;
    }

    const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

    if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); }
    function getStringFromWasm0(ptr, len) {
        ptr = ptr >>> 0;
        return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
    }

    let cachedFloat64Memory0 = null;

    function getFloat64Memory0() {
        if (cachedFloat64Memory0 === null || cachedFloat64Memory0.byteLength === 0) {
            cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
        }
        return cachedFloat64Memory0;
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
                    wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

                } else {
                    state.a = a;
                }
            }
        };
        real.original = state;

        return real;
    }
    function __wbg_adapter_28(arg0, arg1, arg2) {
        wasm.wasm_bindgen__convert__closures__invoke1_mut__h3036d3f81f5179b8(arg0, arg1, arg2);
    }

    let stack_pointer = 128;

    function addBorrowedObject(obj) {
        if (stack_pointer == 1) throw new Error('out of js stack');
        heap[--stack_pointer] = obj;
        return stack_pointer;
    }
    function __wbg_adapter_31(arg0, arg1, arg2) {
        try {
            wasm.wasm_bindgen__convert__closures__invoke1_mut_ref__hddadd7d4f3a0cb5a(arg0, arg1, addBorrowedObject(arg2));
        } finally {
            heap[stack_pointer++] = undefined;
        }
    }

    function __wbg_adapter_34(arg0, arg1, arg2) {
        wasm.wasm_bindgen__convert__closures__invoke1_mut__he639c58012f41cf6(arg0, arg1, addHeapObject(arg2));
    }

    function handleError(f, args) {
        try {
            return f.apply(this, args);
        } catch (e) {
            wasm.__wbindgen_exn_store(addHeapObject(e));
        }
    }

    function getArrayU8FromWasm0(ptr, len) {
        ptr = ptr >>> 0;
        return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
    }

    let cachedFloat32Memory0 = null;

    function getFloat32Memory0() {
        if (cachedFloat32Memory0 === null || cachedFloat32Memory0.byteLength === 0) {
            cachedFloat32Memory0 = new Float32Array(wasm.memory.buffer);
        }
        return cachedFloat32Memory0;
    }

    function getArrayF32FromWasm0(ptr, len) {
        ptr = ptr >>> 0;
        return getFloat32Memory0().subarray(ptr / 4, ptr / 4 + len);
    }

    async function __wbg_load(module, imports) {
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

    function __wbg_get_imports() {
        const imports = {};
        imports.wbg = {};
        imports.wbg.__wbg_bindFramebuffer_33174ee82d938627 = function(arg0, arg1, arg2) {
            getObject(arg0).bindFramebuffer(arg1 >>> 0, getObject(arg2));
        };
        imports.wbg.__wbg_readPixels_2ca1e2963e797936 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) {
            getObject(arg0).readPixels(arg1, arg2, arg3, arg4, arg5 >>> 0, arg6 >>> 0, arg7 === 0 ? undefined : getArrayU8FromWasm0(arg7, arg8));
        }, arguments) };
        imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
            takeObject(arg0);
        };
        imports.wbg.__wbindgen_cb_drop = function(arg0) {
            const obj = takeObject(arg0).original;
            if (obj.cnt-- == 1) {
                obj.a = 0;
                return true;
            }
            const ret = false;
            return ret;
        };
        imports.wbg.__wbg_abort_2c4fb490d878d2b2 = function(arg0) {
            getObject(arg0).abort();
        };
        imports.wbg.__wbg_bindBuffer_c71ed62c7c21bed0 = function(arg0, arg1, arg2) {
            getObject(arg0).bindBuffer(arg1 >>> 0, getObject(arg2));
        };
        imports.wbg.__wbindgen_memory = function() {
            const ret = wasm.memory;
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_buffer_a448f833075b71ba = function(arg0) {
            const ret = getObject(arg0).buffer;
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_newwithbyteoffsetandlength_fa811509d2a67254 = function(arg0, arg1, arg2) {
            const ret = new Float32Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_bufferData_11f5ff31cb447750 = function(arg0, arg1, arg2, arg3) {
            getObject(arg0).bufferData(arg1 >>> 0, getObject(arg2), arg3 >>> 0);
        };
        imports.wbg.__wbg_vertexAttribPointer_fd7fa402160fe558 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
            getObject(arg0).vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
        };
        imports.wbg.__wbg_enableVertexAttribArray_f8678d164c294659 = function(arg0, arg1) {
            getObject(arg0).enableVertexAttribArray(arg1 >>> 0);
        };
        imports.wbg.__wbg_createShader_25391a4dceb30291 = function(arg0, arg1) {
            const ret = getObject(arg0).createShader(arg1 >>> 0);
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        };
        imports.wbg.__wbg_shaderSource_8581035b723a56a7 = function(arg0, arg1, arg2, arg3) {
            getObject(arg0).shaderSource(getObject(arg1), getStringFromWasm0(arg2, arg3));
        };
        imports.wbg.__wbg_compileShader_df38c9b4d109df2c = function(arg0, arg1) {
            getObject(arg0).compileShader(getObject(arg1));
        };
        imports.wbg.__wbg_getShaderParameter_d5af258ca8110f13 = function(arg0, arg1, arg2) {
            const ret = getObject(arg0).getShaderParameter(getObject(arg1), arg2 >>> 0);
            return addHeapObject(ret);
        };
        imports.wbg.__wbindgen_boolean_get = function(arg0) {
            const v = getObject(arg0);
            const ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
            return ret;
        };
        imports.wbg.__wbg_getShaderInfoLog_207d91c9201acffa = function(arg0, arg1, arg2) {
            const ret = getObject(arg1).getShaderInfoLog(getObject(arg2));
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len1;
            getInt32Memory0()[arg0 / 4 + 0] = ptr1;
        };
        imports.wbg.__wbg_createBuffer_993ecd2e92aabe3c = function(arg0) {
            const ret = getObject(arg0).createBuffer();
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        };
        imports.wbg.__wbg_createTexture_fc71efc6d11fdbcb = function(arg0) {
            const ret = getObject(arg0).createTexture();
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        };
        imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
            const ret = getObject(arg0);
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_text_668782292b0bc561 = function() { return handleError(function (arg0) {
            const ret = getObject(arg0).text();
            return addHeapObject(ret);
        }, arguments) };
        imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
            const obj = getObject(arg1);
            const ret = typeof(obj) === 'string' ? obj : undefined;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len1;
            getInt32Memory0()[arg0 / 4 + 0] = ptr1;
        };
        imports.wbg.__wbg_newwithstr_8aa8479760b1e560 = function() { return handleError(function (arg0, arg1) {
            const ret = new Request(getStringFromWasm0(arg0, arg1));
            return addHeapObject(ret);
        }, arguments) };
        imports.wbg.__wbg_new_9fb8d994e1c0aaac = function() {
            const ret = new Object();
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_new_0ae46f44b7485bb2 = function() { return handleError(function () {
            const ret = new AbortController();
            return addHeapObject(ret);
        }, arguments) };
        imports.wbg.__wbg_signal_3c701f5f40a5f08d = function(arg0) {
            const ret = getObject(arg0).signal;
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_instanceof_Window_3e5cd1f48c152d01 = function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof Window;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        };
        imports.wbg.__wbg_fetch_0825c1e50075ecad = function(arg0, arg1, arg2) {
            const ret = getObject(arg0).fetch(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_clearColor_253d33d2ea420d1a = function(arg0, arg1, arg2, arg3, arg4) {
            getObject(arg0).clearColor(arg1, arg2, arg3, arg4);
        };
        imports.wbg.__wbg_clear_5e252567fe8f1763 = function(arg0, arg1) {
            getObject(arg0).clear(arg1 >>> 0);
        };
        imports.wbg.__wbg_enable_e39f53a946b9e3a0 = function(arg0, arg1) {
            getObject(arg0).enable(arg1 >>> 0);
        };
        imports.wbg.__wbg_blendFunc_3edf09b56fbb3ffd = function(arg0, arg1, arg2) {
            getObject(arg0).blendFunc(arg1 >>> 0, arg2 >>> 0);
        };
        imports.wbg.__wbg_useProgram_667ebfb0fb0de4c0 = function(arg0, arg1) {
            getObject(arg0).useProgram(getObject(arg1));
        };
        imports.wbg.__wbg_width_cfc58d9656d60465 = function(arg0) {
            const ret = getObject(arg0).width;
            return ret;
        };
        imports.wbg.__wbg_height_1ba9072bd4001d19 = function(arg0) {
            const ret = getObject(arg0).height;
            return ret;
        };
        imports.wbg.__wbg_uniformMatrix4fv_47822ae94c519f11 = function(arg0, arg1, arg2, arg3, arg4) {
            getObject(arg0).uniformMatrix4fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
        };
        imports.wbg.__wbg_uniform4f_83cd1c05881edfde = function(arg0, arg1, arg2, arg3, arg4, arg5) {
            getObject(arg0).uniform4f(getObject(arg1), arg2, arg3, arg4, arg5);
        };
        imports.wbg.__wbg_drawArrays_4ae5359a7c3c5279 = function(arg0, arg1, arg2, arg3) {
            getObject(arg0).drawArrays(arg1 >>> 0, arg2, arg3);
        };
        imports.wbg.__wbg_uniform2f_07d7d700bb962bb6 = function(arg0, arg1, arg2, arg3) {
            getObject(arg0).uniform2f(getObject(arg1), arg2, arg3);
        };
        imports.wbg.__wbg_document_d609202d16c38224 = function(arg0) {
            const ret = getObject(arg0).document;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        };
        imports.wbg.__wbg_body_64abc9aba1891e91 = function(arg0) {
            const ret = getObject(arg0).body;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        };
        imports.wbg.__wbg_getElementById_65b9547a428b5eb4 = function(arg0, arg1, arg2) {
            const ret = getObject(arg0).getElementById(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        };
        imports.wbg.__wbg_instanceof_HtmlCanvasElement_fba0ac991170cc00 = function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof HTMLCanvasElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        };
        imports.wbg.__wbg_set_8761474ad72b9bf1 = function(arg0, arg1, arg2) {
            getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
        };
        imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
            const ret = getStringFromWasm0(arg0, arg1);
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_getContext_52cc019050c5f7bd = function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2), getObject(arg3));
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        }, arguments) };
        imports.wbg.__wbg_instanceof_WebGlRenderingContext_275e63499db8b412 = function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof WebGLRenderingContext;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        };
        imports.wbg.__wbg_canvas_6494c67f5f624a96 = function(arg0) {
            const ret = getObject(arg0).canvas;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        };
        imports.wbg.__wbg_getParameter_fedfba9017d5fbcd = function() { return handleError(function (arg0, arg1) {
            const ret = getObject(arg0).getParameter(arg1 >>> 0);
            return addHeapObject(ret);
        }, arguments) };
        imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
            const obj = getObject(arg1);
            const ret = typeof(obj) === 'number' ? obj : undefined;
            getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
            getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
        };
        imports.wbg.__wbg_length_7210e87a3c22e007 = function(arg0) {
            const ret = getObject(arg0).length;
            return ret;
        };
        imports.wbg.__wbg_new_c8ddc4aa69d56bd7 = function(arg0) {
            const ret = new Float32Array(getObject(arg0));
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_set_db717188fc75d58f = function(arg0, arg1, arg2) {
            getObject(arg0).set(getObject(arg1), arg2 >>> 0);
        };
        imports.wbg.__wbg_getExtension_f109da65b46fbeb8 = function() { return handleError(function (arg0, arg1, arg2) {
            const ret = getObject(arg0).getExtension(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        }, arguments) };
        imports.wbg.__wbg_addEventListener_374cbfd2bbc19ccf = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3), getObject(arg4));
        }, arguments) };
        imports.wbg.__wbg_setwidth_7591ce24118fd14a = function(arg0, arg1) {
            getObject(arg0).width = arg1 >>> 0;
        };
        imports.wbg.__wbg_setheight_f7ae862183d88bd5 = function(arg0, arg1) {
            getObject(arg0).height = arg1 >>> 0;
        };
        imports.wbg.__wbg_viewport_2464c396536924a3 = function(arg0, arg1, arg2, arg3, arg4) {
            getObject(arg0).viewport(arg1, arg2, arg3, arg4);
        };
        imports.wbg.__wbg_deleteFramebuffer_2cc015c1b281e8a1 = function(arg0, arg1) {
            getObject(arg0).deleteFramebuffer(getObject(arg1));
        };
        imports.wbg.__wbg_bindRenderbuffer_0c7738e79a575fdb = function(arg0, arg1, arg2) {
            getObject(arg0).bindRenderbuffer(arg1 >>> 0, getObject(arg2));
        };
        imports.wbg.__wbg_deleteRenderbuffer_80239f946eea133d = function(arg0, arg1) {
            getObject(arg0).deleteRenderbuffer(getObject(arg1));
        };
        imports.wbg.__wbg_deleteTexture_b8458a96b71a0a04 = function(arg0, arg1) {
            getObject(arg0).deleteTexture(getObject(arg1));
        };
        imports.wbg.__wbg_createFramebuffer_1b0ce659f44b2562 = function(arg0) {
            const ret = getObject(arg0).createFramebuffer();
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        };
        imports.wbg.__wbg_createRenderbuffer_cd7b8379638f7c64 = function(arg0) {
            const ret = getObject(arg0).createRenderbuffer();
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        };
        imports.wbg.__wbg_renderbufferStorage_fcb8aee479a5dd50 = function(arg0, arg1, arg2, arg3, arg4) {
            getObject(arg0).renderbufferStorage(arg1 >>> 0, arg2 >>> 0, arg3, arg4);
        };
        imports.wbg.__wbg_framebufferRenderbuffer_09dddaeb9b013985 = function(arg0, arg1, arg2, arg3, arg4) {
            getObject(arg0).framebufferRenderbuffer(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, getObject(arg4));
        };
        imports.wbg.__wbg_bindTexture_df13ba7e7ee5d984 = function(arg0, arg1, arg2) {
            getObject(arg0).bindTexture(arg1 >>> 0, getObject(arg2));
        };
        imports.wbg.__wbg_pixelStorei_48bb580e625ac760 = function(arg0, arg1, arg2) {
            getObject(arg0).pixelStorei(arg1 >>> 0, arg2);
        };
        imports.wbg.__wbg_texParameteri_fba016345d388fd9 = function(arg0, arg1, arg2, arg3) {
            getObject(arg0).texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);
        };
        imports.wbg.__wbg_texImage2D_50acf334022f586d = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
            getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, getObject(arg9));
        }, arguments) };
        imports.wbg.__wbg_framebufferTexture2D_8d99f62eee2d1757 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
            getObject(arg0).framebufferTexture2D(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, getObject(arg4), arg5);
        };
        imports.wbg.__wbg_checkFramebufferStatus_b9d159787168ba02 = function(arg0, arg1) {
            const ret = getObject(arg0).checkFramebufferStatus(arg1 >>> 0);
            return ret;
        };
        imports.wbg.__wbg_createElement_fdd5c113cb84539e = function() { return handleError(function (arg0, arg1, arg2) {
            const ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
            return addHeapObject(ret);
        }, arguments) };
        imports.wbg.__wbg_instanceof_HtmlElement_55a0f0f0f0f0118e = function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof HTMLElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        };
        imports.wbg.__wbg_setclassName_4b720df7b12cb674 = function(arg0, arg1, arg2) {
            getObject(arg0).className = getStringFromWasm0(arg1, arg2);
        };
        imports.wbg.__wbg_appendChild_d30e6b83791d04c0 = function() { return handleError(function (arg0, arg1) {
            const ret = getObject(arg0).appendChild(getObject(arg1));
            return addHeapObject(ret);
        }, arguments) };
        imports.wbg.__wbg_settextContent_3ebccdd9354e1601 = function(arg0, arg1, arg2) {
            getObject(arg0).textContent = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
        };
        imports.wbg.__wbg_instanceof_MouseEvent_358e915c233a0f19 = function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof MouseEvent;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        };
        imports.wbg.__wbg_getBoundingClientRect_4167ccfa40cf88fc = function(arg0) {
            const ret = getObject(arg0).getBoundingClientRect();
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_clientX_1a01963cb1caa614 = function(arg0) {
            const ret = getObject(arg0).clientX;
            return ret;
        };
        imports.wbg.__wbg_clientY_c370190d4150fba9 = function(arg0) {
            const ret = getObject(arg0).clientY;
            return ret;
        };
        imports.wbg.__wbg_left_253905e6c5050382 = function(arg0) {
            const ret = getObject(arg0).left;
            return ret;
        };
        imports.wbg.__wbg_top_da9f6cf2fb5b9968 = function(arg0) {
            const ret = getObject(arg0).top;
            return ret;
        };
        imports.wbg.__wbg_instanceof_KeyboardEvent_a5528292f229cba6 = function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof KeyboardEvent;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        };
        imports.wbg.__wbg_code_878e76a4ddb70157 = function(arg0, arg1) {
            const ret = getObject(arg1).code;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len1;
            getInt32Memory0()[arg0 / 4 + 0] = ptr1;
        };
        imports.wbg.__wbg_instanceof_WheelEvent_ebec41eb17e648ee = function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof WheelEvent;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        };
        imports.wbg.__wbg_deltaMode_5f43eb63f3077df7 = function(arg0) {
            const ret = getObject(arg0).deltaMode;
            return ret;
        };
        imports.wbg.__wbg_deltaX_03d8f6dcd2e14b63 = function(arg0) {
            const ret = getObject(arg0).deltaX;
            return ret;
        };
        imports.wbg.__wbg_deltaY_7d9a7eb25f83e193 = function(arg0) {
            const ret = getObject(arg0).deltaY;
            return ret;
        };
        imports.wbg.__wbg_deltaZ_3556e726737898b1 = function(arg0) {
            const ret = getObject(arg0).deltaZ;
            return ret;
        };
        imports.wbg.__wbg_get_7b48513de5dc5ea4 = function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.get(getObject(arg0), getObject(arg1));
            return addHeapObject(ret);
        }, arguments) };
        imports.wbg.__wbg_requestAnimationFrame_74309aadebde12fa = function() { return handleError(function (arg0, arg1) {
            const ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
            return ret;
        }, arguments) };
        imports.wbg.__wbg_createProgram_76ddcf5596a96a1a = function(arg0) {
            const ret = getObject(arg0).createProgram();
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        };
        imports.wbg.__wbg_bindAttribLocation_fb56e526aef3e55a = function(arg0, arg1, arg2, arg3, arg4) {
            getObject(arg0).bindAttribLocation(getObject(arg1), arg2 >>> 0, getStringFromWasm0(arg3, arg4));
        };
        imports.wbg.__wbg_attachShader_289e2f1d24149257 = function(arg0, arg1, arg2) {
            getObject(arg0).attachShader(getObject(arg1), getObject(arg2));
        };
        imports.wbg.__wbg_linkProgram_1ab5d0990c565f87 = function(arg0, arg1) {
            getObject(arg0).linkProgram(getObject(arg1));
        };
        imports.wbg.__wbg_getProgramParameter_ac16a850d3f251f3 = function(arg0, arg1, arg2) {
            const ret = getObject(arg0).getProgramParameter(getObject(arg1), arg2 >>> 0);
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_getProgramInfoLog_99334d62bea10332 = function(arg0, arg1, arg2) {
            const ret = getObject(arg1).getProgramInfoLog(getObject(arg2));
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len1;
            getInt32Memory0()[arg0 / 4 + 0] = ptr1;
        };
        imports.wbg.__wbg_detachShader_bef8220f8d13b6f7 = function(arg0, arg1, arg2) {
            getObject(arg0).detachShader(getObject(arg1), getObject(arg2));
        };
        imports.wbg.__wbg_deleteProgram_ffe51c2159e56aeb = function(arg0, arg1) {
            getObject(arg0).deleteProgram(getObject(arg1));
        };
        imports.wbg.__wbg_innerWidth_e5d865919c14bdf9 = function() { return handleError(function (arg0) {
            const ret = getObject(arg0).innerWidth;
            return addHeapObject(ret);
        }, arguments) };
        imports.wbg.__wbg_innerHeight_5e414ce6ae3fd139 = function() { return handleError(function (arg0) {
            const ret = getObject(arg0).innerHeight;
            return addHeapObject(ret);
        }, arguments) };
        imports.wbg.__wbg_getAttribLocation_b08751ecb11d328d = function(arg0, arg1, arg2, arg3) {
            const ret = getObject(arg0).getAttribLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
            return ret;
        };
        imports.wbg.__wbg_getUniformLocation_29cc1018d110f9f0 = function(arg0, arg1, arg2, arg3) {
            const ret = getObject(arg0).getUniformLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        };
        imports.wbg.__wbg_self_f0e34d89f33b99fd = function() { return handleError(function () {
            const ret = self.self;
            return addHeapObject(ret);
        }, arguments) };
        imports.wbg.__wbg_window_d3b084224f4774d7 = function() { return handleError(function () {
            const ret = window.window;
            return addHeapObject(ret);
        }, arguments) };
        imports.wbg.__wbg_globalThis_9caa27ff917c6860 = function() { return handleError(function () {
            const ret = globalThis.globalThis;
            return addHeapObject(ret);
        }, arguments) };
        imports.wbg.__wbg_global_35dfdd59a4da3e74 = function() { return handleError(function () {
            const ret = global.global;
            return addHeapObject(ret);
        }, arguments) };
        imports.wbg.__wbindgen_is_undefined = function(arg0) {
            const ret = getObject(arg0) === undefined;
            return ret;
        };
        imports.wbg.__wbg_newnoargs_c62ea9419c21fbac = function(arg0, arg1) {
            const ret = new Function(getStringFromWasm0(arg0, arg1));
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_call_90c26b09837aba1c = function() { return handleError(function (arg0, arg1) {
            const ret = getObject(arg0).call(getObject(arg1));
            return addHeapObject(ret);
        }, arguments) };
        imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
            const ret = debugString(getObject(arg1));
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len1;
            getInt32Memory0()[arg0 / 4 + 0] = ptr1;
        };
        imports.wbg.__wbindgen_throw = function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        };
        imports.wbg.__wbindgen_rethrow = function(arg0) {
            throw takeObject(arg0);
        };
        imports.wbg.__wbg_then_8371cc12cfedc5a2 = function(arg0, arg1, arg2) {
            const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_queueMicrotask_adae4bc085237231 = function(arg0) {
            const ret = getObject(arg0).queueMicrotask;
            return addHeapObject(ret);
        };
        imports.wbg.__wbindgen_is_function = function(arg0) {
            const ret = typeof(getObject(arg0)) === 'function';
            return ret;
        };
        imports.wbg.__wbg_resolve_6e1c6553a82f85b7 = function(arg0) {
            const ret = Promise.resolve(getObject(arg0));
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_then_3ab08cd4fbb91ae9 = function(arg0, arg1) {
            const ret = getObject(arg0).then(getObject(arg1));
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_queueMicrotask_4d890031a6a5a50c = function(arg0) {
            queueMicrotask(getObject(arg0));
        };
        imports.wbg.__wbg_set_759f75cd92b612d2 = function() { return handleError(function (arg0, arg1, arg2) {
            const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
            return ret;
        }, arguments) };
        imports.wbg.__wbg_new_7754f544295e21a9 = function() { return handleError(function (arg0, arg1) {
            const ret = new Event(getStringFromWasm0(arg0, arg1));
            return addHeapObject(ret);
        }, arguments) };
        imports.wbg.__wbindgen_closure_wrapper175 = function(arg0, arg1, arg2) {
            const ret = makeMutClosure(arg0, arg1, 9, __wbg_adapter_28);
            return addHeapObject(ret);
        };
        imports.wbg.__wbindgen_closure_wrapper177 = function(arg0, arg1, arg2) {
            const ret = makeMutClosure(arg0, arg1, 9, __wbg_adapter_31);
            return addHeapObject(ret);
        };
        imports.wbg.__wbindgen_closure_wrapper1246 = function(arg0, arg1, arg2) {
            const ret = makeMutClosure(arg0, arg1, 9, __wbg_adapter_34);
            return addHeapObject(ret);
        };

        return imports;
    }

    function __wbg_finalize_init(instance, module) {
        wasm = instance.exports;
        __wbg_init.__wbindgen_wasm_module = module;
        cachedFloat32Memory0 = null;
        cachedFloat64Memory0 = null;
        cachedInt32Memory0 = null;
        cachedUint8Memory0 = null;

        wasm.__wbindgen_start();
        return wasm;
    }

    async function __wbg_init(input) {
        if (wasm !== undefined) return wasm;

        if (typeof input === 'undefined') {
            input = new URL('index_bg.wasm', (document.currentScript && document.currentScript.src || new URL('index.js', document.baseURI).href));
        }
        const imports = __wbg_get_imports();

        if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
            input = fetch(input);
        }

        const { instance, module } = await __wbg_load(await input, imports);

        return __wbg_finalize_init(instance, module);
    }

    __wbg_init("wasm/assets/demo-d01d4651.wasm").catch(console.error);

})();
//# sourceMappingURL=index.js.map
