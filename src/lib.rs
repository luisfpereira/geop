#![feature(array_chunks)]

pub mod io;
pub mod operator;
pub mod utils;

use baby_shark::geometry::traits::RealNumber;
use baby_shark::mesh::corner_table::table::CornerTable;
use baby_shark::mesh::traits::Mesh;
use nalgebra::Vector3;

pub fn corner_table_from_vertices_and_indices<S>(vertices: &[S], faces: &[usize]) -> CornerTable<S>
where
    S: RealNumber,
{
    let new_vertices: Vec<Vector3<S>> = vertices
        .array_chunks::<3>()
        .map(|&pos| Vector3::new(pos[0], pos[1], pos[2]))
        .collect();

    CornerTable::from_vertices_and_indices(&new_vertices, &faces)
}
