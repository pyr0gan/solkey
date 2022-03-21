#![allow(unused_imports)]

use colored::*;
use ezcli::{flag, option};
use failure::Fallible;
use lib::utils::StringUtils;
use solana_sdk::{
    signature::Signer,
    signer::keypair::{write_keypair_file, Keypair},
};
use std::io::prelude::*;
use std::{fs::create_dir, fs::remove_dir_all, fs::File, io::stdout, path::Path, process::exit};

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1);
        }
    }
}

fn run() -> Fallible<()> {
    flag!(pubkey);
    flag!(keypair);

    if Path::new("keys").is_dir() {
        assert_eq!(Path::new("./keys/").is_dir(), true);
    } else {
        create_dir("keys").unwrap();
    }
    
        if pubkey {
            let random_keypair = Keypair::new();
            println!(
                "\nRandom Pubkey: {}\n",
                random_keypair.pubkey().to_string().green()
            );    
        };

        if keypair {
            let random_keypair = Keypair::new();
            println!(
                "\n        Public Key: {} \nBase58 Private Key: [{}]",
                random_keypair.pubkey().to_string().green(),
                random_keypair.to_base58_string().black().on_black()
            );
            let dir = String::from("./keys/");
            let pkdir = String::from("./keys/");
            let ext = String::from("keypair.json");
            let pkext = String::from("pk.0");
            let short_pubkey_1 = [
                random_keypair
                    .pubkey()
                    .to_string()
                    .substring(0, 6)
                    .to_string(),
                random_keypair
                    .pubkey()
                    .to_string()
                    .substring(38, 6)
                    .to_string(),
            ]
            .join("-");
            let short_pubkey_2 = [
                random_keypair
                    .pubkey()
                    .to_string()
                    .substring(0, 6)
                    .to_string(),
                random_keypair
                    .pubkey()
                    .to_string()
                    .substring(38, 6)
                    .to_string(),
            ]
            .join("-");
            let filename = [short_pubkey_1, ext].join("_");
            let filepath = [dir, filename].join("");
            let privatekey_filename = [short_pubkey_2, pkext].join("_");
            let privatekey_filepath = [pkdir, privatekey_filename].join("");
            let privatekey_filepath_clone = privatekey_filepath.clone();
            let mut pkfile = File::create(privatekey_filepath)?;
            write_keypair_file(&random_keypair, Path::new(&filepath)).unwrap_or_default();
            println!("Saved keypair {}", filepath.yellow());
            pkfile
                .write_all(random_keypair.to_base58_string().as_bytes())
                .unwrap_or_default();
            println!("Saved private key {}\n", privatekey_filepath_clone.yellow());
        };
        Ok(())
    }
