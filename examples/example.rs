extern crate mini_obj;

use mini_obj as obj;

const SAMPLE_DATA: &str = "assets/triangle.obj";


fn main() {
    let object_set = obj::load_file(SAMPLE_DATA);

    assert!(object_set.is_ok());
}
