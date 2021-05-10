use crate::vectormath::*;
use glium::{Frame, Surface};

pub struct Camera {
    pub position: [f32; 3],
    pub forward: [f32; 3],
    pub right: [f32; 3],
    pub up: [f32; 3],
}

impl Camera {
    pub fn new(position: &[f32; 3], direction: &[f32; 3]) -> Self {
        let n_direction = normalize(&direction);
        let p = position.clone();
        let mut s = Self {
            position: p,
            forward: n_direction,
            right: [0.0, 0.0, 0.0],
            up: [0.0, 0.0, 0.0],
        };
        s.calculate_normals();
        s
    }

    pub fn view_matrix(&self) -> [[f32; 4]; 4] {
        let p = [
            -self.position[0] * self.right[0] - self.position[1] * self.right[1] - self.position[2] * self.right[2],
            
            -self.position[0] * self.up[0] - self.position[1] * self.up[1] - self.position[2] * self.up[2],

            -self.position[0] * self.forward[0] - self.position[1] * self.forward[1] - self.position[2] * self.forward[2],
        ];

        [
            [self.right[0], self.up[0], self.forward[0], 0.0],
            [self.right[1], self.up[1], self.forward[1], 0.0],
            [self.right[2], self.up[2], self.forward[2], 0.0],
            [p[0], p[1], p[2], 1.0],
        ]
    }

    fn calculate_normals(&mut self) {
        self.forward = normalize_inplace(self.forward);
        self.right = normalize(&cross(&Y_VECTOR, &self.forward));
        self.up = normalize(&cross(&self.forward, &self.right));
    }

    pub fn translate(&mut self, direction: &[f32; 3]) {
        self.position[0] += direction[0];
        self.position[1] += direction[1];
        self.position[2] += direction[2];

        self.forward[0] += direction[0];
        self.forward[1] += direction[1];
        self.forward[2] += direction[2];

        self.calculate_normals();
    }

    pub fn move_direction(&mut self, direction: &[f32; 3]) {
        self.position[0] += self.right[0] * direction[0];
        self.position[1] += self.right[1] * direction[0];
        self.position[2] += self.right[2] * direction[0];

        self.position[0] += self.up[0] * direction[1];
        self.position[1] += self.up[1] * direction[1];
        self.position[2] += self.up[2] * direction[1];

        self.position[0] += self.forward[0] * direction[2];
        self.position[1] += self.forward[1] * direction[2];
        self.position[2] += self.forward[2] * direction[2];

        self.calculate_normals();
    }

    pub fn rotate_on_y_axis(&mut self, angle: f32) {
        self.forward = crate::vectormath::quaternion_rotate(&self.forward, angle, &self.up);
        self.calculate_normals();
    }

    pub fn rotate_on_x_axis(&mut self, angle: f32) {
        self.forward = crate::vectormath::quaternion_rotate(&self.forward, angle, &self.right);
        self.calculate_normals();
    }
}

pub fn perspective_matrix(target: &Frame) -> [[f32; 4]; 4] {
    let (width, height) = target.get_dimensions();
    let aspect_ratio = height as f32 / width as f32;

    let fov: f32 = 3.141592 / 4.0;
    let zfar = 1024.0;
    let znear = 0.1;

    let f = 1.0 / (fov / 2.0).tan();

    [
        [f * aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
        [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
    ]
}
