use glium::glutin;

use glium;

type V = (f32, f32, f32);
type M = [[f32; 4]; 4];
pub struct Camera {
    dir: V,
    pos: V,
    move_speed: f32,
    rotation_sensitivity: f32,
    moving_forward: bool,
    moving_backward: bool,
    moving_left: bool,
    moving_right: bool,
    yaw: f32,
    pitch: f32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            dir: (0.0, 0.0, -1.0f32),
            pos: (0.0, 0.0, 0.0f32),
            move_speed: 1.0f32,
            rotation_sensitivity: 0.1,
            moving_forward: false,
            moving_backward: false,
            moving_left: false,
            moving_right: false,
            yaw: 0.0,
            pitch: 0.0,
        }
    }

    pub fn get_view_mat(&self) -> M {
        let up = (0.0, 1.0, 0.0f32);
        let z = normalize(&self.dir);
        let x = normalize(&cross(&z, &up));
        let y = normalize(&cross(&x, &z));
        [
            [x.0, y.0, z.0, 0.0],
            [x.1, y.1, z.1, 0.0],
            [x.2, y.2, z.2, 0.0],
            [
                -dot(&x, &self.pos),
                -dot(&y, &self.pos),
                -dot(&z, &self.pos),
                1.0,
            ],
        ]
    }

    fn move_forward(&mut self, dist: f32) {
        let mut forward = normalize(&self.dir);
        forward.1 = 0.0;
        forward = scale(&normalize(&forward), dist);
        self.pos.0 -= forward.0;
        self.pos.1 -= forward.1;
        self.pos.2 -= forward.2;
    }

    fn move_right(&mut self, dist: f32) {
        let up = (0.0, 0.1, 0.0f32);
        let forward = normalize(&self.dir);
        let mut right = normalize(&cross(&up, &forward));
        right.1 = 0.0;
        right = scale(&normalize(&right), dist);
        self.pos.0 -= right.0;
        self.pos.1 -= right.1;
        self.pos.2 -= right.2;
    }

    pub fn process_device_event(&mut self, event: &glutin::event::DeviceEvent) {
        match event {
            glutin::event::DeviceEvent::Key(input) => {
                let key = match input.virtual_keycode {
                    Some(key) => key,
                    None => return,
                };
                let is_pressed = input.state == glutin::event::ElementState::Pressed;

                match key {
                    glutin::event::VirtualKeyCode::W => {
                        self.moving_forward = is_pressed;
                    }
                    glutin::event::VirtualKeyCode::S => {
                        self.moving_backward = is_pressed;
                    }
                    glutin::event::VirtualKeyCode::D => {
                        self.moving_right = is_pressed;
                    }
                    glutin::event::VirtualKeyCode::A => {
                        self.moving_left = is_pressed;
                    }
                    _ => (),
                }
            }
            glutin::event::DeviceEvent::MouseMotion { delta } => {
                self.yaw += (delta.0 as f32) * self.rotation_sensitivity;
                self.pitch -= (delta.1 as f32) * self.rotation_sensitivity;

                self.yaw %= 360.0;
                self.pitch %= 360.0;

                if self.pitch > 89.0 {
                    self.pitch = 89.0;
                }
                if self.pitch < -89.0 {
                    self.pitch = -89.0;
                }
            }
            _ => return,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let dist = self.move_speed * dt;
        let mut forward_dist = 0.0f32;
        let mut right_dist = 0.0f32;
        if self.moving_forward ^ self.moving_backward {
            forward_dist = {
                if self.moving_forward {
                    -dist
                } else if self.moving_backward {
                    dist
                } else {
                    0.0
                }
            }
        };
        if self.moving_left ^ self.moving_right {
            right_dist = {
                if self.moving_right {
                    dist
                } else if self.moving_left {
                    -dist
                } else {
                    0.0
                }
            }
        };

        if (self.moving_forward ^ self.moving_backward) & (self.moving_left ^ self.moving_right) {
            forward_dist /= 2.0f32.sqrt();
            right_dist /= 2.0f32.sqrt();
        }

        self.move_forward(forward_dist);
        self.move_right(right_dist);
        self.rotate();
    }

    fn rotate(&mut self) {
        let yc = self.yaw.to_radians().cos();
        let ys = self.yaw.to_radians().sin();
        let pc = self.pitch.to_radians().cos();
        let ps = self.pitch.to_radians().sin();
        self.dir = (pc * ys, ps, -pc * yc);
    }
}

fn normalize(v: &V) -> V {
    let len = (v.0 * v.0 + v.1 * v.1 + v.2 * v.2).sqrt();
    (v.0 / len, v.1 / len, v.2 / len)
}

fn scale(a: &V, k: f32) -> V {
    (a.0 * k, a.1 * k, a.2 * k)
}

fn cross(a: &V, b: &V) -> V {
    (
        a.1 * b.2 - a.2 * b.1,
        a.2 * b.0 - a.0 * b.2,
        a.0 * b.1 - a.1 * b.0,
    )
}

fn dot(a: &V, b: &V) -> f32 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

pub fn get_perspective(fov: f32, screen_height: u32, screen_width: u32) -> M {
    let aspect_ratio = screen_height as f32 / screen_width as f32;

    let zfar = 1024.0;
    let znear = 0.1;

    let f = 1.0 / (fov.to_radians() / 2.0).tan();

    [
        [f * aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
        [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
    ]
}
