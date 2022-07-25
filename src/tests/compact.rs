use crate::compact::CompactQR;

#[test]
fn push8_lined() {
    let mut expected = [0u8; 64];
    let mut res = CompactQR::with_len(64);

    res.push_u8(0);
    res.push_u8(1);
    res.push_u8(2);

    expected[1] = 1;
    expected[2] = 2;

    assert_eq!(res.len, 8 * 3, "expected 24, got {}", res.len);
    assert_eq!(res.get_data()[..4], expected[..4]);
}

#[test]
fn push_bits_half() {
    let mut expected = [0u8; 64];
    let mut res = CompactQR::with_len(64);

    res.push_bits(0b1111, 4);
    assert_eq!(res.len, 4, "expected 4, got {}", res.len);
    res.push_bits(0, 8);
    assert_eq!(res.len, 12, "expected 12, got {}", res.len);
    res.push_bits(0b1111, 4);
    assert_eq!(res.len, 16, "expected 16, got {}", res.len);

    expected[0] = 0b11110000;
    expected[1] = 0b00001111;

    assert_eq!(res.get_data()[..4], expected[..4]);
}

#[test]
fn push_bits_random() {
    let mut expected = [0u8; 64];
    let mut res = CompactQR::with_len(64);

    res.push_bits(0b1111, 2);
    expected[0] = 0b11000000;
    assert_eq!(res.len, 2, "expected 2, got {}", res.len);
    assert_eq!(res.get_data()[..4], expected[..4]);

    res.push_bits(0, 1);
    expected[0] = 0b11000000;
    assert_eq!(res.len, 3, "expected 3, got {}", res.len);
    assert_eq!(res.get_data()[..4], expected[..4]);

    res.push_bits(5, 3);
    expected[0] = 0b11010100;
    assert_eq!(res.len, 6, "expected 6, got {}", res.len);
    assert_eq!(res.get_data()[..4], expected[..4]);

    res.push_bits(0b1101, 2);
    expected[0] = 0b11010101;
    assert_eq!(res.len, 8, "expected 8, got {}", res.len);
    assert_eq!(res.get_data()[..4], expected[..4]);
}

#[test]
fn push_bits_push8() {
    let mut expected = [0u8; 64];
    let mut res = CompactQR::with_len(64);

    res.push_bits(0b1111, 3);
    expected[0] = 0b11100000;
    assert_eq!(res.get_data()[..4], expected[..4]);

    res.push_u8(0b10011110);
    expected[0] = 0b11110011;
    expected[1] = 0b11000000;
    assert_eq!(res.get_data()[..4], expected[..4]);
}

#[test]
fn push_bits_push8_2() {
    let mut expected = [0u8; 64];
    let mut res = CompactQR::with_len(64);

    res.push_bits(0b1111, 3);
    expected[0] = 0b11100000;
    assert_eq!(res.get_data()[..4], expected[..4]);

    res.push_u8(0b10011110);
    expected[0] = 0b11110011;
    expected[1] = 0b11000000;
    assert_eq!(res.get_data()[..4], expected[..4]);

    res.push_u8(0b10011110);
    expected[0] = 0b11110011;
    expected[1] = 0b11010011;
    expected[2] = 0b11000000;
    assert_eq!(res.get_data()[..4], expected[..4]);
}

#[test]
fn push8_push_bits() {
    let mut expected = [0u8; 64];
    let mut res = CompactQR::with_len(64);

    res.push_u8(0b10011110);
    expected[0] = 0b10011110;
    assert_eq!(res.get_data()[..4], expected[..4]);

    res.push_bits(0b1101110011110, 13);
    expected[0] = 0b10011110;
    expected[1] = 0b11011100;
    expected[2] = 0b11110000;
    assert_eq!(res.get_data()[..4], expected[..4]);
}

#[test]
fn push_slice() {
    let mut expected = [0u8; 64];
    let mut res = CompactQR::with_len(64);

    res.push_u8_slice(&[0b10011110, 0b10011110, 0b10011110]);
    expected[0] = 0b10011110;
    expected[1] = 0b10011110;
    expected[2] = 0b10011110;
    assert_eq!(res.get_data()[..4], expected[..4]);
}

#[test]
fn push_slice_off() {
    let mut expected = [0u8; 64];
    let mut res = CompactQR::with_len(64);

    res.push_bits(0b1111, 3);
    expected[0] = 0b11100000;
    assert_eq!(res.get_data()[..4], expected[..4]);

    res.push_u8_slice(&[0b00000000, 0b11111111, 0b00000000]);
    expected[0] = 0b11100000;
    expected[1] = 0b00011111;
    expected[2] = 0b11100000;
    expected[3] = 0b00000000;
    assert_eq!(res.get_data()[..4], expected[..4]);
}

#[test]
fn push_bitfs_off() {
    let mut expected = [0u8; 64];
    let mut res = CompactQR::with_len(64);

    res.push_bits(0b0_0000_0000_0000_0000, 17);
    expected[0] |= 0b0000_0000;
    expected[1] |= 0b0000_0000;
    expected[2] |= 0b0000_0000;
    assert_eq!(res.len, 17);
    assert_eq!(res.get_data()[..8], expected[..8]);

    res.push_bits(0b1_1111_1111_1111_1111, 17);
    expected[2] |= 0b0111_1111;
    expected[3] |= 0b1111_1111;
    expected[4] |= 0b1100_0000;
    assert_eq!(res.get_data()[..8], expected[..8]);

    res.push_bits(0b0_0000_0000_0000_0000, 17);
    expected[4] |= 0b1100_0000;
    expected[5] |= 0b0000_0000;
    expected[6] |= 0b0000_0000;
    assert_eq!(res.get_data()[..8], expected[..8]);

    res.push_bits(0b1, 2);
    expected[5] |= 0b0000_0000;
    expected[6] |= 0b0000_1000;
    assert_eq!(res.get_data()[..8], expected[..8]);

    res.push_u8(0b1111_1111);
    expected[6] |= 0b0000_1111;
    expected[7] |= 0b1111_1000;
    assert_eq!(res.get_data()[..8], expected[..8]);
}

#[test]
fn push_random() {
    let mut expected = [0u8; 64];
    let mut res = CompactQR::with_len(64);

    res.push_bits(1, 1);
    expected[0] = 0b1000_0000;
    assert_eq!(res.get_data()[..8], expected[..8]);

    res.push_u8(0b10101010);
    expected[0] = 0b1101_0101;
    expected[1] = 0b0000_0000;
    assert_eq!(res.get_data()[..8], expected[..8]);

    res.push_bits(1, 1);
    expected[1] = 0b0100_0000;
    assert_eq!(res.get_data()[..8], expected[..8]);

    res.push_bits(1, 3);
    expected[1] = 0b0100_1000;
    assert_eq!(res.get_data()[..8], expected[..8]);
}
