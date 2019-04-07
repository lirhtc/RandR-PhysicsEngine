// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg/hello_world';`
// will work here one day as well!
const rust = import('./pkg/Rust_Assembly');
let physic;
rust
  .then(m => {
    physic = m;
    test();
  })
  .catch(console.error);

function test() {
  let g = {
    _x: 0,
    _y: 0,
    _vx: 0,
    _vy: 0,
    get_coordinate_x: function () {
      return this._x;
    },
    set_coordinate_x: function (value) {
      this._x = value;
    },
    get_coordinate_y: function () {
      return this._y;
    },
    set_coordinate_y: function (value) {
      this._y = value;
    },
    get_velocity_x: function () {
      return this._vx;
    },
    get_velocity_y: function () {
      return this._vy;
    },
    set_velocity_y: function (value) {
      this._vy = value;
    },
    set_velocity_x: function (value) {
      this._vx = value;
    },
    get_mass: function () {
      return 10;
    }
  }

  let world = physic.World.new();
  let engine = physic.Engine.new(world);
  world.add_one(g);
  let t = JSON.parse(JSON.stringify(g));
  t.set_coordinate_x(10);
  world.add_one(t);
  engine.tick();
  console.log(g.get_coordinate_x())
  console.log(t.get_coordinate_x())
}

export class MyShape {
  constructor() {
    this._x = 0,
      this._y = 0,
      this._vx = 0,
      this._vy = 0,
      this._mass = 0
  }

  get get_coordinate_x() {
    return this._x;
  }

  set set_coordinate_x(value) {
    this._x = value;
  }
  get get_coordinate_y() {
    return this._y;
  }
  set set_coordinate_y(value) {
    this._y = value;
  }
  get get_velocity_x() {
    return this._vx;
  }

  set set_velocity_x(value) {
    return this._vx = value;
  }

  get get_velocity_y() {
    return this._vy;
  }

  set set_velocity_y(value) {
    return this._vy = value;
  }
  get get_mass() {
    return this._mass;
  }
}