#![allow(unused_variables)]

extern crate geop;

use geop::corner_table_from_vertices_and_indices;
use geop::io::OffReader;
use geop::operator::Laplacian;

fn main() {
    println!("Running...");

    let reader = OffReader {
        ..Default::default()
    };

    let filename = "/home/luisfpereira/Repos/third-party/pyFM/examples/data/cat-00.off";

    let (vertices, faces) = reader.read::<f64>(filename);

    let mesh = corner_table_from_vertices_and_indices(&vertices, &faces);

    mesh.laplacian();
}
