const __exports = {};
import { Shape } from './snippets/wasm-test-4c9c4e6aa369d3c5/class_defined.js';

let wasm;

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

function __wbg_getmass_299a05eebd59982f(arg0) {
    return getObject(arg0).get_mass;
}

__exports.__wbg_getmass_299a05eebd59982f = __wbg_getmass_299a05eebd59982f;

function __wbg_setmass_b12d964dacbd10da(arg0, arg1) {
    getObject(arg0).mass = arg1;
}

__exports.__wbg_setmass_b12d964dacbd10da = __wbg_setmass_b12d964dacbd10da;

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

__exports.__wbindgen_throw = __wbindgen_throw;

function freeContainer(ptr) {

    wasm.__wbg_container_free(ptr);
}
/**
*/
export class Container {

    static __wrap(ptr) {
        const obj = Object.create(Container.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeContainer(ptr);
    }

    /**
    * @returns {Container}
    */
    static new() {
        return Container.__wrap(wasm.container_new());
    }
    /**
    * @param {any} one
    * @returns {void}
    */
    add_one(one) {
        return wasm.container_add_one(this.ptr, addHeapObject(one));
    }
    /**
    * @returns {void}
    */
    add_mass() {
        return wasm.container_add_mass(this.ptr);
    }
    /**
    * @returns {any}
    */
    get_first() {
        return takeObject(wasm.container_get_first(this.ptr));
    }
}

__exports.Container = Container;

function __wbindgen_object_clone_ref(idx) {
    return addHeapObject(getObject(idx));
}

__exports.__wbindgen_object_clone_ref = __wbindgen_object_clone_ref;

function __wbindgen_object_drop_ref(i) { dropObject(i); }

__exports.__wbindgen_object_drop_ref = __wbindgen_object_drop_ref;

function init(module_or_path, maybe_memory) {
    let result;
    const imports = { './wasm_test': __exports };
    if (module_or_path instanceof URL || typeof module_or_path === 'string' || module_or_path instanceof Request) {

        const response = fetch(module_or_path);
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            result = WebAssembly.instantiateStreaming(response, imports)
            .catch(e => {
                console.warn("`WebAssembly.instantiateStreaming` failed. Assuming this is because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
                return response
                .then(r => r.arrayBuffer())
                .then(bytes => WebAssembly.instantiate(bytes, imports));
            });
        } else {
            result = response
            .then(r => r.arrayBuffer())
            .then(bytes => WebAssembly.instantiate(bytes, imports));
        }
    } else {

        result = WebAssembly.instantiate(module_or_path, imports)
        .then(instance => {
            return { instance, module: module_or_path };
        });
    }
    return result.then(({instance, module}) => {
        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;

        return wasm;
    });
}

export default init;

