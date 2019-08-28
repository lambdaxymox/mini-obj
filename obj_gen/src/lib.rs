extern crate obj;

use obj::ObjMesh;


pub fn to_rust_code(mesh: &ObjMesh) -> String {
    let ir = compile(mesh);
    synthesize(&ir)
}


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
    SymMacroVec,
    Equals,
    Colon,
    DoubleColon,
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

    fn len(&self) -> usize {
        self.data.len()
    }
}

fn compile(mesh: &ObjMesh) -> ObjMeshIR {
    use Token::*;
    
    let mut ir = vec![];
    // Start the code block.
    ir.push(LCurlyBrace);
    // Generate the points sets.
    ir.push(SymLet); 
    ir.push(SymPoints);
    ir.push(Colon);
    ir.push(SymTypeVec);
    ir.push(LessThan);
    ir.push(SymTypeFloat32);
    ir.push(GreaterThan);
    ir.push(Equals);
    ir.push(SymMacroVec);
    ir.push(LBracket);

    for point in mesh.points() {
        ir.push(LBracket);
        ir.push(Float32(point[0]));
        ir.push(Comma);
        ir.push(Float32(point[1]));
        ir.push(Comma);
        ir.push(Float32(point[2]));
        ir.push(RBracket);
        ir.push(Comma);
    }

    ir.push(RBracket);
    ir.push(Semicolon);

    // Generate the texture coordinates set.
    ir.push(SymLet); 
    ir.push(SymTexCoords);
    ir.push(Colon);
    ir.push(SymTypeVec);
    ir.push(LessThan);
    ir.push(SymTypeFloat32);
    ir.push(GreaterThan);
    ir.push(Equals);
    ir.push(SymMacroVec);
    ir.push(LBracket);

    for tex_coord in mesh.tex_coords() {
        ir.push(LBracket);
        ir.push(Float32(tex_coord[0]));
        ir.push(Comma);
        ir.push(Float32(tex_coord[1]));
        ir.push(RBracket);
        ir.push(Comma);
    }

    ir.push(RBracket);
    ir.push(Semicolon);

    // Generate the normal vector set.
    ir.push(SymLet); 
    ir.push(SymNormals);
    ir.push(Colon);
    ir.push(SymTypeVec);
    ir.push(LessThan);
    ir.push(SymTypeFloat32);
    ir.push(GreaterThan);
    ir.push(Equals);
    ir.push(SymMacroVec);
    ir.push(LBracket);

    for normal in mesh.normals() {
        ir.push(LBracket);
        ir.push(Float32(normal[0]));
        ir.push(Comma);
        ir.push(Float32(normal[1]));
        ir.push(Comma);
        ir.push(Float32(normal[2]));
        ir.push(RBracket);
        ir.push(Comma);
    }

    ir.push(RBracket);
    ir.push(Semicolon);

    // Generate the type constructor invocation.
    ir.push(SymTypeObjMesh);
    ir.push(DoubleColon);
    ir.push(SymConstructor);
    ir.push(LParen);
    ir.push(SymPoints); ir.push(Comma); ir.push(SymTexCoords); ir.push(Comma); ir.push(SymNormals);
    ir.push(RParen);

    // End the code block.    
    ir.push(RCurlyBrace);

    ObjMeshIR::new(ir)
}

fn synthesize_token(token: Token) -> String {
    use Token::*;
    match token {
        SymLet => format!("{}", "let"),
        SymPoints => format!("{}", "points"),
        SymTexCoords => format!("{}", "tex_coords"),
        SymNormals => format!("{}", "normals"),
        SymTypeFloat32 => format!("{}", "f32"),
        SymTypeObjMesh => format!("{}", "ObjMesh"),
        SymTypeVec => format!("{}", "Vec"),
        SymConstructor => format!("{}", "new"),
        SymMacroVec => format!("{}", "vec!"),
        Equals => format!("{}", "="),
        Colon => format!("{}", ":"),
        DoubleColon => format!("{}", "::"),
        Semicolon => format!("{}", ";"),
        LBracket => format!("{}", "["),
        RBracket => format!("{}", "]"),
        LCurlyBrace => format!("{}", "{"),
        RCurlyBrace => format!("{}", "}"),
        GreaterThan => format!("{}", ">"),
        LessThan => format!("{}", "<"),
        Comma => format!("{}", ","),
        LParen => format!("{}", "("),
        RParen => format!("{}", ")"),
        Float32(number) => format!("{:.*}", 8, number),
    }
}

fn synthesize(ir: &ObjMeshIR) -> String {
    use Token::*;

    let mut fragment = String::new();
    for token in ir.data.iter() {
        fragment.push_str(&synthesize_token(*token));
        fragment.push_str(&format!("{}", " "));
    }

    fragment
}


#[cfg(test)]
mod loader_tests {
    use super::{Token, ObjMeshIR};
    use crate::obj::ObjMesh;
    use crate::obj;
    use std::io::{BufReader, Cursor};

    struct Test {
        obj_file: String,
        obj_mesh: ObjMesh,
        ir: ObjMeshIR,
    }

    fn test() -> Test {
        let obj_file = String::from(r"        \
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

        let obj_mesh = ObjMesh {
            points: points,
            tex_coords: tex_coords,
            normals: normals,
        };

        use Token::*; 
        let ir = ObjMeshIR::new(vec![
            LCurlyBrace,
                SymLet, SymPoints, Colon, SymTypeVec, LessThan, SymTypeFloat32, GreaterThan, Equals, SymMacroVec, LBracket,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(1f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(1f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(1f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(1f32), Comma, Float32(1f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(1f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(1f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(1f32), Comma, Float32(1f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(1f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(1f32), Comma, Float32(1f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(1f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(1f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(1f32), Comma, Float32(1f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(1f32), Comma, Float32(1f32), RBracket, Comma,

                    LBracket, Float32(1f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(1f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(1f32), Comma, Float32(1f32), RBracket, Comma,

                    LBracket, Float32(1f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(1f32), Comma, Float32(1f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(0f32), Comma, Float32(1f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(0f32), Comma, Float32(1f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(0f32), Comma, Float32(1f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(1f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(1f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(0f32), Comma, Float32(1f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(1f32), Comma, Float32(1f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), Comma, Float32(1f32), RBracket, Comma,
                    LBracket, Float32(1f32), Comma, Float32(1f32), Comma, Float32(1f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(1f32), Comma, Float32(1f32), RBracket, Comma,
                RBracket, Semicolon,
                SymLet, SymTexCoords, Colon, SymTypeVec, LessThan, SymTypeFloat32, GreaterThan, Equals, SymMacroVec, LBracket,
                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,

                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                    LBracket, Float32(0f32), Comma, Float32(0f32), RBracket, Comma,
                RBracket, Semicolon,
                SymLet, SymNormals, Colon, SymTypeVec, LessThan, SymTypeFloat32, GreaterThan, Equals, SymMacroVec, LBracket,
                    LBracket, Float32( 0f32), Comma, Float32( 0f32), Comma, Float32(-1f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32( 0f32), Comma, Float32(-1f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32( 0f32), Comma, Float32(-1f32), RBracket, Comma,

                    LBracket, Float32( 0f32), Comma, Float32( 0f32), Comma, Float32(-1f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32( 0f32), Comma, Float32(-1f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32( 0f32), Comma, Float32(-1f32), RBracket, Comma,

                    LBracket, Float32(-1f32), Comma, Float32( 0f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32(-1f32), Comma, Float32( 0f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32(-1f32), Comma, Float32( 0f32), Comma, Float32( 0f32), RBracket, Comma,

                    LBracket, Float32(-1f32), Comma, Float32( 0f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32(-1f32), Comma, Float32( 0f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32(-1f32), Comma, Float32( 0f32), Comma, Float32( 0f32), RBracket, Comma,

                    LBracket, Float32( 0f32), Comma, Float32( 1f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32( 1f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32( 1f32), Comma, Float32( 0f32), RBracket, Comma,

                    LBracket, Float32( 0f32), Comma, Float32( 1f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32( 1f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32( 1f32), Comma, Float32( 0f32), RBracket, Comma,

                    LBracket, Float32( 1f32), Comma, Float32( 0f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32( 1f32), Comma, Float32( 0f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32( 1f32), Comma, Float32( 0f32), Comma, Float32( 0f32), RBracket, Comma,

                    LBracket, Float32( 1f32), Comma, Float32( 0f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32( 1f32), Comma, Float32( 0f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32( 1f32), Comma, Float32( 0f32), Comma, Float32( 0f32), RBracket, Comma,

                    LBracket, Float32( 0f32), Comma, Float32(-1f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32(-1f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32(-1f32), Comma, Float32( 0f32), RBracket, Comma,

                    LBracket, Float32( 0f32), Comma, Float32(-1f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32(-1f32), Comma, Float32( 0f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32(-1f32), Comma, Float32( 0f32), RBracket, Comma,

                    LBracket, Float32( 0f32), Comma, Float32( 0f32), Comma, Float32( 1f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32( 0f32), Comma, Float32( 1f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32( 0f32), Comma, Float32( 1f32), RBracket, Comma,

                    LBracket, Float32( 0f32), Comma, Float32( 0f32), Comma, Float32( 1f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32( 0f32), Comma, Float32( 1f32), RBracket, Comma,
                    LBracket, Float32( 0f32), Comma, Float32( 0f32), Comma, Float32( 1f32), RBracket, Comma,
                RBracket, Semicolon,
                SymTypeObjMesh, DoubleColon, SymConstructor, LParen, 
                    SymPoints, Comma, SymTexCoords, Comma, SymNormals, 
                RParen,
            RCurlyBrace,
        ]);

        Test {
            obj_file: obj_file,
            obj_mesh: obj_mesh,
            ir: ir,
        }
    }

    #[test]
    fn test_parse_obj_mesh() {
        let test = test();
        let mut reader = BufReader::new(Cursor::new(test.obj_file.as_bytes()));
        let result = obj::load(&mut reader).unwrap();
        let expected = test.obj_mesh;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_compile_obj_mesh() {
        let test = test();
        let mesh = test.obj_mesh;
        let result = super::compile(&mesh);
        let expected = test.ir;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_compile_obj_mesh_ir_length() {
        let test = test();
        let mesh = test.obj_mesh;
        let result = super::compile(&mesh);
        let expected = test.ir;

        assert_eq!(result.len(), expected.len());
    }

    #[test]
    fn test_compile_obj_mesh_elementwise() {
        let test = test();
        let mesh = test.obj_mesh;
        let result = super::compile(&mesh);
        let expected = test.ir;

        for (i, (result_token, expected_token)) 
            in result.data.iter().zip(expected.data.iter()).enumerate() {
            
            assert_eq!(result_token, expected_token, 
                "Token {} did not match what was expected. Got token `{:?}`. Expected token `{:?}`.",
                i, result_token, expected_token
            );
        }
    }
}
