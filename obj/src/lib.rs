#![allow(dead_code)]
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::mem;
use std::path::Path;

use wavefront_obj::obj;
use wavefront_obj::obj::{
    Element, 
    VTNTriple,
};


#[derive(Clone, Debug, PartialEq)]
pub struct Points {
    inner: Vec<[f32; 3]>,
}

impl Points {
    #[inline]
    pub fn as_ptr(&self) -> *const [f32; 3] {
        self.inner.as_ptr()
    }

    /// Get the length of the points buffer in bytes.
    #[inline]
    pub fn len_bytes(&self) -> usize {
        3 * mem::size_of::<f32>() * self.inner.len()
    }

    /// Get the number of elements in the points buffer.
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextureCoordinates {
    inner: Vec<[f32; 2]>,
}

impl TextureCoordinates {
    #[inline]
    pub fn as_ptr(&self) -> *const [f32; 2] {
        self.inner.as_ptr()
    }

    /// Get the length of the texture coordinates buffer in bytes.
    #[inline]
    pub fn len_bytes(&self) -> usize {
        2 * mem::size_of::<f32>() * self.inner.len()
    }

    /// Get the number of elements in the texture coordinates buffer.
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Normals {
    inner: Vec<[f32; 3]>,
}

impl Normals {
    #[inline]
    pub fn as_ptr(&self) -> *const [f32; 3] {
        self.inner.as_ptr()
    }

    /// Get the length of the normal vector buffer in bytes.
    #[inline]
    pub fn len_bytes(&self) -> usize {
        3 * mem::size_of::<f32>() * self.inner.len()
    }

    /// Get the number of elements in the normal vector buffer.
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

/// An `ObjMesh` is a model space representation of a 3D geometric figure.
/// You typically generate one from parsing a Wavefront *.obj file into
/// an `ObjMesh`.
#[derive(Clone, Debug, PartialEq)]
pub struct ObjMesh {
    pub points: Points,
    pub tex_coords: TextureCoordinates,
    pub normals: Normals,
}

impl ObjMesh {
    /// Generate a new mesh object.
    pub fn new(points: Vec<[f32; 3]>, tex_coords: Vec<[f32; 2]>, normals: Vec<[f32; 3]>) -> ObjMesh {
        ObjMesh {
            points: Points { inner: points },
            tex_coords: TextureCoordinates { inner: tex_coords },
            normals: Normals { inner: normals },
        }
    }

    /// Present the points map as an array slice. This function can be used
    /// to present the internal array buffer to OpenGL or another Graphics
    /// system for rendering.
    #[inline]
    pub fn points(&self) -> &[[f32; 3]] {
        &self.points.inner
    }

    /// Present the texture map as an array slice. This function can be used
    /// to present the internal array buffer to OpenGL or another Graphics
    /// system for rendering.
    #[inline]
    pub fn tex_coords(&self) -> &[[f32; 2]] {
        &self.tex_coords.inner
    }

    /// Present the normal vector map as an array slice. This function can be used
    /// to present the internal array buffer to OpenGL or another Graphics
    /// system for rendering.
    #[inline]
    pub fn normals(&self) -> &[[f32; 3]] {
        &self.normals.inner
    }

    /// Get the number of vertices in the mesh.
    #[inline]
    pub fn len(&self) -> usize {
        self.points.len()
    }
}

pub fn load<R: io::Read>(reader: &mut R) -> Result<ObjMesh, String> {
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).expect("Read error occurred.");
    let object_set = obj::parse(&buffer).expect("Parse error occurred.");
    let object = &object_set.objects[0];

    let mut vertices = vec![];
    let mut tex_coords = vec![];
    let mut normals = vec![];
    for element in object.element_set.iter() {
        match element {
            Element::Face(vtn1, vtn2, vtn3) => {
                let triples = [
                    object.get_vtn_triple(*vtn1).unwrap(),
                    object.get_vtn_triple(*vtn2).unwrap(),
                    object.get_vtn_triple(*vtn3).unwrap(),
                ];

                for triple in triples.iter() {
                    match triple {
                        VTNTriple::V(vp) => {
                            vertices.push([vp.x as f32, vp.y as f32, vp.z as f32]);
                            tex_coords.push([0_f32, 0_f32]);
                            normals.push([0_f32, 0_f32, 0_f32]);
                        }
                        VTNTriple::VT(vp, vt) => {
                            vertices.push([vp.x as f32, vp.y as f32, vp.z as f32]);
                            tex_coords.push([vt.u as f32, vt.v as f32]);
                            normals.push([0_f32, 0_f32, 0_f32]);
                        }
                        VTNTriple::VN(vp, vn) => {
                            vertices.push([vp.x as f32, vp.y as f32, vp.z as f32]);
                            tex_coords.push([0_f32, 0_f32]);
                            normals.push([vn.x as f32, vn.y as f32, vn.z as f32]);
                        }
                        VTNTriple::VTN(vp, vt, vn) => {
                            vertices.push([vp.x as f32, vp.y as f32, vp.z as f32]);
                            tex_coords.push([vt.u as f32, vt.v as f32]);
                            normals.push([vn.x as f32, vn.y as f32, vn.z as f32]);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(ObjMesh::new(vertices, tex_coords, normals))
}

pub fn load_from_memory(buffer: &[u8]) -> Result<ObjMesh, String> {
    let mut reader = BufReader::new(buffer);
    load(&mut reader)
}

pub fn load_file<P: AsRef<Path>>(path: P) -> Result<ObjMesh, String> {
    let file = match File::open(path.as_ref()) {
        Ok(handle) => handle,
        Err(_) => {
            return Err(format!("ERROR: file not found: {}", path.as_ref().display()));
        }
    };

    let mut reader = BufReader::new(file);
    load(&mut reader)
}


#[cfg(test)]
mod loader_tests {
    use super::ObjMesh;
    use std::io::{BufReader, Cursor};

    struct Test {
        obj_file: String,
        obj_mesh: ObjMesh,
    }

    fn test() -> Test {
        let obj_file = String::from(r" \
            o object1                         \
            g cube                            \
            v  0.0  0.0  0.0                  \
            v  0.0  0.0  1.0                  \
            v  0.0  1.0  0.0                  \
            v  0.0  1.0  1.0                  \
            v  1.0  0.0  0.0                  \
            v  1.0  0.0  1.0                  \
            v  1.0  1.0  0.0                  \
            v  1.0  1.0  1.0                  \
                                              \
            vn  0.0  0.0  1.0                 \
            vn  0.0  0.0 -1.0                 \
            vn  0.0  1.0  0.0                 \
            vn  0.0 -1.0  0.0                 \
            vn  1.0  0.0  0.0                 \
            vn -1.0  0.0  0.0                 \
                                              \
            f  1//2  7//2  5//2               \
            f  1//2  3//2  7//2               \
            f  1//6  4//6  3//6               \
            f  1//6  2//6  4//6               \
            f  3//3  8//3  7//3               \
            f  3//3  4//3  8//3               \
            f  5//5  7//5  8//5               \
            f  5//5  8//5  6//5               \
            f  1//4  5//4  6//4               \
            f  1//4  6//4  2//4               \
            f  2//1  6//1  8//1               \
            f  2//1  8//1  4//1               \
        ");
        let points = vec![
            [0.0, 0.0, 0.0], [1.0, 1.0, 0.0], [1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0],
            [0.0, 0.0, 0.0], [0.0, 1.0, 1.0], [0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 1.0],
            [0.0, 1.0, 0.0], [1.0, 1.0, 1.0], [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0], [0.0, 1.0, 1.0], [1.0, 1.0, 1.0],
            [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [1.0, 1.0, 1.0],
            [1.0, 0.0, 0.0], [1.0, 1.0, 1.0], [1.0, 0.0, 1.0],
            [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0],
            [0.0, 0.0, 0.0], [1.0, 0.0, 1.0], [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0], [1.0, 0.0, 1.0], [1.0, 1.0, 1.0],
            [0.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0],
        ];
        let tex_coords = vec![
            [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],
            [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],
            [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],
            [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],
            [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],
            [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],
            [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],
            [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],
            [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],
            [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],
            [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],
            [0.0, 0.0], [0.0, 0.0], [0.0, 0.0],
        ];
        let normals = vec![
            [ 0.0,  0.0, -1.0], [ 0.0,  0.0, -1.0], [ 0.0,  0.0, -1.0],
            [ 0.0,  0.0, -1.0], [ 0.0,  0.0, -1.0], [ 0.0,  0.0, -1.0],
            [-1.0,  0.0,  0.0], [-1.0,  0.0,  0.0], [-1.0,  0.0,  0.0],
            [-1.0,  0.0,  0.0], [-1.0,  0.0,  0.0], [-1.0,  0.0,  0.0],
            [ 0.0,  1.0,  0.0], [ 0.0,  1.0,  0.0], [ 0.0,  1.0,  0.0],
            [ 0.0,  1.0,  0.0], [ 0.0,  1.0,  0.0], [ 0.0,  1.0,  0.0],
            [ 1.0,  0.0,  0.0], [ 1.0,  0.0,  0.0], [ 1.0,  0.0,  0.0],
            [ 1.0,  0.0,  0.0], [ 1.0,  0.0,  0.0], [ 1.0,  0.0,  0.0],
            [ 0.0, -1.0,  0.0], [ 0.0, -1.0,  0.0], [ 0.0, -1.0,  0.0],
            [ 0.0, -1.0,  0.0], [ 0.0, -1.0,  0.0], [ 0.0, -1.0,  0.0],
            [ 0.0,  0.0,  1.0], [ 0.0,  0.0,  1.0], [ 0.0,  0.0,  1.0],
            [ 0.0,  0.0,  1.0], [ 0.0,  0.0,  1.0], [ 0.0,  0.0,  1.0],
        ];

        let obj_mesh = ObjMesh::new(points, tex_coords, normals);

        Test {
            obj_file: obj_file,
            obj_mesh: obj_mesh,
        }
    }

    #[test]
    fn test_parse_obj_mesh_elementwise() {
        let test = test();
        let mut reader = BufReader::new(Cursor::new(test.obj_file.as_bytes()));
        let result = super::load(&mut reader).unwrap();
        let expected = test.obj_mesh;

        assert_eq!(result.points, expected.points);
        assert_eq!(result.tex_coords, expected.tex_coords);
        assert_eq!(result.normals, expected.normals);
    }

    #[test]
    fn test_parse_obj_mesh() {
        let test = test();
        let mut reader = BufReader::new(Cursor::new(test.obj_file.as_bytes()));
        let result = super::load(&mut reader).unwrap();
        let expected = test.obj_mesh;

        assert_eq!(result, expected);
    }
}
