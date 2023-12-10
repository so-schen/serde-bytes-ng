use serde_derive::{Deserialize, Serialize};
use serde_test::{assert_de_tokens, assert_ser_tokens, assert_tokens, Token};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(transparent)]
struct BytesArrayBorrow<'a, const N: usize>(#[serde(with = "serde_bytes_ng", borrow)] &'a [u8; N]);

#[test]
fn test_array_borrow() {
    let empty = BytesArrayBorrow(&[]);
    assert_tokens(&empty, &[Token::BorrowedBytes(b"")]);
    assert_ser_tokens(&empty, &[Token::Bytes(b"")]);
    assert_ser_tokens(&empty, &[Token::ByteBuf(b"")]);
    assert_de_tokens(&empty, &[Token::BorrowedStr("")]);

    let buf = [65, 66, 67];
    let bytes = BytesArrayBorrow(&buf);
    assert_tokens(&bytes, &[Token::BorrowedBytes(b"ABC")]);
    assert_ser_tokens(&bytes, &[Token::Bytes(b"ABC")]);
    assert_ser_tokens(&bytes, &[Token::ByteBuf(b"ABC")]);
    assert_de_tokens(&bytes, &[Token::BorrowedStr("ABC")]);
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(transparent)]
struct BytesArray<const N: usize>(#[serde(with = "serde_bytes_ng")] [u8; N]);

#[test]
fn test_array() {
    let empty = BytesArray([]);
    assert_tokens(&empty, &[Token::BorrowedBytes(b"")]);
    assert_tokens(&empty, &[Token::Bytes(b"")]);
    assert_tokens(&empty, &[Token::ByteBuf(b"")]);
    assert_de_tokens(&empty, &[Token::BorrowedStr("")]);
    assert_de_tokens(&empty, &[Token::Str("")]);
    assert_de_tokens(&empty, &[Token::String("")]);

    // Note: Opt out support of `serde::de::SeqAccess` for arrays.
    // assert_de_tokens(&empty, &[Token::Seq { len: None }, Token::SeqEnd]);
    // assert_de_tokens(&empty, &[Token::Seq { len: Some(0) }, Token::SeqEnd]);

    let buf = [65, 66, 67];
    let bytes = BytesArray(buf);
    assert_tokens(&bytes, &[Token::BorrowedBytes(b"ABC")]);
    assert_tokens(&bytes, &[Token::Bytes(b"ABC")]);
    assert_tokens(&bytes, &[Token::ByteBuf(b"ABC")]);
    assert_de_tokens(&bytes, &[Token::BorrowedStr("ABC")]);
    assert_de_tokens(&bytes, &[Token::Str("ABC")]);
    assert_de_tokens(&bytes, &[Token::String("ABC")]);

    // Note: Opt out support of `serde::de::SeqAccess` for arrays.
    // assert_de_tokens(
    //     &buf,
    //     &[
    //         Token::Seq { len: None },
    //         Token::U8(65),
    //         Token::U8(66),
    //         Token::U8(67),
    //         Token::SeqEnd,
    //     ],
    // );
    // assert_de_tokens(
    //     &buf,
    //     &[
    //         Token::Seq { len: Some(3) },
    //         Token::U8(65),
    //         Token::U8(66),
    //         Token::U8(67),
    //         Token::SeqEnd,
    //     ],
    // );
}
