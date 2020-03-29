use rayon::prelude::*;
use sha2::{Digest, Sha256};
use core::fmt;
use rand::prelude::*;

struct Color(u8, u8, u8);

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}{:x}{:x}", self.0, self.1, self.2)
    }
}

fn get_proof() -> String {
    loop {
        let proof_suffix = (0..30)
            .map(|_| rand::random::<u8>())
            .map(|x| x % 26 + b'a')
            .collect();
        let proof_suffix = String::from_utf8(proof_suffix).unwrap_or_else(|_| "foo".to_owned());

        let hash = Sha256::new()
            .chain("h25")
            .chain("totorigolo-")
            .chain(&proof_suffix)
            .result();

        if format!("{:x}", hash).starts_with("00000") {
            let proof = format!("totorigolo-{}", proof_suffix);
            println!("Found proof: {} -> {:x}", proof, hash);
            return proof;
        }
    }
}

fn paint(pixels: &[(u64, u64, Color)]) {
    pixels.into_par_iter().for_each(|(x, y, color)| {
        let proof = get_proof();

        let req = format!(
            "http://137.74.47.86/setpixel?x={}&y={}&color={}&proof={}",
            x, y, color, proof
        );
        let resp = reqwest::blocking::get(&req);

        match resp {
            Ok(_) => println!("Ok"),
            Err(e) => println!("Failed to send: {:#?}", e),
        }
    });
}

fn paint_pink() {
    let mut coords = Vec::with_capacity(100*100);
    for x in 0..99 {
        for y in 0..99 {
            if x + y % 2 == 0 || x % 2 == 0 {
                let color = Color(61, 66, 170);
                coords.push((99-x, 99-y, color));
            }
        }
    }
    let mut rng = rand::thread_rng();
    coords.shuffle(&mut rng);
    paint(&coords);
}

fn main() {
    loop {
        paint_pink();
    }
}
