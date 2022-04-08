const MAT_FULL: [[bool; 10]; 10] = [[false; 10]; 10];

#[test]
fn mask_0_test() {
    let mut mat = [[false; 10]; 10];
    crate::datamasking::mask(&mut mat, 0, &MAT_FULL);

    assert_eq!(
        mat,
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
    let mut mat = [[false; 10]; 10];
    crate::datamasking::mask(&mut mat, 1, &MAT_FULL);

    assert_eq!(
        mat,
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
    let mut mat = [[false; 10]; 10];
    crate::datamasking::mask(&mut mat, 2, &MAT_FULL);

    assert_eq!(
        mat,
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
    let mut mat = [[false; 10]; 10];
    crate::datamasking::mask(&mut mat, 3, &MAT_FULL);

    assert_eq!(
        mat,
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
    let mut mat = [[false; 10]; 10];
    crate::datamasking::mask(&mut mat, 4, &MAT_FULL);

    assert_eq!(
        mat,
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
    let mut mat = [[false; 10]; 10];
    crate::datamasking::mask(&mut mat, 5, &MAT_FULL);

    assert_eq!(
        mat,
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
    let mut mat = [[false; 10]; 10];
    crate::datamasking::mask(&mut mat, 6, &MAT_FULL);

    assert_eq!(
        mat,
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
    let mut mat = [[false; 10]; 10];
    crate::datamasking::mask(&mut mat, 7, &MAT_FULL);

    assert_eq!(
        mat,
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
