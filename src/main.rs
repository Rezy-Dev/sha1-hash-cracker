use sha1::Digest;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();


    if args.len() != 3 {
        println!("Usage:");
        println!("./sha1-hash-cracker /usr/share/wordlists/rockyou.txt [sha1-hash-here]");
        return Ok(());
    }

    let user_input_hash = args[2].trim();
    if user_input_hash.len() != SHA1_HEX_STRING_LENGTH {
        return Err("The hash you provided is invalid".into());
    }

    let wordlist = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist);

    for line in reader.lines() {
        let line = line?;
        let common_password = line.trim();

        if user_input_hash == &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
            println!("Valid Password Found: {}", &common_password);
            return Ok(());
        }
    }

    Ok(())
}
