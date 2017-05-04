use std::string::String;
use std::str;
use std::cmp;
use std::io::{Read, Write, Result, Error, ErrorKind};

#[allow(dead_code)]
pub struct IBuffer {
    buf : Vec<u8>,
    offset : usize,
}

#[allow(dead_code)]
impl IBuffer {
    pub fn new(s : String) -> IBuffer {
        IBuffer {
            buf : s.as_bytes().to_vec(),
            offset : 0
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

#[allow(dead_code)]
pub struct OBuffer {
    buf : String
}

#[allow(dead_code)]
impl OBuffer {
    pub fn new() -> OBuffer {
        OBuffer {
            buf : String::new()
        }
    }

    pub fn to_string(self) -> String {
        self.buf
    }
}

impl Write for OBuffer {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match str::from_utf8(buf) {
            Ok(s) => {
                self.buf += s;
                Ok(buf.len())
            },
            _ => Err(Error::from(ErrorKind::Other)),
        }
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_from_string() {
        let mut in_buf = IBuffer::new("Hello world".to_string());

        let mut buf = [0u8; 6];

        let res1 = in_buf.read(&mut buf); 
        assert!(match res1 { Ok(6) => true, _ => false });
        assert_eq!(['H' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8, ' ' as u8],
            buf);
    }

    #[test]
    fn read_from_string_overflow() {
        let mut in_buf = IBuffer::new("Hello world".to_string());

        let mut buf = [0u8; 6];

        let _ = in_buf.read(&mut buf); 
        let res = in_buf.read(&mut buf); 
        assert!(match res { Ok(5) => true, _ => false });
        assert_eq!(['w' as u8, 'o' as u8, 'r' as u8, 'l' as u8, 'd' as u8],
            buf[0..5]);
    }

    #[test]
    fn read_from_string_eof() {
        let mut in_buf = IBuffer::new("Hello world".to_string());

        let mut buf = [0u8; 6];

        let _ = in_buf.read(&mut buf); 
        let _ = in_buf.read(&mut buf); 
        let res = in_buf.read(&mut buf); 
        assert!(match res { Ok(0) => true, _ => false });
    }

    #[test]
    fn write_once_to_buffer() {
        let mut out_buf = OBuffer::new();

        let res = out_buf.write(&['H' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8]);

        assert!(match res { Ok(5) => true, _ => false });
        assert_eq!("Hello".to_string(), out_buf.to_string());
    }

    #[test]
    fn write_twice_to_buffer() {
        let mut out_buf = OBuffer::new();

        let _ = out_buf.write(&['H' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8]);
        let res = out_buf.write(&[' ' as u8, 'w' as u8, 'o' as u8, 'r' as u8, 'l' as u8, 'd' as u8]);

        assert!(match res { Ok(6) => true, _ => false });
        assert_eq!("Hello world".to_string(), out_buf.to_string());
    }

}