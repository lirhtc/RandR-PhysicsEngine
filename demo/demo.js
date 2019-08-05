function prepareStage() {
    let app = new PIXI.Application({
        width: 800, // default: 800
        height: 600, // default: 600
        antialias: true, // default: false
        transparent: false, // default: false
        resolution: 1 // default: 1
    });
    window.app = app;
    app.renderer.backgroundColor = 0x061639;
    document.getElementById("app").appendChild(app.view);
    window.wasm.default("../wasm-src/pkg/rhandr_physics_engine_bg.wasm")
        .then(
            t => {});
}


class Rectangle {
    constructor(width, height, x, y) {
        this.width = width;
        this.height = height;
        this._x = x;
        this._y = y;
        this.constructWasmRectangle(width, height, x, y);
        this.setupGraphics();
    }
    constructWasmRectangle(width, height, x, y) {
        let shape = new wasm.ConvexPolygon()
        shape.add_vertex(0, 0);
        shape.add_vertex(width, 0);
        shape.add_vertex(width, height);
        shape.add_vertex(0, height);
        shape.set_x(x);
        shape.set_y(y);
        this.wasm_shape = shape;
    }

    setupGraphics() {
        let graphics = new PIXI.Graphics();
        graphics.beginFill(0xFFFF00);
        graphics.drawRect(0, 0, this.width, this.height);
        this.graphics = graphics;
    }

    setVelocity(vx, vy) {
        this.wasm_shape.set_velocity_x(vx);
        this.wasm_shape.set_velocity_y(vy);
    }

    set x(value) {
        this._x = value;
        this.graphics.x = value;
    }

    set y(value) {
        this._y = value;
        this.graphics.y = value;
    }
}

class World {
    constructor() {
        this.shape = [];
        this.wasm_world = new wasm.SimpleWorld(1);
    }
    addRect(rectangle) {
        this.shape.push(rectangle);
        this.wasm_world.add_convex_polygon(rectangle.wasm_shape);
    }

    update() {
        this.wasm_world.update();
        let length = this.shape.length;
        for (let i = 0; i < length; i++) {
            let x = this.wasm_world.get_polygon_x_at(i);
            let y = this.wasm_world.get_polygon_y_at(i);
            this.shape[i].x = x;
            this.shape[i].y = y;
        }
    }

    start() {
        setInterval(this.update.bind(this), 16);
    }
}

function runDemo() {
    let a = new Rectangle(10, 10, 110, 10)
    let c = new Rectangle(10, 10, 200, 10)
    let d = new Rectangle(10, 10, 300, 10)
    let b = new World()
    a.setVelocity(10, 10)
    c.setVelocity(-10, 10)
    d.setVelocity(-10, 10)
    app.stage.addChild(a.graphics)
    app.stage.addChild(c.graphics)
    app.stage.addChild(d.graphics)
    b.addRect(a)
    b.addRect(c)
    b.addRect(d)
    b.start()
}