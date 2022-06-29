use std::{fs::File, io::Read, path::Path};

use ed25519_dalek::Keypair;
use rand::rngs::OsRng;

pub(crate) fn generate_keypair() -> Keypair {
    let mut csprng = OsRng {};
    Keypair::generate(&mut csprng)
}

pub(crate) fn keypair_from_bytes(bytes: &[u8]) -> Keypair {
    match Keypair::from_bytes(bytes) {
        Ok(keypair) => keypair,
        Err(why) => panic!("couldn't get keypair from bytes: {}", why),
    }
}

pub(crate) fn get_bytes_from_file(path: &str) -> Vec<u8> {
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut bytes = Vec::new();

    if let Err(why) = file.read_to_end(&mut bytes) {
        panic!("couldn't read {}: {}", display, why)
    };

    bytes
}

pub(crate) fn get_keypair_from_file(path: &str) -> Keypair {
    let keypair_bytes = get_bytes_from_file(path);

    keypair_from_bytes(&keypair_bytes)
}
