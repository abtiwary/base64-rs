use std::env;

use base64_rs::base64_encoder::Base64Encoder;
use base64_rs::base64_decoder::Base64Decoder;

use anyhow::{anyhow, Result};


fn main() -> Result<()> {
    let cl_args: Vec<String> = env::args().collect();
    if cl_args.len() != 3 {
        panic!("unsupported arguments");
    }

    let mode = &cl_args[1];
    let message = &cl_args[2];

    match mode.as_str() {
        "e" => {
            let enc = Base64Encoder::new(message.as_bytes());
            let encoded = enc.encode()?;
            println!("{:?}", encoded);
        },
        "d" => {
            let dec = Base64Decoder::new(message);
            let decoded = dec.decode()?;
            println!("{:?}", decoded);
        },
        _ => {
            return Err(anyhow!("unsupported mode!"));
        },
    }

    Ok(())
}
