//! Tests for `IBuffer`.

use buffer::IBuffer;
use std::io::Read;

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
