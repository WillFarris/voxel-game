
struct Rect3 {
    pos: Vector3<f32>,
    size: Vector3<f32>,
}

fn point_vs_rect(p: &Vector3<f32>, r: &Rect3) -> bool { 
    p.x >= r.pos.x &&
    p.y >= r.pos.y &&
    p.z >= r.pos.z &&
    
    p.x <= (r.pos.x + r.size.x) &&
    p.y <= (r.pos.y + r.size.y) &&
    p.z <= (r.pos.z + r.size.z)
}

fn rect_vs_rect(r1: &Rect3, r2: &Rect3) -> bool {
    r1.pos.x < r2.pos.x + r2.size.x && r1.pos.x + r1.size.x > r2.pos.x &&
    r1.pos.y < r2.pos.y + r2.size.y && r1.pos.y + r1.size.y > r2.pos.y &&
    r1.pos.z < r2.pos.z + r2.size.z && r1.pos.z + r1.size.z > r2.pos.z
}