use crate::compact::{CompactQR, KEEP_LAST};
use crate::encode;
use crate::encode::Mode;
use crate::hardcode::cci_bits;

#[test]
fn best_encoding_numeric_0() {
    let res = encode::best_encoding(b"589492");
    assert_eq!(Mode::Numeric, res);
}

#[test]
fn best_encoding_numeric_1() {
    let res = encode::best_encoding(b"95904409521090298052194059450950249521940");
    assert_eq!(Mode::Numeric, res);
}

#[test]
fn best_encoding_alnum_0() {
    let res = encode::best_encoding(b"HELLO WORLD");
    assert_eq!(Mode::Alphanumeric, res);
}

#[test]
fn best_encoding_alnum_1() {
    let res = encode::best_encoding(b"HELLO WORLD MY NAME IS ERWAN VIVIEN: THIS IS A TEST//////");
    assert_eq!(Mode::Alphanumeric, res);
}

#[test]
fn best_encoding_byte_0() {
    let res = encode::best_encoding(b"589492h");
    assert_eq!(Mode::Byte, res);
}

#[test]
fn best_encoding_byte_1() {
    let res = encode::best_encoding(b"HELLO WORLD!");
    assert_eq!(Mode::Byte, res);
}

#[test]
fn best_encoding_byte_2() {
    let res = encode::best_encoding(b"HELLO WORLD MY NAME, IS ERWAN VIVIEN: THIS IS A TEST//////");
    assert_eq!(Mode::Byte, res);
}

fn test_encode_header(compact: &CompactQR, input: &[u8], expected_mode: Mode) {
    let res = compact.get_data();
    let mode = match res[0] >> 4 {
        0b0001 => Mode::Numeric,
        0b0010 => Mode::Alphanumeric,
        0b0100 => Mode::Byte,
        _ => panic!("Invalid encoding mode"),
    };

    assert_eq!(
        expected_mode, mode,
        "Encoding mode should be {:?}",
        expected_mode
    );

    let cci = cci_bits(crate::Version::V01, mode) as u16;
    let character_count: u16 = {
        let first_nb_bits = 4;
        let second_nb_bits = cci - first_nb_bits;

        let first = ((res[0] & 0b0000_1111) as u16) << second_nb_bits;
        let second_mask = u8::MAX << (8 - second_nb_bits);
        let second = ((res[1] & second_mask) as u16) >> (8 - second_nb_bits);

        first | second
    };

    assert_eq!(
        character_count,
        input.len() as u16,
        "Input length, should be {}",
        input.len()
    );
}

#[test]
fn encode_byte_1() {
    let mut compact = CompactQR::new();
    const INPUT: &[u8] = b"Hello WORLD!";
    encode::encode_byte(&mut compact, INPUT, 8);

    test_encode_header(&compact, INPUT, Mode::Byte);

    let res = compact.get_data();
    for (i, b) in INPUT.iter().enumerate() {
        assert_eq!(res[1 + i] & 0b111, *b >> 4, "Left part at index {}", i);
        assert_eq!(res[2 + i] >> 4, *b & 0b1111, "Right part at index {}", i);
    }
}

#[test]
fn encode_alphanumeric_1() {
    let mut compact = CompactQR::new();
    const INPUT: &[u8] = b"HELLO WORLD";
    encode::encode_alphanumeric(&mut compact, INPUT, 9);

    test_encode_header(&compact, INPUT, Mode::Alphanumeric);

    let keep_last = KEEP_LAST.map(|x| x as u16);

    let res = compact
        .get_data()
        .iter()
        .map(|&x| u16::from(x))
        .collect::<Vec<_>>();

    // 13, 'HE'
    assert_eq!(res[1] & 0b0000_0111, (17 * 45 + 14) >> 8);
    assert_eq!(res[2] & 0b1111_1111, (17 * 45 + 14) & keep_last[8]);
    // 24, 'LL'
    assert_eq!(res[3] & 0b1111_1111, (21 * 45 + 21) >> 3);
    assert_eq!(res[4] & 0b1110_0000, (21 * 45 + 21) << 5 & keep_last[8]);
    // 35, 'O '
    assert_eq!(res[4] & 0b0001_1111, (24 * 45 + 36) >> 6);
    assert_eq!(res[5] & 0b1111_1100, (24 * 45 + 36) << 2 & keep_last[8]);
    // 46, 'WO'
    assert_eq!(res[5] & 0b0000_0011, (32 * 45 + 24) >> 9);
    assert_eq!(res[6] & 0b1111_1111, (32 * 45 + 24) >> 1 & keep_last[8]);
    assert_eq!(res[7] & 0b1000_0000, (32 * 45 + 24) << 8 & keep_last[8]);
    // 57, 'RL'
    assert_eq!(res[7] & 0b0111_1111, (27 * 45 + 21) >> 4);
    assert_eq!(res[8] & 0b1111_0000, (27 * 45 + 21) << 4 & keep_last[8]);
    // 68, 'D'
    assert_eq!(res[8] & 0b0000_1111, (13) >> 2);
    assert_eq!(res[9] & 0b1100_0000, (13) << 6 & keep_last[8]);
}

#[test]
fn encode_numeric_1() {
    let mut compact = CompactQR::new();
    const INPUT: &[u8] = b"5894";
    encode::encode_numeric(&mut compact, INPUT, 10);

    test_encode_header(&compact, INPUT, Mode::Numeric);

    let keep_last = KEEP_LAST.map(|x| x as u16);

    let res = compact
        .get_data()
        .iter()
        .map(|&x| u16::from(x))
        .collect::<Vec<_>>();

    // 13, '589'
    assert_eq!(res[1] & 0b0000_0011, 589 >> 8);
    assert_eq!(res[2] & 0b1111_1111, (589 << 0) & keep_last[8]);
    // 24, '4'
    assert_eq!(res[3] & 0b1111_0000, (4 << 4) & keep_last[8]);
}

#[test]
fn encode_numeric_2() {
    let mut compact = CompactQR::new();
    const INPUT: &[u8] = b"58949";
    encode::encode_numeric(&mut compact, INPUT, 10);

    test_encode_header(&compact, INPUT, Mode::Numeric);

    let keep_last = KEEP_LAST.map(|x| x as u16);

    let res = compact
        .get_data()
        .iter()
        .map(|&x| u16::from(x))
        .collect::<Vec<_>>();

    // 13, '589'
    assert_eq!(res[1] & 0b0000_0011, 589 >> 8);
    assert_eq!(res[2] & 0b1111_1111, (589 << 0) & keep_last[8]);
    // 24, '49'
    assert_eq!(res[3] & 0b1111_1110, 49 << 1 & keep_last[8]);
}

#[test]
fn encode_numeric_3() {
    let mut compact = CompactQR::new();
    const INPUT: &[u8] = b"589491";
    encode::encode_numeric(&mut compact, INPUT, 10);

    test_encode_header(&compact, INPUT, Mode::Numeric);

    let keep_last = KEEP_LAST.map(|x| x as u16);

    let res = compact
        .get_data()
        .iter()
        .map(|&x| u16::from(x))
        .collect::<Vec<_>>();

    // 13, '589'
    assert_eq!(res[1] & 0b0000_0011, 589 >> 8);
    assert_eq!(res[2] & 0b1111_1111, (589 << 0) & keep_last[8]);
    // 24, '491'
    assert_eq!(res[3] & 0b1111_1111, (491 >> 2) & keep_last[8]);
    assert_eq!(res[4] & 0b1100_0000, (491 << 6) & keep_last[8]);
}

#[test]
fn encode_numeric_4() {
    let mut compact = CompactQR::new();
    const INPUT: &[u8] = b"200505150001";
    encode::encode_numeric(&mut compact, INPUT, 10);

    test_encode_header(&compact, INPUT, Mode::Numeric);

    let keep_last = KEEP_LAST.map(|x| x as u16);

    let res = compact
        .get_data()
        .iter()
        .map(|&x| u16::from(x))
        .collect::<Vec<_>>();

    // 13, '200'
    assert_eq!(res[1] & 0b0000_0011, 200 >> 8);
    assert_eq!(res[2] & 0b1111_1111, (200 << 0) & keep_last[8]);
    // 24, '505'
    assert_eq!(res[3] & 0b1111_1111, (505 >> 2) & keep_last[8]);
    assert_eq!(res[4] & 0b1100_0000, (505 << 6) & keep_last[8]);
    // 35, '150'
    assert_eq!(res[4] & 0b0011_1111, (150 >> 4) & keep_last[8]);
    assert_eq!(res[5] & 0b1111_0000, (150 << 4) & keep_last[8]);
    // 46, '001'
    assert_eq!(res[5] & 0b0000_1111, (1) >> 6);
    assert_eq!(res[6] & 0b1111_1100, (1) << 2 & keep_last[8]);
}
