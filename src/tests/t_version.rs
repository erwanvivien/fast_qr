#[test]
fn version_size_01() {
    let mat = crate::default::create_matrix_from_version(1);
    let length = mat.len();
    assert_eq!(length, 21);
}
#[test]
fn version_size_03() {
    let mat = crate::default::create_matrix_from_version(3);
    let length = mat.len();
    assert_eq!(length, 29);
}
#[test]
fn version_size_22() {
    let mat = crate::default::create_matrix_from_version(22);
    let length = mat.len();
    assert_eq!(length, 105);
}
#[test]
fn version_size_40() {
    let mat = crate::default::create_matrix_from_version(40);
    let length = mat.len();
    assert_eq!(length, 177);
}
