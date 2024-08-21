use baby_shark::geometry::traits::RealNumber;
use baby_shark::mesh::corner_table::table::CornerTable;
use baby_shark::mesh::traits::Mesh;
use nalgebra::Vector3;

// TODO: use real number from num_traits instead?
pub struct SharedVertexMesh<ScalarType: RealNumber> {
    pub vertices: Vec<ScalarType>,
    pub faces: Vec<usize>,
}

impl<ScalarType: RealNumber> From<SharedVertexMesh<ScalarType>> for CornerTable<ScalarType> {
    fn from(mesh: SharedVertexMesh<ScalarType>) -> CornerTable<ScalarType> {
        let new_vertices: Vec<Vector3<ScalarType>> = mesh
            .vertices
            .array_chunks::<3>()
            .map(|&pos| Vector3::new(pos[0], pos[1], pos[2]))
            .collect();

        CornerTable::<ScalarType>::from_vertices_and_indices(&new_vertices, &mesh.faces)
    }
}
