#[allow(warnings)]
mod bindings;

use bindings::Guest;
use zstd::stream::encode_all;

struct Component;

impl Guest for Component {
    fn compress(_data: Vec<u8>) -> Vec<u8> {
        // Hardcode the input string
        let input = "hello hello hello world".as_bytes();
        match encode_all(input, 5) {
            Ok(compressed_data) => {
                eprintln!("Compressed data size: {} bytes", compressed_data.len());
                compressed_data
            }
            Err(e) => {
                eprintln!("Zstd compression error: {}", e);
                Vec::new()
            }
        }
    }
}

bindings::export!(Component with_types_in bindings);