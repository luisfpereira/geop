#![allow(unused_variables)]

extern crate geop;
use std::time::Instant;

use geop::ds::{DirectedEdgeMesh, SharedVertexMeshData};
use geop::io::OffReader;

fn main() {
    println!("Running...");

    let reader = OffReader {
        ..Default::default()
    };

    let filename = "/home/luisfpereira/Repos/third-party/pyFM/examples/data/cat-00.off";

    let (vertices, faces) = reader.read::<f64>(filename);

    let now = Instant::now();

    let mesh = DirectedEdgeMesh::from(SharedVertexMeshData { vertices, faces });

    let elapsed_time = now.elapsed();

    println!("Running took {} seconds.", elapsed_time.as_secs_f32());
}
