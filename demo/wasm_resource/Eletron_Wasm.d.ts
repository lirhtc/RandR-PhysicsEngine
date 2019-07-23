/* tslint:disable */
/**
*/
export class Engine {
  free(): void;
  constructor(world: World);
  tick(delta: number): void;
  add_one(duck: any): void;
  clear_world(): void;
}
/**
*/
export class World {
  free(): void;
  constructor();
  add_one(duck: any): void;
  size(): number;
  say(): number;
  think(): void;
}
