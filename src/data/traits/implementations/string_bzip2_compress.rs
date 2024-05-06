use std::io::Read;
use bzip2::Compression;
use bzip2::read::BzEncoder;
use crate::data::traits::bzip2_compress::Bzip2Compress;

impl Bzip2Compress for String {
    fn compress_via_bzip2(self) -> Option<Vec<u8>> {
        (&*self).compress_via_bzip2()
    }
}

impl Bzip2Compress for &str {
    fn compress_via_bzip2(self) -> Option<Vec<u8>> {
        let bytes = self.as_bytes();

        let mut compressor = BzEncoder::new(bytes, Compression::best());

        let mut result_buffer: Vec<u8> = Vec::new();

        match compressor.read_to_end(&mut result_buffer) {
            Ok(_) => Some(result_buffer),
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }
}