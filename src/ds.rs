use baby_shark::geometry::primitives::triangle3::Triangle3;
use baby_shark::geometry::traits::RealNumber;
use baby_shark::mesh::corner_table::table::CornerTable;
use baby_shark::mesh::traits::Mesh;
use lox::core::BasicAdj;
use lox::map::PropStoreMut;
use lox::FaceHandle;
use lox::{
    core::DirectedEdgeMesh as TopologicalDirectedEdgeMesh,
    core::SharedVertexMesh as TopologicalSharedVertexMesh,
    core::{HalfEdgeMesh as TopologicalHalfEdgeMesh, MeshMut},
    map::DenseMap,
    prelude::Empty,
    VertexHandle,
};

use nalgebra::Vector3;

// TODO: use real number from num_traits instead?
pub struct SharedVertexMeshData<ScalarType: RealNumber> {
    pub vertices: Vec<ScalarType>,
    pub faces: Vec<usize>,
}

pub struct AbstractMesh<Topology> {
    pub topology: Topology,
    pub vertex_positions: DenseMap<VertexHandle, [f64; 3]>,
}

pub type HalfEdgeMesh = AbstractMesh<TopologicalHalfEdgeMesh>;
pub type DirectedEdgeMesh = AbstractMesh<TopologicalDirectedEdgeMesh>;
pub type SharedVertexMesh = AbstractMesh<TopologicalSharedVertexMesh>;

pub trait VertexPos {
    fn face_polygon(&self, face: FaceHandle) -> Triangle3<f64>;
}

impl<Topology: BasicAdj> VertexPos for AbstractMesh<Topology> {
    fn face_polygon(&self, face: FaceHandle) -> Triangle3<f64> {
        let vertex_positions: Vec<[f64; 3]> = self
            .topology
            .vertices_around_face(face) // TODO: use .vertices_around_triangle
            .map(|face_vertex| self.vertex_positions[face_vertex])
            .collect();

        // TODO: improve here by creating own triangle?
        Triangle3::new(
            Vector3::from(vertex_positions[0]),
            Vector3::from(vertex_positions[1]),
            Vector3::from(vertex_positions[2]),
        )
    }
}

pub trait FromSharedVertex {
    type ScalarType;

    fn from_vertices_and_faces(vertices: &[Self::ScalarType], faces: &[usize]) -> Self;
}

impl<TScalar: RealNumber> FromSharedVertex for CornerTable<TScalar> {
    type ScalarType = TScalar;

    fn from_vertices_and_faces(vertices: &[Self::ScalarType], faces: &[usize]) -> Self {
        let new_vertices: Vec<Vector3<Self::ScalarType>> = vertices
            .array_chunks::<3>()
            .map(|&pos| Vector3::new(pos[0], pos[1], pos[2]))
            .collect();

        Self::from_vertices_and_indices(&new_vertices, faces)
    }
}

impl<Topology> FromSharedVertex for AbstractMesh<Topology>
where
    Topology: Empty + MeshMut,
{
    type ScalarType = f64;

    fn from_vertices_and_faces(vertices: &[Self::ScalarType], faces: &[usize]) -> Self {
        let mut top_mesh = <Topology>::empty();

        let mut vertex_positions = DenseMap::new();
        let mut vertices_ = Vec::new();
        vertices.array_chunks::<3>().for_each(|vertex| {
            let vertex_handle = top_mesh.add_vertex();
            vertices_.push(vertex_handle);
            vertex_positions.insert(vertex_handle, *vertex);
        });

        faces.array_chunks::<3>().for_each(|pos| {
            top_mesh.add_triangle([vertices_[pos[0]], vertices_[pos[1]], vertices_[pos[2]]]);
        });

        Self {
            topology: top_mesh,
            vertex_positions,
        }
    }
}

impl<Topology> From<SharedVertexMeshData<f64>> for AbstractMesh<Topology>
where
    Topology: Empty + MeshMut,
{
    fn from(mesh: SharedVertexMeshData<f64>) -> Self {
        Self::from_vertices_and_faces(&mesh.vertices, &mesh.faces)
    }
}

impl<ScalarType: RealNumber> From<SharedVertexMeshData<ScalarType>> for CornerTable<ScalarType> {
    fn from(mesh: SharedVertexMeshData<ScalarType>) -> Self {
        Self::from_vertices_and_faces(&mesh.vertices, &mesh.faces)
    }
}

// // TODO: find a way to make this work
// impl<TMesh> From<SharedVertexMeshData<f64>> for TMesh
// where
//     TMesh: FromSharedVertex<TMesh = TMesh, ScalarType = f64>,
// {
//     fn from(mesh: SharedVertexMeshData<f64>) -> Self {
//         Self::from_vertices_and_faces(&mesh.vertices, &mesh.faces)
//     }
// }

// TODO: can be made much more general
impl From<SharedVertexMeshData<f64>> for TopologicalHalfEdgeMesh {
    fn from(mesh: SharedVertexMeshData<f64>) -> TopologicalHalfEdgeMesh {
        let mut new_mesh = <TopologicalHalfEdgeMesh>::empty();

        let n_vertices = mesh.vertices.len();

        let mut vertices = Vec::new();
        (0..n_vertices).for_each(|_| vertices.push(new_mesh.add_vertex()));

        mesh.faces.array_chunks::<3>().for_each(|pos| {
            new_mesh.add_triangle([vertices[pos[0]], vertices[pos[1]], vertices[pos[2]]]);
        });

        new_mesh
    }
}
