const __exports = {};

let wasm;

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

function __wbg_getcoordinatex_4bcbe9b0616923e9(arg0) {
    return getObject(arg0).get_coordinate_x;
}

__exports.__wbg_getcoordinatex_4bcbe9b0616923e9 = __wbg_getcoordinatex_4bcbe9b0616923e9;

function __wbg_getcoordinatey_054282542362d71c(arg0) {
    return getObject(arg0).get_coordinate_y;
}

__exports.__wbg_getcoordinatey_054282542362d71c = __wbg_getcoordinatey_054282542362d71c;

function __wbg_getmass_745f0a2975ce4b09(arg0) {
    return getObject(arg0).get_mass;
}

__exports.__wbg_getmass_745f0a2975ce4b09 = __wbg_getmass_745f0a2975ce4b09;

function __wbg_getvelocityx_b92b57e439408404(arg0) {
    return getObject(arg0).get_velocity_x;
}

__exports.__wbg_getvelocityx_b92b57e439408404 = __wbg_getvelocityx_b92b57e439408404;

function __wbg_getvelocityy_78a22e6687f6ced5(arg0) {
    return getObject(arg0).get_velocity_y;
}

__exports.__wbg_getvelocityy_78a22e6687f6ced5 = __wbg_getvelocityy_78a22e6687f6ced5;

function __wbg_setcoordinatex_73213dd23bd9b585(arg0, arg1) {
    getObject(arg0).set_coordinate_x(arg1);
}

__exports.__wbg_setcoordinatex_73213dd23bd9b585 = __wbg_setcoordinatex_73213dd23bd9b585;

function __wbg_setcoordinatey_6f38a5f44ca5ffa5(arg0, arg1) {
    getObject(arg0).set_coordinate_y(arg1);
}

__exports.__wbg_setcoordinatey_6f38a5f44ca5ffa5 = __wbg_setcoordinatey_6f38a5f44ca5ffa5;

function __wbg_setvelocityx_882b11b7d69b8111(arg0, arg1) {
    getObject(arg0).set_velocity_x(arg1);
}

__exports.__wbg_setvelocityx_882b11b7d69b8111 = __wbg_setvelocityx_882b11b7d69b8111;

function __wbg_setvelocityy_cce880dda12a9686(arg0, arg1) {
    getObject(arg0).set_velocity_y(arg1);
}

__exports.__wbg_setvelocityy_cce880dda12a9686 = __wbg_setvelocityy_cce880dda12a9686;

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
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

function freeEngine(ptr) {

    wasm.__wbg_engine_free(ptr);
}
/**
*/
export class Engine {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeEngine(ptr);
    }

    /**
    * @param {World} world
    * @returns {}
    */
    constructor(world) {
        const ptr0 = world.ptr;
        world.ptr = 0;
        this.ptr = wasm.engine_new(ptr0);
    }
    /**
    * @param {number} delta
    * @returns {void}
    */
    tick(delta) {
        return wasm.engine_tick(this.ptr, delta);
    }
    /**
    * @param {any} duck
    * @returns {void}
    */
    add_one(duck) {
        return wasm.engine_add_one(this.ptr, addHeapObject(duck));
    }
    /**
    * @returns {void}
    */
    clear_world() {
        return wasm.engine_clear_world(this.ptr);
    }
}

__exports.Engine = Engine;

function freeWorld(ptr) {

    wasm.__wbg_world_free(ptr);
}
/**
*/
export class World {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeWorld(ptr);
    }

    /**
    * @returns {}
    */
    constructor() {
        this.ptr = wasm.world_new();
    }
    /**
    * @param {any} duck
    * @returns {void}
    */
    add_one(duck) {
        return wasm.world_add_one(this.ptr, addHeapObject(duck));
    }
    /**
    * @returns {number}
    */
    size() {
        return wasm.world_size(this.ptr);
    }
    /**
    * @returns {number}
    */
    say() {
        return wasm.world_say(this.ptr);
    }
    /**
    * @returns {void}
    */
    think() {
        return wasm.world_think(this.ptr);
    }
}

__exports.World = World;

function __wbindgen_object_clone_ref(idx) {
    return addHeapObject(getObject(idx));
}

__exports.__wbindgen_object_clone_ref = __wbindgen_object_clone_ref;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function __wbindgen_object_drop_ref(i) { dropObject(i); }

__exports.__wbindgen_object_drop_ref = __wbindgen_object_drop_ref;

function init(module_or_path, maybe_memory) {
    let result;
    const imports = { './Eletron_Wasm': __exports };
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

