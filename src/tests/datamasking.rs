use crate::datamasking::Mask;
use crate::module::Module;

const F: bool = false;
const T: bool = true;

#[test]
fn mask_checkerboard_test() {
    let mut mat = [[Module::data(F); 10]; 10];
    crate::datamasking::mask(&mut mat, Mask::Checkerboard);

    assert_eq!(
        mat.map(|x| x.map(|x| x.value())),
        [
            [T, F, T, F, T, F, T, F, T, F],
            [F, T, F, T, F, T, F, T, F, T],
            [T, F, T, F, T, F, T, F, T, F],
            [F, T, F, T, F, T, F, T, F, T],
            [T, F, T, F, T, F, T, F, T, F],
            [F, T, F, T, F, T, F, T, F, T],
            [T, F, T, F, T, F, T, F, T, F],
            [F, T, F, T, F, T, F, T, F, T],
            [T, F, T, F, T, F, T, F, T, F],
            [F, T, F, T, F, T, F, T, F, T],
        ]
    )
}
#[test]
fn mask_horizontal_test() {
    let mut mat = [[Module::data(F); 10]; 10];
    crate::datamasking::mask(&mut mat, Mask::HorizontalLines);

    assert_eq!(
        mat.map(|x| x.map(|x| x.value())),
        [
            [T, T, T, T, T, T, T, T, T, T],
            [F, F, F, F, F, F, F, F, F, F],
            [T, T, T, T, T, T, T, T, T, T],
            [F, F, F, F, F, F, F, F, F, F],
            [T, T, T, T, T, T, T, T, T, T],
            [F, F, F, F, F, F, F, F, F, F],
            [T, T, T, T, T, T, T, T, T, T],
            [F, F, F, F, F, F, F, F, F, F],
            [T, T, T, T, T, T, T, T, T, T],
            [F, F, F, F, F, F, F, F, F, F],
        ]
    )
}
#[test]
fn mask_vertical_test() {
    let mut mat = [[Module::data(F); 10]; 10];
    crate::datamasking::mask(&mut mat, Mask::VerticalLines);

    assert_eq!(
        mat.map(|x| x.map(|x| x.value())),
        [
            [T, F, F, T, F, F, T, F, F, T],
            [T, F, F, T, F, F, T, F, F, T],
            [T, F, F, T, F, F, T, F, F, T],
            [T, F, F, T, F, F, T, F, F, T],
            [T, F, F, T, F, F, T, F, F, T],
            [T, F, F, T, F, F, T, F, F, T],
            [T, F, F, T, F, F, T, F, F, T],
            [T, F, F, T, F, F, T, F, F, T],
            [T, F, F, T, F, F, T, F, F, T],
            [T, F, F, T, F, F, T, F, F, T],
        ]
    )
}
#[test]
fn mask_diagonal_test() {
    let mut mat = [[Module::data(F); 10]; 10];
    crate::datamasking::mask(&mut mat, Mask::DiagonalLines);

    assert_eq!(
        mat.map(|x| x.map(|x| x.value())),
        [
            [T, F, F, T, F, F, T, F, F, T],
            [F, F, T, F, F, T, F, F, T, F],
            [F, T, F, F, T, F, F, T, F, F],
            [T, F, F, T, F, F, T, F, F, T],
            [F, F, T, F, F, T, F, F, T, F],
            [F, T, F, F, T, F, F, T, F, F],
            [T, F, F, T, F, F, T, F, F, T],
            [F, F, T, F, F, T, F, F, T, F],
            [F, T, F, F, T, F, F, T, F, F],
            [T, F, F, T, F, F, T, F, F, T],
        ]
    )
}
#[test]
fn mask_large_checkerboard_test() {
    let mut mat = [[Module::data(F); 10]; 10];
    crate::datamasking::mask(&mut mat, Mask::LargeCheckerboard);

    assert_eq!(
        mat.map(|x| x.map(|x| x.value())),
        [
            [T, T, T, F, F, F, T, T, T, F],
            [T, T, T, F, F, F, T, T, T, F],
            [F, F, F, T, T, T, F, F, F, T],
            [F, F, F, T, T, T, F, F, F, T],
            [T, T, T, F, F, F, T, T, T, F],
            [T, T, T, F, F, F, T, T, T, F],
            [F, F, F, T, T, T, F, F, F, T],
            [F, F, F, T, T, T, F, F, F, T],
            [T, T, T, F, F, F, T, T, T, F],
            [T, T, T, F, F, F, T, T, T, F],
        ]
    )
}
#[test]
fn mask_field_test() {
    let mut mat = [[Module::data(F); 10]; 10];
    crate::datamasking::mask(&mut mat, Mask::Fields);

    assert_eq!(
        mat.map(|x| x.map(|x| x.value())),
        [
            [T, T, T, T, T, T, T, T, T, T],
            [T, F, F, F, F, F, T, F, F, F],
            [T, F, F, T, F, F, T, F, F, T],
            [T, F, T, F, T, F, T, F, T, F],
            [T, F, F, T, F, F, T, F, F, T],
            [T, F, F, F, F, F, T, F, F, F],
            [T, T, T, T, T, T, T, T, T, T],
            [T, F, F, F, F, F, T, F, F, F],
            [T, F, F, T, F, F, T, F, F, T],
            [T, F, T, F, T, F, T, F, T, F],
        ]
    )
}
#[test]
fn mask_diamond_test() {
    let mut mat = [[Module::data(F); 10]; 10];
    crate::datamasking::mask(&mut mat, Mask::Diamonds);

    assert_eq!(
        mat.map(|x| x.map(|x| x.value())),
        [
            [T, T, T, T, T, T, T, T, T, T],
            [T, T, T, F, F, F, T, T, T, F],
            [T, T, F, T, T, F, T, T, F, T],
            [T, F, T, F, T, F, T, F, T, F],
            [T, F, T, T, F, T, T, F, T, T],
            [T, F, F, F, T, T, T, F, F, F],
            [T, T, T, T, T, T, T, T, T, T],
            [T, T, T, F, F, F, T, T, T, F],
            [T, T, F, T, T, F, T, T, F, T],
            [T, F, T, F, T, F, T, F, T, F]
        ]
    )
}
#[test]
fn mask_meadow_test() {
    let mut mat = [[Module::data(F); 10]; 10];
    crate::datamasking::mask(&mut mat, Mask::Meadow);

    assert_eq!(
        mat.map(|x| x.map(|x| x.value())),
        [
            [T, F, T, F, T, F, T, F, T, F],
            [F, F, F, T, T, T, F, F, F, T],
            [T, F, F, F, T, T, T, F, F, F],
            [F, T, F, T, F, T, F, T, F, T],
            [T, T, T, F, F, F, T, T, T, F],
            [F, T, T, T, F, F, F, T, T, T],
            [T, F, T, F, T, F, T, F, T, F],
            [F, F, F, T, T, T, F, F, F, T],
            [T, F, F, F, T, T, T, F, F, F],
            [F, T, F, T, F, T, F, T, F, T],
        ]
    )
}
