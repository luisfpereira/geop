use baby_shark::geometry::traits::RealNumber;
use baby_shark::mesh::corner_table::table::CornerTable;
use baby_shark::mesh::traits::{Mesh, TopologicalMesh};
use num_traits::Float;
use std::collections::HashMap;

use crate::utils::opposite_angle;

pub trait Laplacian {
    type ScalarType: RealNumber;

    fn laplacian(&self) -> HashMap<(usize, usize), Self::ScalarType>;
}

impl<TScalar: RealNumber + std::convert::From<i8>> Laplacian for CornerTable<TScalar> {
    type ScalarType = TScalar;

    fn laplacian(&self) -> HashMap<(usize, usize), Self::ScalarType> {
        // TODO: expand for Mesh + TopologicalMesh
        let mut sums = HashMap::new();

        for vertex in self.vertices() {
            let v1 = vertex;
            let mut outer_sum = Into::<Self::ScalarType>::into(0);

            self.edges_around_vertex(&vertex, |edge| {
                // TODO: take advantage of symmetry?

                let (v1_, v2_) = self.edge_vertices(edge);

                let v2 = if v1_ == v1 { v2_ } else { v1_ };

                let (face, another_face) = self.edge_faces(edge);

                let mut faces = Vec::new();
                faces.push(face);
                if another_face.is_some() {
                    faces.push(another_face.unwrap());
                }

                let mut inner_sum = Into::<Self::ScalarType>::into(0);
                for face in faces.iter() {
                    let (v1_, v2_, v3_) = self.face_vertices(&face);

                    let v3 = [v1_, v2_, v3_]
                        .into_iter()
                        .find(|&vertex| vertex != v1 && vertex != v2)
                        .unwrap();

                    let p1 = self.vertex_position(&v1);
                    let p2 = self.vertex_position(&v2);
                    let p3 = self.vertex_position(&v3);

                    let cotan_angle =
                        Into::<Self::ScalarType>::into(1) / Float::tan(opposite_angle(p1, p2, p3));

                    inner_sum += cotan_angle;
                }
                inner_sum /= Into::<Self::ScalarType>::into(2);

                sums.insert((v1, v2), inner_sum);
                outer_sum += inner_sum;
            });
            sums.insert((v1, v1), -outer_sum);
        }

        sums
    }
}
