const __exports = {};

let wasm;

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

function freeCollisionDetectorAabb(ptr) {

    wasm.__wbg_collisiondetectoraabb_free(ptr);
}
/**
*/
export class CollisionDetectorAabb {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeCollisionDetectorAabb(ptr);
    }

    /**
    * @returns {}
    */
    constructor() {
        this.ptr = wasm.collisiondetectoraabb_new();
    }
    /**
    * @param {ConvexPolygon} first
    * @param {ConvexPolygon} second
    * @returns {boolean}
    */
    static collision_detect_polygon_polygon(first, second) {
        return (wasm.collisiondetectoraabb_collision_detect_polygon_polygon(first.ptr, second.ptr)) !== 0;
    }
}

__exports.CollisionDetectorAabb = CollisionDetectorAabb;

function freeCollisionResolver(ptr) {

    wasm.__wbg_collisionresolver_free(ptr);
}
/**
*/
export class CollisionResolver {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeCollisionResolver(ptr);
    }

    /**
    * @param {ConvexPolygon} first
    * @param {ConvexPolygon} second
    * @param {number} delta
    * @returns {void}
    */
    static collision_resolver_polygon_polygon(first, second, delta) {
        return wasm.collisionresolver_collision_resolver_polygon_polygon(first.ptr, second.ptr, delta);
    }
}

__exports.CollisionResolver = CollisionResolver;

function freeConvexPolygon(ptr) {

    wasm.__wbg_convexpolygon_free(ptr);
}
/**
*/
export class ConvexPolygon {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeConvexPolygon(ptr);
    }

    /**
    * @returns {}
    */
    constructor() {
        this.ptr = wasm.convexpolygon_new();
    }
    /**
    * @returns {number}
    */
    get_x() {
        return wasm.convexpolygon_get_x(this.ptr);
    }
    /**
    * @param {number} value
    * @returns {void}
    */
    set_x(value) {
        return wasm.convexpolygon_set_x(this.ptr, value);
    }
    /**
    * @returns {number}
    */
    get_y() {
        return wasm.convexpolygon_get_y(this.ptr);
    }
    /**
    * @param {number} value
    * @returns {void}
    */
    set_y(value) {
        return wasm.convexpolygon_set_y(this.ptr, value);
    }
    /**
    * @returns {number}
    */
    get_velocity_x() {
        return wasm.convexpolygon_get_velocity_x(this.ptr);
    }
    /**
    * @returns {number}
    */
    get_velocity_y() {
        return wasm.convexpolygon_get_velocity_y(this.ptr);
    }
    /**
    * @param {number} new_v
    * @returns {void}
    */
    set_velocity_x(new_v) {
        return wasm.convexpolygon_set_velocity_x(this.ptr, new_v);
    }
    /**
    * @param {number} new_v
    * @returns {void}
    */
    set_velocity_y(new_v) {
        return wasm.convexpolygon_set_velocity_y(this.ptr, new_v);
    }
    /**
    * @returns {number}
    */
    get_mass() {
        return wasm.convexpolygon_get_mass(this.ptr);
    }
    /**
    * @param {number} new_mass
    * @returns {void}
    */
    set_mass(new_mass) {
        return wasm.convexpolygon_set_mass(this.ptr, new_mass);
    }
    /**
    * @param {number} x
    * @param {number} y
    * @returns {void}
    */
    add_vertex(x, y) {
        return wasm.convexpolygon_add_vertex(this.ptr, x, y);
    }
}

__exports.ConvexPolygon = ConvexPolygon;

function freeSimpleWorld(ptr) {

    wasm.__wbg_simpleworld_free(ptr);
}
/**
*/
export class SimpleWorld {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeSimpleWorld(ptr);
    }

    /**
    * @param {number} id
    * @returns {}
    */
    constructor(id) {
        this.ptr = wasm.simpleworld_new(id);
    }
    /**
    * @returns {void}
    */
    update() {
        return wasm.simpleworld_update(this.ptr);
    }
    /**
    * @param {ConvexPolygon} polygon
    * @returns {number}
    */
    add_convex_polygon(polygon) {
        const ptr0 = polygon.ptr;
        polygon.ptr = 0;
        return wasm.simpleworld_add_convex_polygon(this.ptr, ptr0);
    }
    /**
    * @param {number} delta
    * @returns {void}
    */
    set_update_delta(delta) {
        return wasm.simpleworld_set_update_delta(this.ptr, delta);
    }
    /**
    * @param {number} idx
    * @returns {number}
    */
    get_polygon_x_at(idx) {
        return wasm.simpleworld_get_polygon_x_at(this.ptr, idx);
    }
    /**
    * @param {number} idx
    * @returns {number}
    */
    get_polygon_y_at(idx) {
        return wasm.simpleworld_get_polygon_y_at(this.ptr, idx);
    }
}

__exports.SimpleWorld = SimpleWorld;

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function __wbindgen_object_drop_ref(i) { dropObject(i); }

__exports.__wbindgen_object_drop_ref = __wbindgen_object_drop_ref;

function init(module_or_path, maybe_memory) {
    let result;
    const imports = { './rhandr_physics_engine': __exports };
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

