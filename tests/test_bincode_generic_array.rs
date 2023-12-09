use serde_derive::{Deserialize, Serialize};

#[test]
fn test_bincode_serde() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Test<'a> {
        start: usize,
        #[serde(with = "serde_bytes_ng")]
        array: [u8; 314],

        #[serde(with = "serde_bytes_ng", borrow)]
        array_ref: &'a [u8; 314],
        end: usize,
    }
    let test_value = Test {
        start: 0xdeadbeef,
        array: [0xaa; 314],
        array_ref: &[0xff; 314],
        end: 0xdeadbeef,
    };
    let serialized = bincode::serialize(&test_value).unwrap();
    let deserialized: Test = bincode::deserialize(&serialized).unwrap();
    assert_eq!(test_value, deserialized);
}
