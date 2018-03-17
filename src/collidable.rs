extern crate tiled;
extern crate nalgebra;

use piston_window::*;
use piston::input::Key;
use app::{ButtonStates};
use std::iter::*;
use std::slice::*;
use std::any::Any;
use std::collections::*;
use entity::Entity;
use self::tiled::{Map, PropertyValue, Tile};
use self::nalgebra::{Vector2, Point2, Similarity2};

#[derive(Debug)]
pub struct Collidable {
    pub pos: Point2<f32>,
    pub speed: Vector2<f32>,
    pub grounded: bool,
}

impl Collidable {
    fn correct_collision(&mut self, walls: &Vec<(Point2<f32>, Point2<f32>)>, transform: &Similarity2<f32>, point: &Point2<f32>, movement: &Vector2<f32>) -> Option<(Point2<f32>, f32)> {
        let mut smallest_u = 2f32;
        let mut smallest_pos = Point2::new(0f32, 0f32);
        for vector in walls {
            let (p1, p2) = vector.clone();
            let p = transform * p1;
            let q = point.clone();
            let r = transform * p2 - p;
            let s = movement;
            let t = (q - p).perp(&s) / r.perp(&s);
            let u = (q - p).perp(&r) / r.perp(&s);
            if t >= 0f32 && t <= 1f32 && u >= 0f32 && u <= 1f32 && u < smallest_u {
                smallest_u = u;
                let change_vec = s - r.normalize() * s.dot(&r.normalize());
                smallest_pos = self.pos - (change_vec * (1f32 - u)) - change_vec.normalize() * 0.0001234f32;
                //println!("{:?} {:?} {:?}", q, s, u);
            }
        }
        if smallest_u <= 1f32 {
            let change = Some((smallest_pos, smallest_u));
            return change;
        }
        return None;
    }

    pub fn handle_collisions(&mut self, points: &Vec<Point2<f32>>, map: &Map, prev_pos: &Point2<f32>) {
        let tileset = map.get_tileset_by_gid(1).unwrap();
        let layer: &tiled::Layer = &map.layers[0];
        let tile_width = tileset.tile_width;
        let tile_height = tileset.tile_height;
        let (small_x, large_x) = if self.pos.x < prev_pos.x { (self.pos.x - 1f32, prev_pos.x + 33f32) } else { (prev_pos.x - 1f32, self.pos.x + 33f32) };
        let (small_y, large_y) = if self.pos.y < prev_pos.y { (self.pos.y - 1f32, prev_pos.y + 33f32) } else { (prev_pos.y - 1f32, self.pos.y + 33f32) };
        let tile_x = (small_x as u32 / tile_width) as usize;
        let tile_y = (small_y as u32 / tile_height) as usize;
        let tile_x2 = (large_x / tile_width as f32) as usize;
        let tile_y2 = (large_y / tile_height as f32) as usize;
        let mut tiles_solid = vec![];
        for y in tile_y..(tile_y2 + 1) {
            let mut solid_tile_row = vec![];
            for x in tile_x..(tile_x2 + 1) {
                solid_tile_row.push(false);
                let solid_vec_size = solid_tile_row.len() - 1usize;
                let ref mut tile_solid = solid_tile_row[solid_vec_size];
                if y >= layer.tiles.len() || x > layer.tiles[y].len() {
                    continue;
                }
                let tile = layer.tiles[y as usize][x as usize] as usize;
                if tile == 0 {
                    continue;
                }
                let tile_properties = tileset.tiles.binary_search_by_key(&(tile - 1), |&Tile{id, ..}| id as usize);
                if let Ok(tile_index) = tile_properties {
                    if let Some(solid) = tileset.tiles[tile_index].properties.get("solid") {
                        match solid {
                            &PropertyValue::BoolValue(true) => {
                                *tile_solid = true;
                            },
                            &PropertyValue::StringValue(ref is_solid) if is_solid == "true" => {
                                *tile_solid = true;
                            },
                            _ => {}
                        }
                    }
                }
            }
            tiles_solid.push(solid_tile_row);
        }

        self.grounded = false;
        let translate = Similarity2::new(Vector2::new(tile_x as f32 * tile_width as f32, tile_y as f32 * tile_height as f32), 0f32, tile_width as f32);
        let walls = get_vectors_from_tiles(&tiles_solid);
        for _i in 0..9 {
            let start_pos = self.pos;
            let mut min_pos = (self.pos, 2f32);
            for point in points {
                let mut pos = self.pos;
                if let Some(new_pos) = self.correct_collision(&walls, &translate, &(point + (prev_pos - Point2::new(0f32, 0f32))), &(pos - prev_pos)) {
                    if new_pos.1 < min_pos.1 {
                        min_pos = new_pos;
                    }
                }
            }
            if min_pos.0 != self.pos {
                if (min_pos.0 - self.pos).angle(&Vector2::new(0f32, -1f32)).abs() < 0.3f32 {
                    self.grounded = true;
                }
                let correction_direction = (self.pos - min_pos.0).normalize();
                //println!("{:?} {:?} {:?}", self.pos, self.speed, _i);
                self.speed = self.speed - correction_direction * self.speed.dot(&correction_direction);
                self.pos = min_pos.0;
                //println!("{:?} {:?} {:?}", self.pos, self.speed, _i);
                //println!("{:?}", prev_pos);
            }
            if start_pos == self.pos {
                break;
            }
        }
    }
}

fn get_vectors_from_tiles(tiles: &Vec<Vec<bool>>) -> Vec<(Point2<f32>, Point2<f32>)> {
    let width = tiles[0].len();
    let height = tiles.len();
    let mut vectors = vec![];
    for y in 0..height + 1usize {
        let mut vec = (Point2::new(0f32, 0f32), Point2::new(0f32, 0f32));
        let mut prev_line = false;
        for x in 0..width {
            let is_line = if (y == 0 && tiles[y][x]) ||
                (y == height && tiles[y - 1][x]) ||
                (y > 0 && y < height && (tiles[y - 1][x] ^ tiles[y][x])) {
                true
            } else {
                false
            };
            if !prev_line && is_line {
                vec = (Point2::new(x as f32, y as f32), Point2::new(x as f32 + 1f32, y as f32));
            }
            if prev_line && is_line {
                vec = (vec.0, Point2::new(x as f32 + 1f32, y as f32));
            }
            if (prev_line && !is_line) || (is_line && x == width - 1usize) {
                vectors.push(vec);
            }
            prev_line = is_line;
        }
    }
    for x in 0..width + 1usize {
        let mut vec = (Point2::new(0f32, 0f32), Point2::new(0f32, 0f32));
        let mut prev_line = false;
        for y in 0..height {
            let is_line = if (x == 0 && tiles[y][x]) ||
                (x == width && tiles[y][x - 1]) ||
                (x > 0 && x < width && (tiles[y][x - 1] ^ tiles[y][x])) {
                true
            } else {
                false
            };
            if !prev_line && is_line {
                vec = (Point2::new(x as f32, y as f32), Point2::new(x as f32, y as f32 + 1f32));
            }
            if prev_line && is_line {
                vec = (vec.0, Point2::new(x as f32, y as f32 + 1f32));
            }
            if (prev_line && !is_line) || (is_line && y == height - 1usize) {
                vectors.push(vec);
            }
            prev_line = is_line;
        }
    }
    return vectors;
}