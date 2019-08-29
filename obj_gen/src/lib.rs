extern crate obj;

use obj::ObjMesh;


pub fn to_rust_code(mesh: &ObjMesh) -> String {
    let ir = generate_code(mesh);
    synthesize_code(&ir)
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
    ArrayLength(usize),
    Newline,
    Whitespace(usize),
    POTATO,
}

#[derive(Clone, Debug, PartialEq)]
struct ObjMeshIR {
    data: Vec<Token>,
}

impl ObjMeshIR {
    fn new(data: Vec<Token>) -> ObjMeshIR {
        ObjMeshIR { data: data }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn push(&mut self, item: Token) {
        self.data.push(item);
    }
}

/// Generate the points set code for the object mesh.
fn generate_points_code(ir: &mut ObjMeshIR, mesh: &ObjMesh, indent: usize) {
    use Token::*;

    ir.push(Whitespace(indent));
    ir.push(SymLet);
    ir.push(Whitespace(1));
    ir.push(SymPoints);
    ir.push(Colon);
    ir.push(Whitespace(1));
    ir.push(SymTypeVec);
    ir.push(LessThan);

    ir.push(LBracket);
    ir.push(SymTypeFloat32); ir.push(Semicolon); ir.push(Whitespace(1)); ir.push(ArrayLength(3));
    ir.push(RBracket);

    ir.push(GreaterThan);
    ir.push(Whitespace(1));
    ir.push(Equals);
    ir.push(Whitespace(1));
    ir.push(SymMacroVec);
    ir.push(LBracket);
    ir.push(Newline);
    ir.push(Whitespace(indent));
    ir.push(Whitespace(indent));

    for point in mesh.points() {
        ir.push(LBracket);
        ir.push(Float32(point[0]));
        ir.push(Comma);
        ir.push(Whitespace(1));
        ir.push(Float32(point[1]));
        ir.push(Comma);
        ir.push(Whitespace(1));
        ir.push(Float32(point[2]));
        ir.push(RBracket);
        ir.push(Comma);
        ir.push(Whitespace(1));
    }

    ir.push(Newline);
    ir.push(Whitespace(indent));
    ir.push(RBracket);
    ir.push(Semicolon);
}

/// Generate the tex coordinates set code for the object mesh.
fn generate_tex_coords_code(ir: &mut ObjMeshIR, mesh: &ObjMesh, indent: usize) {
    use Token::*;

    ir.push(Whitespace(indent));
    ir.push(SymLet);
    ir.push(Whitespace(1)); 
    ir.push(SymTexCoords);
    ir.push(Colon);
    ir.push(Whitespace(1));
    ir.push(SymTypeVec);
    ir.push(LessThan);

    ir.push(LBracket); 
    ir.push(SymTypeFloat32); ir.push(Semicolon); ir.push(Whitespace(1)); ir.push(ArrayLength(2));
    ir.push(RBracket);

    ir.push(GreaterThan);
    ir.push(Whitespace(1));
    ir.push(Equals);
    ir.push(Whitespace(1));
    ir.push(SymMacroVec);
    ir.push(LBracket);
    ir.push(Newline);
    ir.push(Whitespace(indent));
    ir.push(Whitespace(indent));

    for tex_coord in mesh.tex_coords() {
        ir.push(LBracket);
        ir.push(Float32(tex_coord[0]));
        ir.push(Comma);
        ir.push(Whitespace(1));
        ir.push(Float32(tex_coord[1]));
        ir.push(RBracket);
        ir.push(Comma);
        ir.push(Whitespace(1));
    }

    ir.push(Newline);
    ir.push(Whitespace(indent));
    ir.push(RBracket);
    ir.push(Semicolon);
}

/// Generate the normal vector set code for the object mesh.
fn generate_normals_code(ir: &mut ObjMeshIR, mesh: &ObjMesh, indent: usize) {
    use Token::*;

    ir.push(Whitespace(indent));
    ir.push(SymLet);
    ir.push(Whitespace(1));
    ir.push(SymNormals);
    ir.push(Colon);
    ir.push(Whitespace(1));
    ir.push(SymTypeVec);
    ir.push(LessThan);

    ir.push(LBracket); 
    ir.push(SymTypeFloat32); ir.push(Semicolon); ir.push(Whitespace(1)); ir.push(ArrayLength(3)); 
    ir.push(RBracket);

    ir.push(GreaterThan);
    ir.push(Whitespace(1));
    ir.push(Equals);
    ir.push(Whitespace(1));
    ir.push(SymMacroVec);
    ir.push(LBracket);
    ir.push(Newline);
    ir.push(Whitespace(indent));
    ir.push(Whitespace(indent));

    for normal in mesh.normals() {
        ir.push(LBracket);
        ir.push(Float32(normal[0]));
        ir.push(Comma);
        ir.push(Whitespace(1));
        ir.push(Float32(normal[1]));
        ir.push(Comma);
        ir.push(Whitespace(1));
        ir.push(Float32(normal[2]));
        ir.push(RBracket);
        ir.push(Comma);
        ir.push(Whitespace(1));
    }

    ir.push(Newline);
    ir.push(Whitespace(indent));
    ir.push(RBracket);
    ir.push(Semicolon);
}

/// Generate the type constructor invocation code.
fn generate_type_constructor_invocation(ir: &mut ObjMeshIR, mesh: &ObjMesh, indent: usize) {
    use Token::*;

    ir.push(Whitespace(indent));
    ir.push(SymTypeObjMesh);
    ir.push(DoubleColon);
    ir.push(SymConstructor);
    ir.push(LParen);
    ir.push(SymPoints); ir.push(Comma); ir.push(Whitespace(1)); ir.push(SymTexCoords); ir.push(Comma); ir.push(Whitespace(1)); ir.push(SymNormals);
    ir.push(RParen);
}

/// Generate the Rust code expression block for constructing the 
/// object mesh at compile time.
fn generate_code(mesh: &ObjMesh) -> ObjMeshIR {
    use Token::*;
    
    let mut ir = ObjMeshIR::new(vec![]);
    let indent = 4;
    // Start the code block.
    ir.push(LCurlyBrace);
    ir.push(Newline);

    // Generate the points sets.
    generate_points_code(&mut ir, mesh, indent);
    ir.push(Newline);

    // Generate the texture coordinates set.
    generate_tex_coords_code(&mut ir, mesh, indent);
    ir.push(Newline);

    // Generate the normal vector set.
    generate_normals_code(&mut ir, mesh, indent);
    ir.push(Newline);
    ir.push(Newline);

    // Generate the type constructor invocation.
    generate_type_constructor_invocation(&mut ir, mesh, indent);
    ir.push(Newline);

    // End the code block.    
    ir.push(RCurlyBrace);

    ir
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
        ArrayLength(number) => format!("{}", number),
        Newline => format!("{}", "\n"),
        Whitespace(number) => format!("{:width$}", "", width = number),
        POTATO => format!(""),
    }
}

fn synthesize_code(ir: &ObjMeshIR) -> String {
    let mut fragment = String::new();
    for token in ir.data.iter() {
        fragment.push_str(&synthesize_token(*token));
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
            LCurlyBrace, Newline,
                Whitespace(4), SymLet, Whitespace(1), SymPoints, Colon, Whitespace(1), SymTypeVec, LessThan, 
                    LBracket, SymTypeFloat32, Semicolon, Whitespace(1), ArrayLength(3), RBracket, 
                GreaterThan, Whitespace(1), Equals, Whitespace(1), SymMacroVec, LBracket, 
                Newline, Whitespace(4), Whitespace(4),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(1f32), Comma, Whitespace(1), Float32(1f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(1f32), Comma, Whitespace(1), Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(1f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(1f32), Comma, Whitespace(1), Float32(1f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(1f32), Comma, Whitespace(1), Float32(1f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(1f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), Comma, Whitespace(1), Float32(1f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(1f32), Comma, Whitespace(1), Float32(1f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(1f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(1f32), Comma, Whitespace(1), Float32(1f32), Comma, Whitespace(1), Float32(1f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(1f32), Comma, Whitespace(1), Float32(1f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(1f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(1f32), Comma, Whitespace(1), Float32(1f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(1f32), Comma, Whitespace(1), Float32(1f32), Comma, Whitespace(1), Float32(1f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32(1f32), Comma, Whitespace(1), Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(1f32), Comma, Whitespace(1), Float32(1f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(1f32), Comma, Whitespace(1), Float32(1f32), Comma, Whitespace(1), Float32(1f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32(1f32), Comma, Whitespace(1), Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(1f32), Comma, Whitespace(1), Float32(1f32), Comma, Whitespace(1), Float32(1f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(1f32), Comma, Whitespace(1), Float32(0f32), Comma, Whitespace(1), Float32(1f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(1f32), Comma, Whitespace(1), Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(1f32), Comma, Whitespace(1), Float32(0f32), Comma, Whitespace(1), Float32(1f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(1f32), Comma, Whitespace(1), Float32(0f32), Comma, Whitespace(1), Float32(1f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), Comma, Whitespace(1), Float32(1f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), Comma, Whitespace(1), Float32(1f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(1f32), Comma, Whitespace(1), Float32(0f32), Comma, Whitespace(1), Float32(1f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(1f32), Comma, Whitespace(1), Float32(1f32), Comma, Whitespace(1), Float32(1f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), Comma, Whitespace(1), Float32(1f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(1f32), Comma, Whitespace(1), Float32(1f32), Comma, Whitespace(1), Float32(1f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(1f32), Comma, Whitespace(1), Float32(1f32), RBracket, Comma, Whitespace(1),
                Newline, Whitespace(4), RBracket, Semicolon, Newline,

                Whitespace(4), SymLet, Whitespace(1), SymTexCoords, Colon, Whitespace(1), SymTypeVec, LessThan,
                    LBracket, SymTypeFloat32, Semicolon, Whitespace(1), ArrayLength(2), RBracket, 
                GreaterThan, Whitespace(1), Equals, Whitespace(1), SymMacroVec, LBracket,
                Newline, Whitespace(4), Whitespace(4),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(0f32), Comma, Whitespace(1), Float32(0f32), RBracket, Comma, Whitespace(1),
                Newline, Whitespace(4), RBracket, Semicolon, Newline,
                
                Whitespace(4), SymLet, Whitespace(1), SymNormals, Colon, Whitespace(1), SymTypeVec, LessThan, 
                    LBracket, SymTypeFloat32, Semicolon, Whitespace(1), ArrayLength(3), RBracket, 
                GreaterThan, Whitespace(1), Equals, Whitespace(1), SymMacroVec, LBracket, 
                Newline, Whitespace(4), Whitespace(4),
                    LBracket, Float32( 0f32), Comma, Whitespace(1), Float32( 0f32), Comma, Whitespace(1), Float32(-1f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32( 0f32), Comma, Whitespace(1), Float32( 0f32), Comma, Whitespace(1), Float32(-1f32), RBracket, Comma, Whitespace(1), 
                    LBracket, Float32( 0f32), Comma, Whitespace(1), Float32( 0f32), Comma, Whitespace(1), Float32(-1f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32( 0f32), Comma, Whitespace(1), Float32( 0f32), Comma, Whitespace(1), Float32(-1f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32( 0f32), Comma, Whitespace(1), Float32( 0f32), Comma, Whitespace(1), Float32(-1f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32( 0f32), Comma, Whitespace(1), Float32( 0f32), Comma, Whitespace(1), Float32(-1f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32(-1f32), Comma, Whitespace(1), Float32( 0f32), Comma, Whitespace(1), Float32( 0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(-1f32), Comma, Whitespace(1), Float32( 0f32), Comma, Whitespace(1), Float32( 0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(-1f32), Comma, Whitespace(1), Float32( 0f32), Comma, Whitespace(1), Float32( 0f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32(-1f32), Comma, Whitespace(1), Float32( 0f32), Comma, Whitespace(1), Float32( 0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(-1f32), Comma, Whitespace(1), Float32( 0f32), Comma, Whitespace(1), Float32( 0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32(-1f32), Comma, Whitespace(1), Float32( 0f32), Comma, Whitespace(1), Float32( 0f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32( 0f32), Comma, Whitespace(1), Float32( 1f32), Comma, Whitespace(1), Float32( 0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32( 0f32), Comma, Whitespace(1), Float32( 1f32), Comma, Whitespace(1), Float32( 0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32( 0f32), Comma, Whitespace(1), Float32( 1f32), Comma, Whitespace(1), Float32( 0f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32( 0f32), Comma, Whitespace(1), Float32( 1f32), Comma, Whitespace(1), Float32( 0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32( 0f32), Comma, Whitespace(1), Float32( 1f32), Comma, Whitespace(1), Float32( 0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32( 0f32), Comma, Whitespace(1), Float32( 1f32), Comma, Whitespace(1), Float32( 0f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32( 1f32), Comma, Whitespace(1), Float32( 0f32), Comma, Whitespace(1), Float32( 0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32( 1f32), Comma, Whitespace(1), Float32( 0f32), Comma, Whitespace(1), Float32( 0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32( 1f32), Comma, Whitespace(1), Float32( 0f32), Comma, Whitespace(1), Float32( 0f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32( 1f32), Comma, Whitespace(1), Float32( 0f32), Comma, Whitespace(1), Float32( 0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32( 1f32), Comma, Whitespace(1), Float32( 0f32), Comma, Whitespace(1), Float32( 0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32( 1f32), Comma, Whitespace(1), Float32( 0f32), Comma, Whitespace(1), Float32( 0f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32( 0f32), Comma, Whitespace(1), Float32(-1f32), Comma, Whitespace(1), Float32( 0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32( 0f32), Comma, Whitespace(1), Float32(-1f32), Comma, Whitespace(1), Float32( 0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32( 0f32), Comma, Whitespace(1), Float32(-1f32), Comma, Whitespace(1), Float32( 0f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32( 0f32), Comma, Whitespace(1), Float32(-1f32), Comma, Whitespace(1), Float32( 0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32( 0f32), Comma, Whitespace(1), Float32(-1f32), Comma, Whitespace(1), Float32( 0f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32( 0f32), Comma, Whitespace(1), Float32(-1f32), Comma, Whitespace(1), Float32( 0f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32( 0f32), Comma, Whitespace(1), Float32( 0f32), Comma, Whitespace(1), Float32( 1f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32( 0f32), Comma, Whitespace(1), Float32( 0f32), Comma, Whitespace(1), Float32( 1f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32( 0f32), Comma, Whitespace(1), Float32( 0f32), Comma, Whitespace(1), Float32( 1f32), RBracket, Comma, Whitespace(1),

                    LBracket, Float32( 0f32), Comma, Whitespace(1), Float32( 0f32), Comma, Whitespace(1), Float32( 1f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32( 0f32), Comma, Whitespace(1), Float32( 0f32), Comma, Whitespace(1), Float32( 1f32), RBracket, Comma, Whitespace(1),
                    LBracket, Float32( 0f32), Comma, Whitespace(1), Float32( 0f32), Comma, Whitespace(1), Float32( 1f32), RBracket, Comma, Whitespace(1),
                Newline, Whitespace(4), RBracket, Semicolon, Newline,
                Newline, Whitespace(4),
                SymTypeObjMesh, DoubleColon, SymConstructor, LParen, 
                    SymPoints, Comma, Whitespace(1), SymTexCoords, Comma, Whitespace(1), SymNormals, 
                RParen, Newline,
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
        let result = super::generate_code(&mesh);
        let expected = test.ir;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_compile_obj_mesh_ir_length() {
        let test = test();
        let mesh = test.obj_mesh;
        let result = super::generate_code(&mesh);
        let expected = test.ir;

        assert_eq!(result.len(), expected.len());
    }

    #[test]
    fn test_compile_obj_mesh_elementwise() {
        let test = test();
        let mesh = test.obj_mesh;
        let result = super::generate_code(&mesh);
        let expected = test.ir;

        for (i, (result_token, expected_token)) 
            in result.data.iter().zip(expected.data.iter()).enumerate() {
            
            assert_eq!(result_token, expected_token, 
                "Token {} did not match what was expected. Got token `{:?}`. Expected token `{:?}`.
                 Result Context: {:?}
                 Expected Context: {:?}\n",
                i, result_token, expected_token, &result.data[i-10..i+10], &expected.data[i-10..i+10],
            );
        }
    }
}
