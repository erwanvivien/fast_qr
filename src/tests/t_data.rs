#[test]
fn alpha_encoding_hello_world() {
    let version = 1;
    const QUALITY: crate::vecl::ECL = crate::vecl::ECL::Q;
    let string_to_encode: String = String::from("HELLO WORLD");

    let vec = crate::encoding::alphanum::encode(&string_to_encode, version, QUALITY);
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

#[test]
fn byte_encoding_hello_world_2() {
    let version = 2;
    const QUALITY: crate::vecl::ECL = crate::vecl::ECL::Q;

    let string_to_encode: String = String::from("Hello, world!");

    let vec = crate::encoding::byte::encode(&string_to_encode, version, QUALITY);
    let tmp1 = vec.unwrap().to_vec();

    let mut bitstorage = crate::bitstorage::BitStorage::new();

    // byte mode
    bitstorage.push_binstr("0100");
    // Char count
    bitstorage.push_last(13, 8);
    // H: 01001000
    // e: 01100101
    // l: 01101100
    // l: 01101100
    // o: 01101111
    // ,: 00101100
    //  : 00100000
    // w: 01110111
    // o: 01101111
    // r: 01110010
    // l: 01101100
    // d: 01100100
    // !: 00100001
    bitstorage.push_binstr("01001000011001010110110001101100011011110010110000100000011101110110111101110010011011000110010000100001");
    // Terminator String
    bitstorage.push_last(0, 4);
    // Padding to 8
    bitstorage.push_last(0, 0);

    // padding to required bits
    for _ in 0..7 / 2 {
        bitstorage.push_binstr("11101100");
        bitstorage.push_binstr("00010001");
    }
    bitstorage.push_binstr("11101100");

    assert_eq!(tmp1, bitstorage.to_vec());
}

#[test]
fn byte_encoding_hello_world_15() {
    let version = 15;
    const QUALITY: crate::vecl::ECL = crate::vecl::ECL::Q;

    let string_to_encode: String = String::from("Hello, world!");

    let vec = crate::encoding::byte::encode(&string_to_encode, version, QUALITY);
    let bits = vec.unwrap();
    let tmp1 = bits.to_vec();

    let mut bitstorage = crate::bitstorage::BitStorage::new();

    // byte mode
    bitstorage.push_binstr("0100");
    // Char count
    bitstorage.push_last(13, 16);
    // H: 01001000
    // e: 01100101
    // l: 01101100
    // l: 01101100
    // o: 01101111
    // ,: 00101100
    //  : 00100000
    // w: 01110111
    // o: 01101111
    // r: 01110010
    // l: 01101100
    // d: 01100100
    // !: 00100001
    bitstorage.push_binstr("01001000011001010110110001101100011011110010110000100000011101110110111101110010011011000110010000100001");
    // Terminator String
    bitstorage.push_last(0, 4);
    // Padding to 8
    bitstorage.push_last(0, 0);

    let padding = (2360 - bitstorage.len()) / 8;
    // padding to required bits
    for _ in 0..padding / 2 {
        bitstorage.push_binstr("11101100");
        bitstorage.push_binstr("00010001");
    }
    if padding & 1 != 0 {
        bitstorage.push_binstr("11101100");
    }

    assert_eq!(bits.len(), bitstorage.len());
    assert_eq!(tmp1, bitstorage.to_vec());
}
