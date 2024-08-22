use crate::ds::{AbstractMesh, VertexPos};
use crate::utils::opposite_angle;
use baby_shark::geometry::traits::RealNumber;
use baby_shark::mesh::corner_table::table::CornerTable;
use baby_shark::mesh::traits::{Mesh as BabyMesh, TopologicalMesh as BabyTopologicalMesh};
use lox::core::{BasicAdj, EdgeAdj, FullAdj, Mesh as LoxMesh};
use lox::Handle;
use nalgebra::Vector3;
use num_traits::Float;
use std::collections::HashMap;

pub trait Laplacian {
    type ScalarType: RealNumber;

    fn laplace_matrix(&self) -> HashMap<(usize, usize), Self::ScalarType>;
    fn mass_matrix(&self) -> Vec<Self::ScalarType>;
}

impl<TScalar: RealNumber + std::convert::From<i8>> Laplacian for CornerTable<TScalar> {
    type ScalarType = TScalar;

    fn laplace_matrix(&self) -> HashMap<(usize, usize), Self::ScalarType> {
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

    fn mass_matrix(&self) -> Vec<Self::ScalarType> {
        let mut areas = Vec::new();

        for vertex in self.vertices() {
            let mut area = Into::<Self::ScalarType>::into(0);
            self.faces_around_vertex(&vertex, |face| area += self.face_positions(face).get_area());
            areas.push(area / Into::<Self::ScalarType>::into(3))
        }
        areas
    }
}

impl<Topology> Laplacian for AbstractMesh<Topology>
where
    Topology: LoxMesh + BasicAdj + EdgeAdj + FullAdj,
{
    type ScalarType = f64;

    fn laplace_matrix(&self) -> HashMap<(usize, usize), Self::ScalarType> {
        let top = &self.topology;

        let mut sums = HashMap::new();

        top.vertices().for_each(|v1| {
            let mut outer_sum = 0.;
            top.vertices_around_vertex(v1.handle()).for_each(|v2| {
                let mut inner_sum = 0.;
                let edge = top.edge_between_vertices(v1.handle(), v2).unwrap();
                top.faces_of_edge(edge).iter().for_each(|&face| {
                    let v3 = top
                        .vertices_around_face(face)
                        .filter(|&v3| v3 != v1.handle() && v3 != v2)
                        .last()
                        .unwrap();

                    // TODO: improve here
                    let p1 = Vector3::from(self.vertex_positions[v1.handle()]);
                    let p2 = Vector3::from(self.vertex_positions[v2]);
                    let p3 = Vector3::from(self.vertex_positions[v3]);

                    let cotan_angle = 1. / opposite_angle(&p1, &p2, &p3).tan();

                    inner_sum += cotan_angle;
                });
                inner_sum /= 2.;
                sums.insert((v1.handle().to_usize(), v2.to_usize()), inner_sum);

                outer_sum += inner_sum;
            });
            sums.insert((v1.handle().to_usize(), v1.handle().to_usize()), -outer_sum);
        });

        sums
    }

    fn mass_matrix(&self) -> Vec<Self::ScalarType> {
        let top = &self.topology;

        top.vertices()
            .map(|vertex| {
                let mut area: f64 = top
                    .faces_around_vertex(vertex.handle())
                    .map(|face| self.face_polygon(face).get_area())
                    .sum();
                area /= 3.;

                area
            })
            .collect()
    }
}
