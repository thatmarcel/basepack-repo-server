pub trait Bzip2Compress {
    fn compress_via_bzip2(self) -> Option<Vec<u8>>;
}