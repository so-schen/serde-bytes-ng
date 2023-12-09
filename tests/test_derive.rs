use serde::{Deserialize, Serialize};
use serde_test::{assert_tokens, Token};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Test<'a> {
    #[serde(with = "serde_bytes_ng")]
    array: [u8; 314],
    #[serde(with = "serde_bytes_ng", borrow)]
    array_ref: &'a [u8; 314],
    #[serde(with = "serde_bytes_ng")]
    opt_array: Option<[u8; 314]>,
}

#[test]
fn test() {
    let test = Test {
        array: [0; 314],
        array_ref: &[1; 314],
        opt_array: Some([2; 314]),
    };

    assert_tokens(
        &test,
        &[
            Token::Struct {
                name: "Test",
                len: 3,
            },
            Token::Str("array"),
            Token::Bytes(&[0; 314]),
            Token::Str("array_ref"),
            Token::BorrowedBytes(&[1; 314]),
            Token::Str("opt_array"),
            Token::Some,
            Token::Bytes(&[2; 314]),
            Token::StructEnd,
        ],
    );
}
