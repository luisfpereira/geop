#![feature(array_chunks)]

pub mod io;

use std::collections::HashMap;

use baby_shark::geometry::traits::RealNumber;
use baby_shark::mesh::corner_table::table::CornerTable;
use nalgebra::Vector3;
use num_traits::Float;

use baby_shark::mesh::traits::{Mesh, TopologicalMesh};

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

pub fn opposite_angle<S>(p1: &Vector3<S>, p2: &Vector3<S>, p3: &Vector3<S>) -> S
where
    S: RealNumber + std::convert::From<i8>,
{
    let a_squared = (p1 - p2).norm_squared();
    let b_squared = (p2 - p3).norm_squared();
    let c_squared = (p3 - p1).norm_squared();

    Float::acos(
        (b_squared + c_squared - a_squared)
            / (Into::<S>::into(2 as i8) * Float::sqrt(b_squared) * Float::sqrt(c_squared)),
    )
}

// TODO: as trait
// TODO: create operators

pub fn mesh_laplacian<S>(mesh: CornerTable<S>) -> HashMap<(usize, usize), S>
where
    S: RealNumber + std::convert::From<i8>,
{
    // TODO: expand for Mesh + TopologicalMesh
    let mut sums = HashMap::new();

    for vertex in mesh.vertices() {
        let v1 = vertex;
        let mut outer_sum = Into::<S>::into(0);

        mesh.edges_around_vertex(&vertex, |edge| {
            // TODO: take advantage of symmetry?

            let (v1_, v2_) = mesh.edge_vertices(edge);

            let v2 = if v1_ == v1 { v2_ } else { v1_ };

            let (face, another_face) = mesh.edge_faces(edge);

            let mut faces = Vec::new();
            faces.push(face);
            if another_face.is_some() {
                faces.push(another_face.unwrap());
            }

            let mut inner_sum = Into::<S>::into(0);
            for face in faces.iter() {
                let (v1_, v2_, v3_) = mesh.face_vertices(&face);

                let v3 = [v1_, v2_, v3_]
                    .into_iter()
                    .find(|&vertex| vertex != v1 && vertex != v2)
                    .unwrap();

                let p1 = mesh.vertex_position(&v1);
                let p2 = mesh.vertex_position(&v2);
                let p3 = mesh.vertex_position(&v3);

                let cotan_angle = Into::<S>::into(1) / Float::tan(opposite_angle(p1, p2, p3));

                inner_sum += cotan_angle;
            }
            inner_sum /= Into::<S>::into(2);

            sums.insert((v1, v2), inner_sum);
            outer_sum += inner_sum;
        });
        sums.insert((v1, v1), -outer_sum);
    }

    sums
}
