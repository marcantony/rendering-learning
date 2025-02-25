use crate::mesh::{Face, FaceN, Mesh};

pub fn subdivide<F: Face>(mesh: Mesh<F>, times: u32) -> Mesh<FaceN<4>> {
    assert!(times > 0, "Must iterate at least 1 time");
    let mut tmp = subdivide_inner(mesh);
    for _ in 0..(times - 1) {
        tmp = subdivide_inner(tmp)
    }
    tmp
}

fn subdivide_inner<F: Face>(mesh: Mesh<F>) -> Mesh<FaceN<4>> {
    todo!()
}
