// 0 => Numeric
// 1 => ALNUM
// 2 => KANJI
// 3 => BYTE

#[test]
fn encoding_numeric_0() {
    let res = crate::encoding::best_encoding(&String::from("589492"));
    assert_eq!(0, res);
}

#[test]
fn encoding_numeric_1() {
    let res =
        crate::encoding::best_encoding(&String::from("95904409521090298052194059450950249521940"));
    assert_eq!(0, res);
}

#[test]
fn encoding_alnum_0() {
    let res = crate::encoding::best_encoding(&String::from("HELLO WORLD"));
    assert_eq!(1, res);
}

#[test]
fn encoding_alnum_1() {
    let res = crate::encoding::best_encoding(&String::from(
        "HELLO WORLD MY NAME IS ERWAN VIVIEN: THIS IS A TEST//////",
    ));
    assert_eq!(1, res);
}

#[test]
fn encoding_byte_0() {
    let res = crate::encoding::best_encoding(&String::from("589492h"));
    assert_eq!(3, res);
}

#[test]
fn encoding_byte_1() {
    let res = crate::encoding::best_encoding(&String::from("HELLO WORLD!"));
    assert_eq!(3, res);
}
#[test]
fn encoding_byte_2() {
    let res = crate::encoding::best_encoding(&String::from(
        "HELLO WORLD MY NAME, IS ERWAN VIVIEN: THIS IS A TEST//////",
    ));
    assert_eq!(3, res);
}
