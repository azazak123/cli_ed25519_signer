use std::{
    fs::File,
    io::{BufReader, Read, Write},
    path::Path,
};

use clap::Subcommand;
use ed25519_dalek::{Keypair, Signature, Signer};
use hex::FromHex;

use crate::utils::{
    generate_keypair, get_bytes_from_file, get_keypair_from_file, keypair_from_bytes,
};

#[derive(Subcommand)]
pub enum Commands {
    /// Generate keypair
    Generate {
        /// Path to generated keypair file
        #[clap(short, long, value_parser, default_value = "./keypair")]
        path: String,
    },

    /// Print public key
    Pubkey {
        /// Path to keypair file
        #[clap(short, long, value_parser, default_value = "./keypair")]
        path: String,
    },

    /// Sign file
    Sign {
        /// Path to keypair file
        #[clap(short, long, value_parser, default_value = "./keypair")]
        path: String,
        /// Path to data file
        #[clap(short, long, value_parser)]
        data_path: String,
    },

    ///  Verify file
    Verify {
        /// Path to keypair file
        #[clap(short, long, value_parser, default_value = "./keypair")]
        path: String,
        /// Path to data file
        #[clap(short, long, value_parser)]
        data_path: String,
        /// Signature
        #[clap(short, long, value_parser)]
        signature: String,
    },
}

impl Commands {
    pub fn do_action(&self) {
        match self {
            Commands::Generate { path } => generate_keypair_file(path),
            Commands::Pubkey { path } => print_pubkey(path),
            Commands::Sign { path, data_path } => sign_data_from_file(path, data_path),
            Commands::Verify {
                path,
                signature,
                data_path,
            } => verify_data(path, data_path, signature),
        }
    }
}

fn generate_keypair_file(path: &str) {
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let keypair = generate_keypair();

    if let Err(why) = file.write_all(&keypair.to_bytes()) {
        panic!("Couldn't write to {}: {}", display, why)
    }
}

fn print_pubkey(path: &str) {
    let keypair = get_keypair_from_file(path);

    println!("Public key: {:?}", keypair.public.as_bytes());
}

fn sign_data_from_file(path: &str, data_path: &str) {
    let keypair = get_keypair_from_file(path);

    let data = get_bytes_from_file(data_path);

    let signature = keypair.sign(&data);

    println!("Signature:{}", signature);
}

fn verify_data(path: &str, data_path: &str, signature_str: &str) {
    let keypair = get_keypair_from_file(path);

    let data = get_bytes_from_file(data_path);

    let signature_bytes = match Vec::from_hex(signature_str) {
        Err(why) => panic!("Couldn't read signature: {}", why),
        Ok(signature_bytes) => signature_bytes,
    };

    let signature = match Signature::from_bytes(&signature_bytes) {
        Err(why) => panic!("Couldn't parse signature: {}", why),
        Ok(signature) => signature,
    };

    match keypair.verify(&data, &signature) {
        Ok(_) => println!("Verification is successful !"),
        Err(_) => println!("Verification is not successful !"),
    };
}
