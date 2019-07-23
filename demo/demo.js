function demo_twoBody() {
    app.stage.children.length = 0;
    engine.clear_world();
    var g = new Circle(10, 0x051BFA);
    g.mass = 100000;
    g.set_coordinate_x(390);
    g.set_coordinate_y(300);
    g.add_to_stage(app.stage);

    var gg = new Circle(10, 0x05FA5E);
    gg.mass = 10;
    gg.set_coordinate_x(390);
    gg.set_coordinate_y(200);
    gg.velocity_x = 100;
    gg.add_to_stage(app.stage);
    engine.add_one(gg);
    engine.add_one(g);
};

function demo_threeBody() {
    app.stage.children.length = 0;
    engine.clear_world();
    var g = new Circle(10, 0x051BFA);
    g.mass = 100000;
    g.set_coordinate_x(390);
    g.set_coordinate_y(300);
    g.add_to_stage(app.stage);

    var gg = new Circle(10, 0x05FA5E);
    gg.mass = 10;
    gg.set_coordinate_x(390);
    gg.set_coordinate_y(200);
    gg.velocity_x = 105;
    gg.add_to_stage(app.stage);


    var ggg = new Circle(10, 0x059AFA);
    ggg.mass = 500;
    ggg.set_coordinate_x(390);
    ggg.set_coordinate_y(350);
    ggg.velocity_x = 100;
    ggg.add_to_stage(app.stage);

    engine.add_one(ggg);
    engine.add_one(gg);
    engine.add_one(g);
};


function demo_addRandom() {
    factor = Math.random() * 5
    mass = Math.round(Math.pow(10, factor));
    var g = new Circle(10, massToColor(mass));
    g.mass = mass
    g.set_coordinate_x(Math.round(Math.random() * 600) + 100);
    g.set_coordinate_y(Math.round(Math.random() * 400) + 100);
    g.set_velocity_x(Math.random() * 100 - 50);
    g.set_velocity_y(Math.random() * 50 - 25);
    g.add_to_stage(app.stage);
    engine.add_one(g);
};


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
    window.gwasm.default("./wasm_resource/Eletron_Wasm_bg.wasm")
        .then(
            t => {
                let wo = new gwasm.World();
                let engin = new gwasm.Engine(wo)
                window.engine = engin;
                setInterval(function () {
                    engin.tick(10)
                }, 10)
                document.getElementById("twobody").disabled = false;
                document.getElementById("threebody").disabled = false;
                document.getElementById("add_random").disabled = false;
            });
}

var rgbToHex = function (rgb) {
    var hex = Number(rgb).toString(16);
    if (hex.length < 2) {
        hex = "0" + hex;
    }
    return hex;
};

var fullColorHex = function (r, g, b) {
    var red = rgbToHex(r);
    var green = rgbToHex(g);
    var blue = rgbToHex(b);
    return red + green + blue;
};

var massToColor = function (mass) {
    let factor = Math.log10(mass);
    return parseInt(fullColorHex(250, Math.round(factor * 50), 10), 16);
}