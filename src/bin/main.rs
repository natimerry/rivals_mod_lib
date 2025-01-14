use std::path::{Path, PathBuf};
use rivals_mod_lib::pak::{NetEasePak, ReadPak};

fn main() {
    let x = NetEasePak{
        path: PathBuf::from(Path::new("main.pak")),
        aes_str: "0x0C263D8C22DCB085894899C3A3796383E9BF9DE0CBFB08C9BF2DEF2E84F29D74".to_string(),
        files: vec![],
    };
    x.read_pak();
}