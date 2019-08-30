extern crate obj;
extern crate obj_gen;

use std::fs::File;
use std::io;
use std::path::Path;
use std::io::Write;


fn generate_code_fragment<P: AsRef<Path>>(path: P) -> String {
    let model = obj::load_file(path).unwrap();
    let fragment = obj_gen::to_rust_code(&model);

    fragment
}

fn write_code_fragment(fragment: &str, fragment_name: &str) -> io::Result<()> {
    let path = Path::new("tests").join(fragment_name);
    let mut file = File::create(&path)?;
    file.write_all(fragment.as_bytes())?;
    file.sync_all()
}

fn main() -> io::Result<()> {
    let fragment = generate_code_fragment("assets/triangle.obj");
    write_code_fragment(&fragment, "triangle_obj_code_gen_test.in")
}
