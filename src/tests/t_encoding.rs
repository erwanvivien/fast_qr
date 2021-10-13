#[test]
fn encoding_numeric_0() {
    let res = crate::encode::best_encoding(b"589492");
    assert_eq!(crate::encode::Mode::Numeric, res);
}

#[test]
fn encoding_numeric_1() {
    let res = crate::encode::best_encoding(b"95904409521090298052194059450950249521940");
    assert_eq!(crate::encode::Mode::Numeric, res);
}

#[test]
fn encoding_alnum_0() {
    let res = crate::encode::best_encoding(b"HELLO WORLD");
    assert_eq!(crate::encode::Mode::Alphanumeric, res);
}

#[test]
fn encoding_alnum_1() {
    let res =
        crate::encode::best_encoding(b"HELLO WORLD MY NAME IS ERWAN VIVIEN: THIS IS A TEST//////");
    assert_eq!(crate::encode::Mode::Alphanumeric, res);
}

#[test]
fn encoding_byte_0() {
    let res = crate::encode::best_encoding(b"589492h");
    assert_eq!(crate::encode::Mode::Byte, res);
}

#[test]
fn encoding_byte_1() {
    let res = crate::encode::best_encoding(b"HELLO WORLD!");
    assert_eq!(crate::encode::Mode::Byte, res);
}
#[test]
fn encoding_byte_2() {
    let res =
        crate::encode::best_encoding(b"HELLO WORLD MY NAME, IS ERWAN VIVIEN: THIS IS A TEST//////");
    assert_eq!(crate::encode::Mode::Byte, res);
}
