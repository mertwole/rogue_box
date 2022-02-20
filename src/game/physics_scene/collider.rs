use crate::game::common::math::Vec2;

use super::collision_data::CollisionData;

pub struct Collider {
    shape: ColliderShape,
    pub(super) position: Vec2,
    collision_mask: u32,
    is_trigger: bool,
}

pub enum ColliderShape {
    Box { size: Vec2 },
    Circle { radius: f32 },
}

impl ColliderShape {
    fn segment_segment_overlap(
        a_left: f32,
        a_right: f32,
        b_left: f32,
        b_right: f32,
    ) -> Option<f32> {
        if a_right < b_left || a_left > b_right {
            None
        } else {
            let pos_overlap = a_right - b_left;
            let neg_overlap = a_left - b_right;
            if pos_overlap < -neg_overlap {
                Some(pos_overlap)
            } else {
                Some(neg_overlap)
            }
        }
    }

    fn box_box(a_pos: Vec2, a_size: Vec2, b_pos: Vec2, b_size: Vec2) -> Option<CollisionData> {
        let a_min = a_pos - a_size * 0.5;
        let a_max = a_min + a_size;
        let b_min = b_pos - b_size * 0.5;
        let b_max = b_min + b_size;

        let x_overlap = Self::segment_segment_overlap(a_min.x, a_max.x, b_min.x, b_max.x);
        if x_overlap.is_none() {
            return None;
        }

        let y_overlap = Self::segment_segment_overlap(a_min.y, a_max.y, b_min.y, b_max.y);
        if y_overlap.is_none() {
            return None;
        }

        let x_overlap = x_overlap.unwrap();
        let y_overlap = y_overlap.unwrap();

        let data = if f32::abs(x_overlap) < f32::abs(y_overlap) {
            CollisionData {
                normal: Vec2::new(f32::signum(x_overlap), 0.0),
                depth: f32::abs(x_overlap),
            }
        } else {
            CollisionData {
                normal: Vec2::new(0.0, f32::signum(y_overlap)),
                depth: f32::abs(y_overlap),
            }
        };

        Some(data)
    }

    fn circle_circle(a_center: Vec2, a_r: f32, b_center: Vec2, b_r: f32) -> Option<CollisionData> {
        let center_dist_sqr = (a_center - b_center).sqr_length();
        if center_dist_sqr > (a_r + b_r) * (a_r + b_r) {
            None
        } else {
            let center_dist = f32::sqrt(center_dist_sqr);
            let data = CollisionData {
                normal: (b_center - a_center) / center_dist,
                depth: a_r + b_r - center_dist,
            };
            Some(data)
        }
    }

    fn box_circle(a_pos: Vec2, a_size: Vec2, b_center: Vec2, b_r: f32) -> Option<CollisionData> {
        let a_min = a_pos - a_size * 0.5;
        let a_max = a_min + a_size;

        fn in_range(t: f32, min: f32, max: f32) -> bool {
            f32::signum(max - t) * f32::signum(t - min) > 0.0
        }

        if in_range(b_center.x, a_min.x, a_max.x) || in_range(b_center.y, a_min.y, a_max.y) {
            let circle_aabb_size = Vec2::new(b_r, b_r) * 2.0;
            return Self::box_box(a_pos, a_size, b_center, circle_aabb_size);
        }

        let rect_verts = vec![
            a_min,
            Vec2::new(a_min.x, a_max.y),
            a_max,
            Vec2::new(a_max.x, a_min.y),
        ];

        let closest_vert = rect_verts
            .into_iter()
            .map(|v| (v, (v - b_center).sqr_length()))
            .min_by(|x, y| x.1.partial_cmp(&y.1).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap()
            .0;

        let normal_vec = closest_vert - b_center;
        let normal_vec_len = normal_vec.length();

        Some(CollisionData {
            normal: -1.0 * normal_vec / normal_vec_len,
            depth: b_r - normal_vec_len,
        })
    }
}

impl Collider {
    pub fn new(shape: ColliderShape, position: Vec2) -> Collider {
        Collider {
            shape,
            position,
            collision_mask: 0xFFFFFFFF,
            is_trigger: false,
        }
    }

    pub fn new_trigger(shape: ColliderShape, position: Vec2) -> Collider {
        Collider {
            shape,
            position,
            collision_mask: 0xFFFFFFFF,
            is_trigger: true,
        }
    }

    pub fn collide(&self, other: &Collider) -> Option<CollisionData> {
        if self.collision_mask & other.collision_mask == 0 {
            return None;
        }

        match (&self.shape, &other.shape) {
            (ColliderShape::Box { size: a_size }, ColliderShape::Box { size: b_size }) => {
                ColliderShape::box_box(self.position, *a_size, other.position, *b_size)
            }
            (ColliderShape::Circle { radius: a_r }, ColliderShape::Circle { radius: b_r }) => {
                ColliderShape::circle_circle(self.position, *a_r, other.position, *b_r)
            }
            (ColliderShape::Box { size: a_size }, ColliderShape::Circle { radius: b_r }) => {
                ColliderShape::box_circle(self.position, *a_size, other.position, *b_r)
            }
            (ColliderShape::Circle { radius: a_r }, ColliderShape::Box { size: b_size }) => {
                let mut data =
                    ColliderShape::box_circle(other.position, *b_size, self.position, *a_r);
                match &mut data {
                    None => {}
                    Some(result) => {
                        result.reverse();
                    }
                }
                data
            }
        }
    }
}
