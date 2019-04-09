function demo_twoBody() {

    var g = new Circle(30);
    g.mass = 100000;
    g.set_coordinate_x(390);
    g.set_coordinate_y(300);
    g.add_to_stage(app.stage);
    let wo = new gwasm.World();
    wo.add_one(g);
    let engin = new gwasm.Engine(wo)
    var gg = new Circle(10);
    gg.mass = 10;
    gg.set_coordinate_x(390);
    gg.set_coordinate_y(200);
    gg.velocity_x = 100;
    gg.add_to_stage(app.stage);
    engin.add_one(gg);
};

function demo_threeBody() {
    var g = new Circle(7, 0x051BFA);
    g.mass = 100000;
    g.set_coordinate_x(390);
    g.set_coordinate_y(300);
    g.add_to_stage(app.stage);

    var gg = new Circle(7, 0x05FA5E);
    gg.mass = 10;
    gg.set_coordinate_x(390);
    gg.set_coordinate_y(200);
    gg.velocity_x = 105;
    gg.add_to_stage(app.stage);


    var ggg = new Circle(7, 0x059AFA);
    ggg.mass = 500;
    ggg.set_coordinate_x(390);
    ggg.set_coordinate_y(350);
    ggg.velocity_x = 100;
    ggg.add_to_stage(app.stage);

    engin.add_one(ggg);
    engin.add_one(gg);
    engin.add_one(g);
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
    document.body.appendChild(app.view);
    gwasm.default("./wasm-src/pkg/Eletron_Wasm_bg.wasm")
        .then(
            t => {
                let wo = new gwasm.World();
                let engin = new gwasm.Engine(wo)
                window.engine = engin;
                setInterval(function () {
                    engin.tick(10)
                }, 10)
                document.getElementById("twobody").disable = false;
                document.getElementById("threebody").disable = false;
            });
}