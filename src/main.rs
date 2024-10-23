
mod sss;
mod utils;
use sss::{Charset, SecretSharing};
use std::process::exit;

fn main() {
    let secret = "supersecretmessage";
    let mut secret_sharing = SecretSharing::new(3, 8, Charset::Alphanumeric);

    match secret_sharing.generate_shares(secret) {
        Ok(shares) => {
            println!("Shares:");
            for (i, share) in shares.iter().enumerate() {
                println!("Share {}: {}", i + 1, share);
            }

            let reconstruction_shares = &shares[0..3];
            match secret_sharing.reconstruct_secret(reconstruction_shares) {
                Ok(reconstructed_secret) => {
                    println!("Reconstructed secret: {}", reconstructed_secret);
                }
                Err(e) => {
                    eprintln!("Error reconstructing secret: {:?}", e);
                    exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error generating shares: {:?}", e);
            exit(1);
        }
    }
}