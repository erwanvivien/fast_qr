use crate::module::{Matrix, Module};

#[test]
fn mask_0_test() {
    let mut mat = [[Module::data(false); 10]; 10];
    crate::datamasking::mask(&mut mat, 0);

    assert_eq!(
        mat.map(|x| x.map(|x| x.value())),
        [
            [true, false, true, false, true, false, true, false, true, false],
            [false, true, false, true, false, true, false, true, false, true],
            [true, false, true, false, true, false, true, false, true, false],
            [false, true, false, true, false, true, false, true, false, true],
            [true, false, true, false, true, false, true, false, true, false],
            [false, true, false, true, false, true, false, true, false, true],
            [true, false, true, false, true, false, true, false, true, false],
            [false, true, false, true, false, true, false, true, false, true],
            [true, false, true, false, true, false, true, false, true, false],
            [false, true, false, true, false, true, false, true, false, true],
        ]
    )
}
#[test]
fn mask_1_test() {
    let mut mat = [[Module::data(false); 10]; 10];
    crate::datamasking::mask(&mut mat, 1);

    assert_eq!(
        mat.map(|x| x.map(|x| x.value())),
        [
            [true, true, true, true, true, true, true, true, true, true],
            [false, false, false, false, false, false, false, false, false, false],
            [true, true, true, true, true, true, true, true, true, true],
            [false, false, false, false, false, false, false, false, false, false],
            [true, true, true, true, true, true, true, true, true, true],
            [false, false, false, false, false, false, false, false, false, false],
            [true, true, true, true, true, true, true, true, true, true],
            [false, false, false, false, false, false, false, false, false, false],
            [true, true, true, true, true, true, true, true, true, true],
            [false, false, false, false, false, false, false, false, false, false],
        ]
    )
}
#[test]
fn mask_2_test() {
    let mut mat = [[Module::data(false); 10]; 10];
    crate::datamasking::mask(&mut mat, 2);

    assert_eq!(
        mat.map(|x| x.map(|x| x.value())),
        [
            [true, false, false, true, false, false, true, false, false, true],
            [true, false, false, true, false, false, true, false, false, true],
            [true, false, false, true, false, false, true, false, false, true],
            [true, false, false, true, false, false, true, false, false, true],
            [true, false, false, true, false, false, true, false, false, true],
            [true, false, false, true, false, false, true, false, false, true],
            [true, false, false, true, false, false, true, false, false, true],
            [true, false, false, true, false, false, true, false, false, true],
            [true, false, false, true, false, false, true, false, false, true],
            [true, false, false, true, false, false, true, false, false, true],
        ]
    )
}
#[test]
fn mask_3_test() {
    let mut mat = [[Module::data(false); 10]; 10];
    crate::datamasking::mask(&mut mat, 3);

    assert_eq!(
        mat.map(|x| x.map(|x| x.value())),
        [
            [true, false, false, true, false, false, true, false, false, true],
            [false, false, true, false, false, true, false, false, true, false],
            [false, true, false, false, true, false, false, true, false, false],
            [true, false, false, true, false, false, true, false, false, true],
            [false, false, true, false, false, true, false, false, true, false],
            [false, true, false, false, true, false, false, true, false, false],
            [true, false, false, true, false, false, true, false, false, true],
            [false, false, true, false, false, true, false, false, true, false],
            [false, true, false, false, true, false, false, true, false, false],
            [true, false, false, true, false, false, true, false, false, true],
        ]
    )
}
#[test]
fn mask_4_test() {
    let mut mat = [[Module::data(false); 10]; 10];
    crate::datamasking::mask(&mut mat, 4);

    assert_eq!(
        mat.map(|x| x.map(|x| x.value())),
        [
            [true, true, true, false, false, false, true, true, true, false],
            [true, true, true, false, false, false, true, true, true, false],
            [false, false, false, true, true, true, false, false, false, true],
            [false, false, false, true, true, true, false, false, false, true],
            [true, true, true, false, false, false, true, true, true, false],
            [true, true, true, false, false, false, true, true, true, false],
            [false, false, false, true, true, true, false, false, false, true],
            [false, false, false, true, true, true, false, false, false, true],
            [true, true, true, false, false, false, true, true, true, false],
            [true, true, true, false, false, false, true, true, true, false],
        ]
    )
}
#[test]
fn mask_5_test() {
    let mut mat = [[Module::data(false); 10]; 10];
    crate::datamasking::mask(&mut mat, 5);

    assert_eq!(
        mat.map(|x| x.map(|x| x.value())),
        [
            [true, true, true, true, true, true, true, true, true, true],
            [true, false, false, false, false, false, true, false, false, false],
            [true, false, false, true, false, false, true, false, false, true],
            [true, false, true, false, true, false, true, false, true, false],
            [true, false, false, true, false, false, true, false, false, true],
            [true, false, false, false, false, false, true, false, false, false],
            [true, true, true, true, true, true, true, true, true, true],
            [true, false, false, false, false, false, true, false, false, false],
            [true, false, false, true, false, false, true, false, false, true],
            [true, false, true, false, true, false, true, false, true, false],
        ]
    )
}
#[test]
fn mask_6_test() {
    let mut mat = [[Module::data(false); 10]; 10];
    crate::datamasking::mask(&mut mat, 6);

    assert_eq!(
        mat.map(|x| x.map(|x| x.value())),
        [
            [true, true, true, true, true, true, true, true, true, true],
            [true, true, true, false, false, false, true, true, true, false],
            [true, true, false, true, true, false, true, true, false, true],
            [true, false, true, false, true, false, true, false, true, false],
            [true, false, true, true, false, true, true, false, true, true],
            [true, false, false, false, true, true, true, false, false, false],
            [true, true, true, true, true, true, true, true, true, true],
            [true, true, true, false, false, false, true, true, true, false],
            [true, true, false, true, true, false, true, true, false, true],
            [true, false, true, false, true, false, true, false, true, false]
        ]
    )
}
#[test]
fn mask_7_test() {
    let mut mat = [[Module::data(false); 10]; 10];
    crate::datamasking::mask(&mut mat, 7);

    assert_eq!(
        mat.map(|x| x.map(|x| x.value())),
        [
            [true, false, true, false, true, false, true, false, true, false],
            [false, false, false, true, true, true, false, false, false, true],
            [true, false, false, false, true, true, true, false, false, false],
            [false, true, false, true, false, true, false, true, false, true],
            [true, true, true, false, false, false, true, true, true, false],
            [false, true, true, true, false, false, false, true, true, true],
            [true, false, true, false, true, false, true, false, true, false],
            [false, false, false, true, true, true, false, false, false, true],
            [true, false, false, false, true, true, true, false, false, false],
            [false, true, false, true, false, true, false, true, false, true],
        ]
    )
}
