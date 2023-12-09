use serde::{Deserialize, Serialize};
use serde_test::{assert_de_tokens, assert_ser_tokens, assert_tokens, Token};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(transparent)]
struct Bytes<const N: usize>(#[serde(with = "serde_bytes_ng")] [u8; N]);

#[test]
fn test_array() {
    let empty = Bytes([]);
    assert_tokens(&empty, &[Token::BorrowedBytes(b"")]);
    assert_tokens(&empty, &[Token::Bytes(b"")]);
    assert_tokens(&empty, &[Token::ByteBuf(b"")]);
    assert_de_tokens(&empty, &[Token::BorrowedStr("")]);

    let buf = [65, 66, 67];
    let bytes = Bytes(buf);
    assert_tokens(&bytes, &[Token::BorrowedBytes(b"ABC")]);
    assert_tokens(&bytes, &[Token::Bytes(b"ABC")]);
    assert_tokens(&bytes, &[Token::ByteBuf(b"ABC")]);
    assert_de_tokens(&bytes, &[Token::BorrowedStr("ABC")]);
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(transparent)]
struct BytesRef<'a, const N: usize>(#[serde(with = "serde_bytes_ng", borrow)] &'a [u8; N]);

#[test]
fn test_array_ref() {
    let empty = BytesRef(&[]);
    assert_tokens(&empty, &[Token::BorrowedBytes(b"")]);
    assert_ser_tokens(&empty, &[Token::Bytes(b"")]);
    assert_ser_tokens(&empty, &[Token::ByteBuf(b"")]);
    assert_de_tokens(&empty, &[Token::BorrowedStr("")]);

    let buf = [65, 66, 67];
    let bytes = BytesRef(&buf);
    assert_tokens(&bytes, &[Token::BorrowedBytes(b"ABC")]);
    assert_ser_tokens(&bytes, &[Token::Bytes(b"ABC")]);
    assert_ser_tokens(&bytes, &[Token::ByteBuf(b"ABC")]);
    assert_de_tokens(&bytes, &[Token::BorrowedStr("ABC")]);
}
