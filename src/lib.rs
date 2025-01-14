pub mod pak;

use pak::NetEasePak;
pub fn load_pak(left: u64, right: u64) -> u64 {
    left + right
}
