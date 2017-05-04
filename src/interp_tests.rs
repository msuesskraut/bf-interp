use super::*;

use std::string::String;
use std::cmp;

struct IBuffer {
    buf : Vec<u8>,
    offset : usize,
}

impl IBuffer {
    fn new(s : String) -> IBuffer {
        IBuffer {
            buf : s.as_bytes().to_vec(),
            offset : 0
        }
    }
}

impl Read for IBuffer {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let len = cmp::min(buf.len(), self.buf.len() - self.offset);
        buf.copy_from_slice(&self.buf[self.offset..len]);
        self.offset += len;
        Ok(len)
    }
}

#[test]
fn read_from_string() {
    let mut in_buf = IBuffer::new("Hello world".to_string());

    let mut buf = [0u8; 6];

    let res1 = in_buf.read(&mut buf); 
    assert!(match res1 { Ok(6) => true, _ => false });
    assert_eq!(['H' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8, ' ' as u8],
        buf);
}

struct OBuffer {
    buf : String
}

impl OBuffer {
    fn new() -> OBuffer {
        OBuffer {
            buf : String::new()
        }
    }

    fn to_string(self) -> String {
        self.buf
    }
}

//impl Write for OBuffer {
//
//}

//#[test]
//fn output_zero() {
//    let mut input = "".to_string();
//    let mut output = String::new();
//    Program{instructions: vec![Output]}.interp(&mut input, &mut output);
//    assert_eq!("\0", output);
//}