use baby_shark::geometry::primitives::triangle3::Triangle3;
use baby_shark::geometry::traits::RealNumber;
use baby_shark::mesh::corner_table::table::CornerTable;
use baby_shark::mesh::traits::Mesh;
use lox::core::BasicAdj;
use lox::map::PropStoreMut;
use lox::FaceHandle;
use lox::{
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

// TODO: can be made much more general
pub struct HalfEdgeMesh {
    pub topology: TopologicalHalfEdgeMesh,
    pub vertex_positions: DenseMap<VertexHandle, [f64; 3]>,
}

// TODO: needs to be brough in as trait, get inspired by baby shark
impl HalfEdgeMesh {
    pub fn face_polygon(&self, face: FaceHandle) -> Triangle3<f64> {
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

impl<ScalarType: RealNumber> From<SharedVertexMeshData<ScalarType>> for CornerTable<ScalarType> {
    fn from(mesh: SharedVertexMeshData<ScalarType>) -> CornerTable<ScalarType> {
        let new_vertices: Vec<Vector3<ScalarType>> = mesh
            .vertices
            .array_chunks::<3>()
            .map(|&pos| Vector3::new(pos[0], pos[1], pos[2]))
            .collect();

        CornerTable::<ScalarType>::from_vertices_and_indices(&new_vertices, &mesh.faces)
    }
}

impl From<SharedVertexMeshData<f64>> for HalfEdgeMesh {
    fn from(mesh: SharedVertexMeshData<f64>) -> HalfEdgeMesh {
        let mut top_mesh = <TopologicalHalfEdgeMesh>::empty();

        let mut vertex_positions = DenseMap::new();
        let mut vertices = Vec::new();
        mesh.vertices.array_chunks::<3>().for_each(|vertex| {
            let vertex_handle = top_mesh.add_vertex();
            vertices.push(vertex_handle);
            vertex_positions.insert(vertex_handle, *vertex);
        });

        mesh.faces.array_chunks::<3>().for_each(|pos| {
            top_mesh.add_triangle([vertices[pos[0]], vertices[pos[1]], vertices[pos[2]]]);
        });

        HalfEdgeMesh {
            topology: top_mesh,
            vertex_positions,
        }
    }
}

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
