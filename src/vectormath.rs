use cgmath::{Matrix3, Vector3, Vector4};

pub const _X_VECTOR: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
pub const Y_VECTOR: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
pub const _Z_VECTOR: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);

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
    (vec.x * vec.x + vec.y * vec.y + vec.z * vec.z).sqrt()
}

pub fn normalize(vec: &Vector3<f32>) -> Vector3<f32> {
    let len = len(vec);
    Vector3::new(vec.x / len, vec.y / len, vec.z / len)
}

pub fn normalize_inplace(vec: Vector3<f32>) -> Vector3<f32> {
    let len = len(&vec);
    Vector3::new(vec.x / len, vec.y / len, vec.z / len)
}

pub fn cross(a: &Vector3<f32>, b: &Vector3<f32>) -> Vector3<f32> {
    Vector3::new(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}

pub fn _dot(u: &Vector3<f32>, v: &Vector3<f32>) -> f32 {
    u.x * v.x + u.y * v.y + u.z * v.z
}
