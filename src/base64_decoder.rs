use std::collections::HashMap;

use crate::common::ALPHABET;

use anyhow::Result;

#[allow(non_camel_case_types)]
#[non_exhaustive]
pub struct Base64Decoder<'message> {
    message: &'message str,
}

impl <'message> Base64Decoder<'message> {

    pub fn new(message: &'message str) -> Self {
        Base64Decoder { message }
    }

    pub fn decode(&self) -> Result<String> {
        // create index lookup by character (for decode)
        let mut idx_lookup_by_char = HashMap::new();
        for idx in 0usize..ALPHABET.len() {
            idx_lookup_by_char.insert(ALPHABET[idx], idx);
        }

        // decode the encoded string
        let enc_chars: Vec<char> = self.message.chars().collect();

        let mut enc_str_bytes: Vec<u8> = Vec::with_capacity(enc_chars.len());
        enc_chars.into_iter().for_each(|c| {
            let key = &c;
            if *key == '=' {
                enc_str_bytes.push(255u8);
            } else {
                let val = idx_lookup_by_char[key];
                enc_str_bytes.push(val as u8);
            }
        });

        let message_size = enc_str_bytes.len();
        let mut enc_start_idx = 0usize;
        let mut res_bytes: Vec<u8> = Vec::new();

        loop {
            let enc_end_idx = std::cmp::min(enc_start_idx+4, message_size);
            let src = &enc_str_bytes[enc_start_idx..enc_end_idx];

            let mut sextets = vec![0; 4usize];
            for i in 0..src.len() {
                sextets[i] = src[i];
            }

            let mut bit_offset = 0;
            let mut byte_offset = 0;
            let mut octets_found = 0;
            let mut padding_found = false;

            loop {
                let mut res = 0u8;

                if octets_found == 3 {
                    break;
                }

                match bit_offset {
                    0 => {
                        res = (sextets[byte_offset] & 0b0011_1111) << 2;
                        byte_offset += 1;
                        if sextets[byte_offset] != 255 {
                            res |= (sextets[byte_offset] & 0b0011_0000) >> 4;
                            bit_offset = 4;
                        } else {
                            bit_offset = 0;
                            padding_found = true;
                        }
                    },
                    4 => {
                        res = (sextets[byte_offset] & 0b0000_1111) << 4;
                        byte_offset += 1;
                        if sextets[byte_offset] != 255 {
                            res |= (sextets[byte_offset] & 0b0011_1100) >> 2;
                            bit_offset = 6;
                        } else {
                            bit_offset = 0;
                            padding_found = true;
                        }
                    },
                    6 => {
                        res = (sextets[byte_offset] & 0b0000_0011) << 6;
                        byte_offset += 1;
                        if sextets[byte_offset] != 255 {
                            res |= sextets[byte_offset] & 0b0011_1111;
                            bit_offset = 0;
                        } else {
                            padding_found = true;
                        }
                    }
                    _ => {},
                }

                if !padding_found {
                    res_bytes.push(res);
                    octets_found += 1;
                }
            
                if byte_offset >= 4 || padding_found {
                    break;
                }
            }

            enc_start_idx += 4;
            if enc_start_idx >= message_size {
                break;
            }
        }

        Ok(String::from_utf8(res_bytes)?)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_single_padding() {
        let text = "bGlnaHQgd29yay4=".to_string();
        
        let enc = Base64Decoder::new(&text);
        let encoded = enc.decode().unwrap();

        assert_eq!(encoded, "light work.".to_string());
    }

    #[test]
    fn it_works_double_padding() {
        let text = "bGlnaHQgd29yaw==".to_string();
        
        let enc = Base64Decoder::new(&text);
        let encoded = enc.decode().unwrap();

        assert_eq!(encoded, "light work".to_string());
    }

    #[test]
    fn it_works_no_padding() {
        let text = "bGlnaHQgd29y".to_string();
        
        let enc = Base64Decoder::new(&text);
        let encoded = enc.decode().unwrap();

        assert_eq!(encoded, "light wor".to_string());
    }

}
