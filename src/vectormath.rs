use std::intrinsics::transmute;

pub const _X_VECTOR: [f32; 3] = [1.0, 0.0, 0.0];
pub const Y_VECTOR: [f32; 3] = [0.0, 1.0, 0.0];
pub const _Z_VECTOR: [f32; 3] = [0.0, 0.0, 1.0];

pub const IDENTITY_MAT4: [[f32; 4]; 4] = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

pub fn quaternion_rotate(vec: &[f32; 3], angle: f32, axis: &[f32; 3]) -> [f32; 3] {
    let q = [
        (angle / 2.0).cos(),
        axis[0] * (angle / 2.0).sin(),
        axis[1] * (angle / 2.0).sin(),
        axis[2] * (angle / 2.0).sin(),
    ];

    let rotation_matrix: [[f32; 3]; 3] = [
        [
            q[0] * q[0] + q[1] * q[1] - q[2] * q[2] - q[3] * q[3],
            2.0 * (q[1] * q[2] - q[0] * q[3]),
            2.0 * (q[0] * q[2] + q[0] * q[2]),
        ],
        [
            2.0 * (q[1] * q[2] + q[0] * q[3]),
            q[0] * q[0] - q[1] * q[1] + q[2] * q[2] - q[3] * q[3],
            2.0 * (q[2] * q[3] - q[0] * q[1]),
        ],
        [
            2.0 * (q[1] * q[3] - q[0] * q[2]),
            2.0 * (q[2] * q[3] + q[0] * q[1]),
            q[0] * q[0] - q[1] * q[1] - q[2] * q[2] + q[3] * q[3],
        ],
    ];

    [
        (vec[0] * rotation_matrix[0][0]
            + vec[1] * rotation_matrix[0][1]
            + vec[2] * rotation_matrix[0][2]),
        (vec[0] * rotation_matrix[1][0]
            + vec[1] * rotation_matrix[1][1]
            + vec[2] * rotation_matrix[1][2]),
        (vec[0] * rotation_matrix[2][0]
            + vec[1] * rotation_matrix[2][1]
            + vec[2] * rotation_matrix[2][2]),
    ]
}

pub fn quaternion_rotation_matrix(axis: &[f32; 3], angle: f32) -> [[f32; 3]; 3] {
    
    let n_axis = normalize(axis);
    let q = [
        (angle / 2.0).cos(),
        n_axis[0] * (angle / 2.0).sin(),
        n_axis[1] * (angle / 2.0).sin(),
        n_axis[2] * (angle / 2.0).sin(),
    ];

    [
        [
            q[0] * q[0] + q[1] * q[1] - q[2] * q[2] - q[3] * q[3],
            2.0 * (q[1] * q[2] - q[0] * q[3]),
            2.0 * (q[0] * q[2] + q[0] * q[2]),
        ],
        [
            2.0 * (q[1] * q[2] + q[0] * q[3]),
            q[0] * q[0] - q[1] * q[1] + q[2] * q[2] - q[3] * q[3],
            2.0 * (q[2] * q[3] - q[0] * q[1]),
        ],
        [
            2.0 * (q[1] * q[3] - q[0] * q[2]),
            2.0 * (q[2] * q[3] + q[0] * q[1]),
            q[0] * q[0] - q[1] * q[1] - q[2] * q[2] + q[3] * q[3],
        ],
    ]
}

pub fn transpose(mat: &[[f32; 3]; 3]) -> [[f32; 3]; 3] {
    let mut t = [[0f32; 3]; 3];
    for x in 0..3 {
        for y in 0..3 {
            t[x][y] = mat[y][x];
        }
    }
    t
}

pub fn len(vec: &[f32; 3]) -> f32 {
    (vec[0] * vec[0] + vec[1] * vec[1] + vec[2] * vec[2]).sqrt()
}

pub fn normalize(vec: &[f32; 3]) -> [f32; 3] {
    let len = len(vec);
    [vec[0] / len, vec[1] / len, vec[2] / len]
}

pub fn normalize_inplace(vec: [f32; 3]) -> [f32; 3] {
    let len = len(&vec);
    [vec[0] / len, vec[1] / len, vec[2] / len]
}

pub fn cross(a: &[f32; 3], b: &[f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

pub fn _dot(u: &[f32; 3], v: &[f32; 3]) -> f32 {
    u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
}
