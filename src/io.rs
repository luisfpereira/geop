use baby_shark::geometry::traits::RealNumber;
use std::path::Path;

#[derive(Debug)]
pub struct OffReader {
    pub limits: off_rs::parser::options::Limits,
}

impl OffReader {
    fn get_options(&self) -> off_rs::parser::options::Options {
        off_rs::parser::options::Options {
            limits: self.limits,
            ..Default::default()
        }
    }

    pub fn read<S>(&self, filename: &str) -> (Vec<S>, Vec<usize>)
    where
        S: RealNumber + std::convert::From<f32>,
    {
        let off_path = Path::new(filename);
        let mesh = off_rs::from_path(off_path, self.get_options()).expect("Can't read mesh.");

        let faces = mesh
            .faces
            .into_iter()
            .flat_map(|face| face.vertices)
            .collect();

        let vertices = mesh
            .vertices
            .iter()
            .flat_map(|vertex| {
                [
                    Into::<S>::into(vertex.position.x),
                    Into::<S>::into(vertex.position.y),
                    Into::<S>::into(vertex.position.z),
                ]
            })
            .collect();

        (vertices, faces)
    }
}

impl Default for OffReader {
    fn default() -> Self {
        let limits = off_rs::parser::options::Limits {
            vertex_count: 10000,
            face_count: 20000,
            face_vertex_count: 3,
        };
        Self { limits }
    }
}
