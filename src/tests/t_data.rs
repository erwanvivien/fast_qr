#[test]
fn data_encoding_hello_world() {
    let version = 1;
    const QUALITY: crate::vecl::ECL = crate::vecl::ECL::Q;
    let string_to_encode: String = String::from("HELLO WORLD");

    let vec = crate::encoding::alphanum::encode_alphanum(string_to_encode, version, QUALITY);
    let tmp1 = vec.unwrap().to_vec();

    let mut bitstorage = crate::bitstorage::BitStorage::new();
    bitstorage.push_binstr("00100000010110110000101101111000110100010111001011011100010011010100001101000000111011000001000111101100");

    assert_eq!(
        tmp1,
        [
            0b00100000, 0b01011011, 0b00001011, 0b01111000, 0b11010001, 0b01110010, 0b11011100,
            0b01001101, 0b01000011, 0b01000000, 0b11101100, 0b00010001, 0b11101100
        ]
    );
    assert_eq!(tmp1, bitstorage.to_vec());
    //00100000,01011011,00001011,01111000,11010001,01110010,11011100,01001101,01000011,01000000,11101100,00010001,11101100
}
