extern crate include_dir;

use include_dir::include_dir;

fn main() {
    include_dir!("$CARGO_MANIFEST_DIR/config");
    include_dir!("$CARGO_MANIFEST_DIR/data");
}
