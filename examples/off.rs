#![allow(unused_variables)]

extern crate geop;

use geop::io::OffReader;
use geop::{corner_table_from_vertices_and_indices, mesh_laplacian};

fn main() {
    println!("Running...");

    let reader = OffReader {
        ..Default::default()
    };

    let filename = "/home/luisfpereira/Repos/third-party/pyFM/examples/data/cat-00.off";

    let (vertices, faces) = reader.read::<f64>(filename);

    let mesh = corner_table_from_vertices_and_indices(&vertices, &faces);

    mesh_laplacian(mesh);
}
