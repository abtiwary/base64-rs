use crate::common::ALPHABET;
use crate::common::PADDING;

use anyhow::Result;

#[allow(non_camel_case_types)]
#[non_exhaustive]
pub struct Base64Encoder<'message> {
    message: &'message[u8],
}

impl <'message> Base64Encoder<'message> {

    pub fn new(message: &'message[u8]) -> Self {
        Base64Encoder { message }
    }

    pub fn encode(&self) -> Result<String> {
        let message_size = self.message.len();

        let mut src_start_idx = 0usize;

        let mut encoded_string = String::new();

        loop {
            let end_idx = std::cmp::min(src_start_idx+3, message_size);
            let src = &self.message[src_start_idx..end_idx];

            let mut octets = vec![255; 3usize];
            for i in 0..src.len() {
                octets[i] = src[i];
            }

            let mut byte_offset = 0;
            let mut bit_offset = 0;

            loop {
                if octets[byte_offset] == 255 {
                    encoded_string.push(PADDING);
                    byte_offset += 1;
                    if byte_offset >= 3 {
                        break;
                    }
                
                } else {
                    let mut res = 0;
                    match bit_offset {
                        0 => {
                            res = (octets[byte_offset] & 0b1111_1100) >> 2;
                            bit_offset = 6;
                        },
                        6 => {
                            res = (octets[byte_offset] & 0b0000_0011) << 4;
                            byte_offset += 1;
                            if octets[byte_offset] != 255 {
                                res |= (octets[byte_offset] & 0b1111_0000) >> 4;
                                bit_offset = 4;
                            } else {
                                bit_offset = 0;
                            }
                        },
                        4 => {
                            res = (octets[byte_offset] & 0b0000_1111) << 2;
                            byte_offset += 1;
                            if octets[byte_offset] != 255 {
                                res |= (octets[byte_offset] & 0b1100_0000) >> 6;
                                bit_offset = 2;
                            } else {
                                bit_offset = 0;
                            }
                        },
                        2 => {
                            res = octets[byte_offset] & 0b0011_1111;
                            byte_offset += 1; 
                            bit_offset = 0;
                        },
                        _ => {},
                    }
                    encoded_string.push(ALPHABET[res as usize]);

                    if byte_offset >= 3 {
                        break;
                    }
                }
            }

            src_start_idx += 3;
            if src_start_idx >= message_size {
                break;
            }
        }

        Ok(encoded_string)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_single_padding() {
        let text = "light work.".to_string();
        
        let enc = Base64Encoder::new(&text.as_bytes());
        let encoded = enc.encode().unwrap();

        assert_eq!(encoded, "bGlnaHQgd29yay4=".to_string());
    }

    #[test]
    fn it_works_double_padding() {
        let text = "light work".to_string();
        
        let enc = Base64Encoder::new(&text.as_bytes());
        let encoded = enc.encode().unwrap();

        assert_eq!(encoded, "bGlnaHQgd29yaw==".to_string());
    }

    #[test]
    fn it_works_no_padding() {
        let text = "light wor".to_string();
        
        let enc = Base64Encoder::new(&text.as_bytes());
        let encoded = enc.encode().unwrap();

        assert_eq!(encoded, "bGlnaHQgd29y".to_string());
    }
}
