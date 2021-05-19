pub const CHUNK_SIZE: usize = 16;

struct Chunk {
    position: Vector3<i32>,
    blocks: [[[usize; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    mesh: Mesh
}