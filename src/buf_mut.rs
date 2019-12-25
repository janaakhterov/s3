pub trait BufMut {
    fn add_header<T: AsRef<str>>(&mut self, header: &str, value: T);
}

impl BufMut for Vec<u8> {
    #[inline(always)]
    fn add_header<T: AsRef<str>>(&mut self, header: &str, value: T) {
        self.extend_from_slice(header.as_bytes());
        self.push(b':');
        self.extend_from_slice(value.as_ref().as_bytes());
        self.push(b'\n');
    }
}
