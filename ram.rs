use crate::errors;

pub const DIGEST_SIZE: usize = 4;

// ABI
//

pub struct MetaMagic {
    car: [u8; 4],
}
