#![allow(unused_variables)]

extern crate geop;
use std::time::Instant;

use geop::ds::{HalfEdgeMesh, SharedVertexMeshData};
use geop::io::OffReader;
use geop::operator::Laplacian;

fn main() {
    println!("Running...");

    let reader = OffReader {
        ..Default::default()
    };

    let filename = "/home/luisfpereira/Repos/third-party/pyFM/examples/data/cat-00.off";

    let (vertices, faces) = reader.read::<f64>(filename);

    let now = Instant::now();

    let mesh = HalfEdgeMesh::from(SharedVertexMeshData { vertices, faces });

    let laplace_dict = mesh.laplace_matrix();
    let areas = mesh.mass_matrix();

    let elapsed_time = now.elapsed();

    println!("Running took {} seconds.", elapsed_time.as_secs_f32());
}
