#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    fn decompress(data: Vec<u8>) -> Vec<u8> {
        match decode_all(&data) {
            Ok(decompressed_data) => decompressed_data,
            Err(e) => {
                eprintln!("Zstd decompression error: {}", e);
                Vec::new() // Or handle the error more gracefully
            }
        }
    }
}

bindings::export!(Component with_types_in bindings);
