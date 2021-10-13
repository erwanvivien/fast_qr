#[test]
fn mask_0_test() {
    let mat = [[false; 10]; 10];
    let mat = crate::datamasking::mask(mat, 0, &mat);

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
    let mat = [[false; 10]; 10];
    let mat = crate::datamasking::mask(mat, 1, &mat);

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
    let mat = [[false; 10]; 10];
    let mat = crate::datamasking::mask(mat, 2, &mat);

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
    let mat = [[false; 10]; 10];
    let mat = crate::datamasking::mask(mat, 3, &mat);

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
    let mat = [[false; 10]; 10];
    let mat = crate::datamasking::mask(mat, 4, &mat);

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
    let mat = [[false; 10]; 10];
    let mat = crate::datamasking::mask(mat, 5, &mat);

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
    let mat = [[false; 10]; 10];
    let mat = crate::datamasking::mask(mat, 6, &mat);

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
    let mat = [[false; 10]; 10];
    let mat = crate::datamasking::mask(mat, 7, &mat);

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
