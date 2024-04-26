use sha3::{Digest, Sha3_256};

struct Bin {
    file_size: u64,
    hash: String,
    file_data: Vec<u8>,
}
fn hash_file(file_data: &[u8]) -> String {
    let mut hasher: sha3::digest::core_api::CoreWrapper<sha3::Sha3_256Core> = Sha3_256::new();
    hasher.update(file_data);
    let result: sha3::digest::generic_array::GenericArray<
                    u8, 
                    sha3::digest::typenum::UInt<
                        sha3::digest::typenum::UInt<
                            sha3::digest::typenum::UInt<
                                sha3::digest::typenum::UInt<
                                    sha3::digest::typenum::UInt<
                                        sha3::digest::typenum::UInt<
                                            sha3::digest::typenum::UTerm,sha3::digest::consts::B1>, 
                                    sha3::digest::consts::B0>, 
                                sha3::digest::consts::B0>,
                            sha3::digest::consts::B0>,
                        sha3::digest::consts::B0>,
                    sha3::digest::consts::B0>> = hasher.finalize();
    format!("{:x}", result)
}
fn _hash_file(file_data: &[u8]) -> String {
    use sha3::digest::generic_array::GenericArray;
    use sha3::digest::typenum::UInt;
    use sha3::digest::typenum::UTerm;
    use sha3::digest::consts::B0;
    use sha3::digest::consts::B1;
    let mut hasher = Sha3_256::new();
    hasher.update(file_data);
    let result: 
        GenericArray<
            u8,
             UInt<
                UInt<
                    UInt<
                        UInt<
                            UInt<
                                UInt<
                                    UTerm,
                                    B1
                                >, 
                                B0
                            >, B0>, 
                        B0>, 
                    B0>, 
            B0>
        > = hasher.finalize();
    format!("{:x}", result)
}

fn read_file(file_path: &str) -> Result<Bin, std::io::Error> {
    let file_data = std::fs::read(file_path)?;
    let file_size = file_data.len() as u64;
    let hash = hash_file(&file_data);
    Ok(Bin {
        file_size,
        hash,
        file_data,
    })
}
fn bin_struct_to_bytes(bin: &Bin) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&bin.file_size.to_le_bytes());
    bytes.extend_from_slice(bin.hash.as_bytes());
    bytes.extend_from_slice(&bin.file_data);
    bytes
}
fn bytes_to_bin_struct(bytes: &[u8]) -> Bin {
    let file_size = u64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]);
    let hash = std::str::from_utf8(&bytes[8..40]).unwrap().to_string();
    let file_data = bytes[40..].to_vec();
    Bin {
        file_size,
        hash,
        file_data,
    }
}
fn write_file(file_path: &str, bin: &Bin) -> Result<(), std::io::Error> {
    std::fs::write(file_path, bin_struct_to_bytes(bin))?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    
}
