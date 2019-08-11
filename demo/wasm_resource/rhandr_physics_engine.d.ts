/* tslint:disable */
/**
*/
export class CollisionDetectorAabb {
  free(): void;
  constructor();
  static collision_detect_polygon_polygon(first: ConvexPolygon, second: ConvexPolygon): boolean;
}
/**
*/
export class CollisionResolver {
  free(): void;
  static collision_resolver_polygon_polygon(first: ConvexPolygon, second: ConvexPolygon, delta: number): void;
}
/**
*/
export class ConvexPolygon {
  free(): void;
  constructor();
  get_x(): number;
  set_x(value: number): void;
  get_y(): number;
  set_y(value: number): void;
  get_velocity_x(): number;
  get_velocity_y(): number;
  set_velocity_x(new_v: number): void;
  set_velocity_y(new_v: number): void;
  get_mass(): number;
  set_mass(new_mass: number): void;
  add_vertex(x: number, y: number): void;
}
/**
*/
export class SimpleWorld {
  free(): void;
  constructor(id: number);
  update(): void;
  add_convex_polygon(polygon: ConvexPolygon): number;
  set_update_delta(delta: number): void;
  get_polygon_x_at(idx: number): number;
  get_polygon_y_at(idx: number): number;
}
