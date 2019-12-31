use std::iter;

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
    ax: f32,
    ay: f32,
    bx: f32,
    by: f32,
    cx: f32,
    cy: f32,
    dx: f32,
    dy: f32,
) -> bool {
    let point_a = Vector2::new(ax, ay);
    let point_b = Vector2::new(bx, by);
    let point_c = Vector2::new(cx, cy);
    let point_d = Vector2::new(dx, dy);

    let line_ab = point_a - point_b;
    let line_cd = point_c - point_d;

    line_ab.dot(&line_cd) == 1.0
}

// Checks to see if two line segments intersect. It is assumed parallels are checked elsewhere
pub fn is_line_intersected(
    ax: f32,
    ay: f32,
    bx: f32,
    by: f32,
    cx: f32,
    cy: f32,
    dx: f32,
    dy: f32,
) -> bool {
    // lines the same
    if ax == cx && ay == cy && bx == dx && by == dy {
        return true;
    }

    // do intersection test
    let ta = ((cy - dy) * (ax - cx) + (dx - cx) * (ay - cy))
        / ((dx - cx) * (ay - by) - (ax - bx) * (dy - cy));

    // ta intersection is outside the bounds of the line segments
    if ta > 1.0 || ta < 0.0 {
        return false;
    }

    let tb = ((ay - by) * (ax - cx) + (bx - ax) * (ay - cy))
        / ((dx - cx) * (ay - by) - (ax - bx) * (dy - cy));

    // tb intersection is outside the bounds of the line segments
    if tb > 1.0 || ta < 0.0 {
        return false;
    }

    // ta and tb are within their respective line segments, thus they intersect
    true
}
