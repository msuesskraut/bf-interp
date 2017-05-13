use std::str;
use std::cmp;
use std::io::{Read, Result};

#[allow(dead_code)]
pub struct IBuffer {
    buf: Vec<u8>,
    offset: usize,
}

#[allow(dead_code)]
impl IBuffer {
    pub fn from_str(s: &str) -> IBuffer {
        IBuffer {
            buf: s.as_bytes().to_vec(),
            offset: 0,
        }
    }
}

impl Read for IBuffer {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let len = cmp::min(buf.len(), self.buf.len() - self.offset);
        buf[0..len].copy_from_slice(&self.buf[self.offset..(self.offset + len)]);
        self.offset += len;
        Ok(len)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_from_string() {
        let mut in_buf = IBuffer::from_str("Hello world");

        let mut buf = [0u8; 6];

        let res = in_buf.read(&mut buf);
        assert_eq!(6, res.expect("read must be ok"));
        assert_eq!([b'H', b'e', b'l', b'l', b'o', b' '], buf);
    }

    #[test]
    fn read_from_string_overflow() {
        let mut in_buf = IBuffer::from_str("Hello world");

        let mut buf = [0u8; 6];

        let _ = in_buf.read(&mut buf);
        let res = in_buf.read(&mut buf);
        assert_eq!(5, res.expect("read must be ok"));
        assert_eq!([b'w', b'o', b'r', b'l', b'd'], buf[0..5]);
    }

    #[test]
    fn read_from_string_eof() {
        let mut in_buf = IBuffer::from_str("Hello world");

        let mut buf = [0u8; 6];

        let _ = in_buf.read(&mut buf);
        let _ = in_buf.read(&mut buf);
        let res = in_buf.read(&mut buf);
        assert_eq!(0, res.expect("read must be ok"));
    }

}
