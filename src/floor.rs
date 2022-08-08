#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f32, f32, f32),
}

glium::implement_vertex!(Vertex, position);

pub const FLOOR_VERTICES: [Vertex; 4] = [
    Vertex {
        position: (-1000.0, 0.0, -1000.0),
    },
    Vertex {
        position: (-1000.0, 0.0, 1000.0),
    },
    Vertex {
        position: (1000.0, 0.0, 1000.0),
    },
    Vertex {
        position: (1000.0, 0.0, -1000.0),
    },
];

pub const FLOOR_INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];
