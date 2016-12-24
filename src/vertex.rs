#[derive(Copy, Clone)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub color: [f32; 3],
}

implement_vertex!(Vertex, pos, color);
