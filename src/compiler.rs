use std::fmt;
use crate::obj::ObjMesh;


#[derive(Copy, Clone, Debug, PartialEq)]
enum Token {
    SymLet,
    SymPoints,
    SymTexCoords,
    SymNormals,
    SymTypeFloat32,
    SymTypeObjMesh,
    SymTypeVec,
    SymConstructor,
    SymVecMacro,
    Colon,
    Semicolon,
    LBracket,
    RBracket,
    LCurlyBrace,
    RCurlyBrace,
    GreaterThan,
    LessThan,
    Comma,
    LParen,
    RParen,
    Float32(f32),
}

#[derive(Clone, Debug, PartialEq)]
struct ObjMeshIR {
    data: Vec<Token>,
}

impl ObjMeshIR {
    fn new(data: Vec<Token>) -> ObjMeshIR {
        ObjMeshIR { data }
    }
}

fn compile(mesh: &ObjMesh) -> ObjMeshIR {
    unimplemented!("Compile has not been implemented yet!");
}

fn synthesize(ir: &ObjMeshIR) -> String {
    unimplemented!("Code synthesis has not been implemented yet!");
}

pub fn to_rust_code(mesh: &ObjMesh) -> String {
    let ir = compile(mesh);
    synthesize(&ir)
}

