use std::ops::Mul;

use cgmath::{Matrix3, Vector3, Vector4};

use crate::world;

pub const X_VECTOR: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
pub const Y_VECTOR: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
#[allow(unused)]
pub const Z_VECTOR: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);

#[derive(PartialEq)]
enum Vec3Direction {
    X,
    Y,
    Z
}

pub fn quaternion_rotate(vec: &Vector3<f32>, angle: f32, axis: &Vector3<f32>) -> Vector3<f32> {
    let q = Vector4::new(
        (angle / 2.0).cos(),
        axis.x * (angle / 2.0).sin(),
        axis.y * (angle / 2.0).sin(),
        axis.z * (angle / 2.0).sin(),
    );

    let rotation_matrix: Matrix3<f32> = Matrix3::from_cols(
        Vector3::new(
            q.x * q.x + q.y * q.y - q.z * q.z - q[3] * q[3],
            2.0 * (q.y * q.z - q.x * q[3]),
            2.0 * (q.x * q.z + q.x * q.z),
        ),
        Vector3::new(
            2.0 * (q.y * q.z + q.x * q[3]),
            q.x * q.x - q.y * q.y + q.z * q.z - q[3] * q[3],
            2.0 * (q.z * q[3] - q.x * q.y),
        ),
        Vector3::new(
            2.0 * (q.y * q[3] - q.x * q.z),
            2.0 * (q.z * q[3] + q.x * q.y),
            q.x * q.x - q.y * q.y - q.z * q.z + q[3] * q[3],
        ),
    );

    Vector3::new(
        vec.x * rotation_matrix.x.x
            + vec.y * rotation_matrix.x.y
            + vec.z * rotation_matrix.x.z,
        vec.x * rotation_matrix.y.x
            + vec.y * rotation_matrix.y.y
            + vec.z * rotation_matrix.y.z,
        vec.x * rotation_matrix.z.x
            + vec.y * rotation_matrix.z.y
            + vec.z * rotation_matrix.z.z,
        )
}

pub fn _quaternion_rotation_matrix(axis: &Vector3<f32>, angle: f32) -> Matrix3<f32> {
    
    let n_axis = normalize(axis);
    let q: Vector4<f32> = Vector4::new(
        (angle / 2.0).cos(),
        n_axis.x * (angle / 2.0).sin(),
        n_axis.y * (angle / 2.0).sin(),
        n_axis.z * (angle / 2.0).sin(),
    );

    Matrix3::from_cols(
        Vector3::new(
            q.x * q.x + q.y * q.y - q.z * q.z - q[3] * q[3],
            2.0 * (q.y * q.z - q.x * q[3]),
            2.0 * (q.x * q.z + q.x * q.z),
        ),
        Vector3::new(
            2.0 * (q.y * q.z + q.x * q[3]),
            q.x * q.x - q.y * q.y + q.z * q.z - q[3] * q[3],
            2.0 * (q.z * q[3] - q.x * q.y),
        ),
        Vector3::new(
            2.0 * (q.y * q[3] - q.x * q.z),
            2.0 * (q.z * q[3] + q.x * q.y),
            q.x * q.x - q.y * q.y - q.z * q.z + q[3] * q[3],
        ),
    )
}


pub fn len(vec: &Vector3<f32>) -> f32 {
    let len = (vec.x * vec.x + vec.y * vec.y + vec.z * vec.z).sqrt();
    if len.is_nan() { return 0.0; }
    len
}

pub fn normalize(vec: &Vector3<f32>) -> Vector3<f32> {
    let len = len(vec);
    Vector3::new(vec.x / len, vec.y / len, vec.z / len)
}

pub fn normalize_inplace(vec: &mut Vector3<f32>) {
    *vec /= len(vec);
}

pub fn cross(a: Vector3<f32>, b: Vector3<f32>) -> Vector3<f32> {
    Vector3::new(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}

pub fn dot<T: std::ops::Add<Output = T> + Mul<Output = T>>(u: Vector3<T>, v: Vector3<T>) -> T {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn dda(world: &world::World, start: &Vector3<f32>, dir: &Vector3<f32>, max_dist: f32) -> Option<(Vector3<f32>, Vector3<isize>)> {
    let ray_dir = normalize(dir);

    let mut ray_unit_step_size = Vector3 {
        x: (1.0 + (ray_dir.y/ray_dir.x)*(ray_dir.y/ray_dir.x) + (ray_dir.z/ray_dir.x)*(ray_dir.z/ray_dir.x)).sqrt(),
        y: ((ray_dir.x/ray_dir.y)*(ray_dir.x/ray_dir.y) + 1.0 + (ray_dir.z/ray_dir.y)*(ray_dir.z/ray_dir.y)).sqrt(),
        z: ((ray_dir.x/ray_dir.z)*(ray_dir.x/ray_dir.z) + (ray_dir.y/ray_dir.z)*(ray_dir.y/ray_dir.z) + 1.0).sqrt(),
    };

    if ray_unit_step_size.x.is_nan() {
        ray_unit_step_size.x = 1.0;
    }
    if ray_unit_step_size.y.is_nan() {
        ray_unit_step_size.y = 1.0;
    }
    if ray_unit_step_size.z.is_nan() {
        ray_unit_step_size.z = 1.0;
    }

    let mut map_check = Vector3 {
        x: start.x.floor() as isize,
        y: start.y.floor() as isize,
        z: start.z.floor() as isize,
    };
    let mut ray_length_1d = Vector3 {x: 0.0, y: 0.0, z: 0.0 };
    let mut step = Vector3 {x: 0, y: 0, z: 0};

    if ray_dir.x < 0.0 {
        step.x = -1;
        ray_length_1d.x = (start.x - map_check.x as f32) * ray_unit_step_size.x;
    } else {
        step.x = 1;
        ray_length_1d.x = ((map_check.x as f32 + 1.0) - start.x) * ray_unit_step_size.x;
    }

    if ray_dir.y < 0.0 {
        step.y = -1;
        ray_length_1d.y = (start.y - map_check.y as f32) * ray_unit_step_size.y;
    } else {
        step.y = 1;
        ray_length_1d.y = ((map_check.y as f32 + 1.0) - start.y) * ray_unit_step_size.y;
    }

    if ray_dir.z < 0.0 {
        step.z = -1;
        ray_length_1d.z = (start.z - map_check.z as f32) * ray_unit_step_size.z;
    } else {
        step.z = 1;
        ray_length_1d.z = ((map_check.z as f32 + 1.0) - start.z) * ray_unit_step_size.z;
    }

    let mut dist = 0.0;
    while dist < max_dist {

        let mut min_dist = ray_length_1d.x;
        let mut min_dir = Vec3Direction::X;
        if ray_length_1d.y < min_dist { min_dist = ray_length_1d.y; min_dir = Vec3Direction::Y }
        if ray_length_1d.z < min_dist { min_dist = ray_length_1d.z; min_dir = Vec3Direction::Z }

        if min_dir == Vec3Direction::X {
            map_check.x += step.x;
            dist = ray_length_1d.x;
            ray_length_1d.x += ray_unit_step_size.x;
        } else if min_dir == Vec3Direction::Y {
            map_check.y += step.y;
            dist = ray_length_1d.y;
            ray_length_1d.y += ray_unit_step_size.y;
        } else {
            map_check.z += step.z;
            dist = ray_length_1d.z;
            ray_length_1d.z += ray_unit_step_size.z;
        }
        if world.collision_at_world_pos(map_check) {
            return Some(
                (start + ray_dir * dist, Vector3 { x: map_check.x, y: map_check.y, z: map_check.z})
            );
        }
    }
    None
}