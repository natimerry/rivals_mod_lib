use aes::cipher::KeyInit;
use aes::Aes256;
use base64::engine::general_purpose;
use base64::Engine;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use repak_rivals::PakBuilder;

pub struct ModPak {
    path: String,
    description: Option<String>,
    pak_location: PathBuf,
    files: Vec<String>,
}

pub struct NetEasePak {
    pub path: PathBuf,
    pub aes_str: String,
    pub files: Vec<String>,
}

impl NetEasePak {
    fn get_aes_obj(&self) -> Aes256 {
        let try_parse = |mut bytes: Vec<_>| {
            bytes.chunks_mut(4).for_each(|c| c.reverse());
            aes::Aes256::new_from_slice(&bytes).ok()
        };
        hex::decode(self.aes_str.strip_prefix("0x").unwrap_or(&self.aes_str))
            .ok()
            .and_then(try_parse)
            .or_else(|| {
                general_purpose::STANDARD_NO_PAD
                    .decode(self.aes_str.trim_end_matches('='))
                    .ok()
                    .and_then(try_parse)
            })
            .expect("Unable to parse AES key")
    }
}

pub trait ReadPak {
    fn read_pak(&self);
    fn encode_pak(&self);
}

impl ReadPak for NetEasePak {
    fn read_pak(&self) {
        let aes = self.get_aes_obj();
        let rdr = repak_rivals::PakBuilder::new()
            .key(aes)
            .reader(&mut BufReader::new(File::open(&self.path).unwrap()))
            .expect("Unable to open file");

        let mount_point = PathBuf::from(rdr.mount_point());

        let full_paths = rdr.files().into_iter().for_each(|path| {
            println!("{}", path);
        });
    }

    fn encode_pak(&self) {
        let aes = self.get_aes_obj();
        let pak = PakBuilder::new().key(aes).encrypter(&mut BufReader::new(File::open(&self.path).unwrap())).expect("Unable to build pakker");

    }
}
