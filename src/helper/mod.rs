use legion::EntityStore;
use legion::{Entity, world::SubWorld};
use num_traits::abs;

use crate::component::{Collider, Position, Velocity};

#[derive(Debug)]
pub struct Resolution {
    pub ent_a: Entity,
    pub ent_b: Entity,
    pub overlap_x: f64,
    pub overlap_y: f64,
    pub dir_x: f64,
    pub dir_y: f64,
    pub axis: bool,
}

pub fn aabb_overlap(
    pos_a: &Position<f64>,
    col_a: &Collider,
    pos_b: &Position<f64>,
    col_b: &Collider,
) -> Option<(f64, f64)> {
    let r1 = pos_a.x + col_a.w;
    let r2 = pos_b.x + col_b.w;
    let b1 = pos_a.y + col_a.h;
    let b2 = pos_b.y + col_b.h;

    if pos_a.x < r2 && r1 > pos_b.x && pos_a.y < b2 && b1 > pos_b.y {
        let overlap_x = r1.min(r2) - pos_a.x.max(pos_b.x);
        let overlap_y = b1.min(b2) - pos_a.y.max(pos_b.y);

        Some((overlap_x, overlap_y))
    } else {
        None
    }
}

pub fn apply_resolution(world: &mut SubWorld, res: &Resolution) {
    if let Ok(mut entry_a) = world.entry_mut(res.ent_a) {
        if res.axis {
            if let Ok(pos) = entry_a.get_component_mut::<Position<f64>>() {
                pos.x += res.overlap_x / 2.0 * res.dir_x;
            }
            if let Ok(velo) = entry_a.get_component_mut::<Velocity<f64>>() {
                velo.dx *= -1.0;
            }
        } else {
            if let Ok(pos) = entry_a.get_component_mut::<Position<f64>>() {
                pos.y += res.overlap_y / 2.0 * res.dir_y;
            }
            if let Ok(velo) = entry_a.get_component_mut::<Velocity<f64>>() {
                velo.dy *= -1.0;
            }
        }
    }

    if let Ok(mut entry_b) = world.entry_mut(res.ent_b) {
        if res.axis {
            if let Ok(pos) = entry_b.get_component_mut::<Position<f64>>() {
                pos.x -= res.overlap_x / 2.0 * res.dir_x;
            }
            if let Ok(velo) = entry_b.get_component_mut::<Velocity<f64>>() {
                velo.dx *= -1.0;
            }
        } else {
            if let Ok(pos) = entry_b.get_component_mut::<Position<f64>>() {
                pos.y -= res.overlap_y / 2.0 * res.dir_y;
            }
            if let Ok(velo) = entry_b.get_component_mut::<Velocity<f64>>() {
                velo.dy *= -1.0;
            }
        }
    }
}
