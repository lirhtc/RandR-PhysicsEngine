function demo() {
    let app = new PIXI.Application({
        width: 800, // default: 800
        height: 600, // default: 600
        antialias: true, // default: false
        transparent: false, // default: false
        resolution: 1 // default: 1
    });
    gwasm.default("./wasm-src/pkg/Eletron_Wasm_bg.wasm")
        .then(t => {
            var g = new Circle(20);
            g.mass = 100000;
            g.set_coordinate_x(390);
            g.set_coordinate_y(300);
            app.renderer.backgroundColor = 0x061639;
            document.body.appendChild(app.view);
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

            setInterval(function () {
                engin.tick(100)
            }, 10)
        })
};