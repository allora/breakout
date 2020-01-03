use std::iter;

use float_cmp::*;

use amethyst::{
    core::{math::*, transform::ParentHierarchy},
    ecs::{
        error::WrongGeneration,
        prelude::{Entity, World, WorldExt},
    },
};

/// delete the specified root entity and all of its descendents as specified
/// by the Parent component and maintained by the ParentHierarchy resource
// from https://github.com/amethyst/evoli src/utils/hierarchy_util.rs
pub fn delete_hierarchy(root: Entity, world: &mut World) -> Result<(), WrongGeneration> {
    let entities = {
        iter::once(root)
            .chain(
                world
                    .read_resource::<ParentHierarchy>()
                    .all_children_iter(root),
            )
            .collect::<Vec<Entity>>()
    };
    world.delete_entities(&entities)
}

// A point is in a box when its coordinates are smaller or equal than the top
// right and larger or equal than the bottom left.
pub fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}

// Checks to see if two line segments are parallel
pub fn is_vector_parallel(
    a: Vector2<f32>,
    b: Vector2<f32>,
    c: Vector2<f32>,
    d: Vector2<f32>,
) -> bool {
    let line_ab = a - b;
    let line_cd = c - d;

    approx_eq!(f32, line_ab.dot(&line_cd), 1.0)
}

// Checks to see if two line segments intersect. It is assumed parallels are checked elsewhere
pub fn is_line_intersected(
    a: Vector2<f32>,
    b: Vector2<f32>,
    c: Vector2<f32>,
    d: Vector2<f32>,
) -> bool {
    // lines the same
    if approx_eq!(f32, a.x, c.x)
        && approx_eq!(f32, a.y, c.y)
        && approx_eq!(f32, b.x, d.x)
        && approx_eq!(f32, b.y, d.y)
    {
        return true;
    }

    // do intersection test
    let ta = ((c.y - d.y) * (a.x - c.x) + (d.x - c.x) * (a.y - c.y))
        / ((d.x - c.x) * (a.y - b.y) - (a.x - b.x) * (d.y - c.y));

    // ta intersection is outside the bounds of the line segments
    if ta > 1.0 || ta < 0.0 {
        return false;
    }

    let tb = ((a.y - b.y) * (a.x - c.x) + (b.x - a.x) * (a.y - c.y))
        / ((d.x - c.x) * (a.y - b.y) - (a.x - b.x) * (d.y - c.y));

    // tb intersection is outside the bounds of the line segments
    if tb > 1.0 || ta < 0.0 {
        return false;
    }

    // ta and tb are within their respective line segments, thus they intersect
    true
}
