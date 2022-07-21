use crate::module::Module;

#[rustfmt::skip]
const MAT_FAST_QR_COM: [[Module; 29]; 29] =[
    [Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(14), Module(9), Module(1), Module(1), Module(1), Module(1), Module(1), Module(1), Module(1), Module(1), Module(1), Module(1), Module(1), Module(0), Module(14), Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(3)],
    [Module(3), Module(2), Module(2), Module(2), Module(2), Module(2), Module(3), Module(14), Module(8), Module(0), Module(0), Module(0), Module(0), Module(1), Module(1), Module(0), Module(0), Module(1), Module(1), Module(1), Module(0), Module(14), Module(3), Module(2), Module(2), Module(2), Module(2), Module(2), Module(3)],
    [Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3), Module(14), Module(9), Module(0), Module(1), Module(0), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0), Module(0), Module(1), Module(1), Module(14), Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3)],
    [Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3), Module(14), Module(8), Module(1), Module(1), Module(1), Module(1), Module(0), Module(1), Module(0), Module(0), Module(1), Module(0), Module(1), Module(0), Module(14), Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3)],
    [Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3), Module(14), Module(9), Module(0), Module(0), Module(1), Module(0), Module(1), Module(0), Module(0), Module(1), Module(0), Module(0), Module(1), Module(1), Module(14), Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3)],
    [Module(3), Module(2), Module(2), Module(2), Module(2), Module(2), Module(3), Module(14), Module(8), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(1), Module(0), Module(1), Module(0), Module(1), Module(14), Module(3), Module(2), Module(2), Module(2), Module(2), Module(2), Module(3)],
    [Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(14), Module(7), Module(6), Module(7), Module(6), Module(7), Module(6), Module(7), Module(6), Module(7), Module(6), Module(7), Module(6), Module(7), Module(14), Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(3)],
    [Module(14), Module(14), Module(14), Module(14), Module(14), Module(14), Module(14), Module(14), Module(9), Module(0), Module(1), Module(0), Module(0), Module(1), Module(0), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0), Module(14), Module(14), Module(14), Module(14), Module(14), Module(14), Module(14), Module(14)],
    [Module(8), Module(8), Module(8), Module(8), Module(8), Module(9), Module(7), Module(8), Module(8), Module(0), Module(1), Module(1), Module(1), Module(0), Module(1), Module(0), Module(0), Module(0), Module(1), Module(0), Module(1), Module(8), Module(9), Module(8), Module(9), Module(8), Module(9), Module(8), Module(9)],
    [Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(6), Module(1), Module(1), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0), Module(0), Module(0), Module(0), Module(0), Module(0), Module(1), Module(0), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(0)],
    [Module(0), Module(1), Module(0), Module(0), Module(1), Module(0), Module(7), Module(0), Module(1), Module(0), Module(1), Module(0), Module(1), Module(1), Module(0), Module(0), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(1), Module(0), Module(1), Module(0), Module(0), Module(0)],
    [Module(0), Module(0), Module(0), Module(1), Module(1), Module(0), Module(6), Module(0), Module(0), Module(0), Module(1), Module(1), Module(1), Module(0), Module(0), Module(1), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(1), Module(0), Module(1), Module(0), Module(0), Module(0)],
    [Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(7), Module(0), Module(1), Module(0), Module(1), Module(1), Module(0), Module(0), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(0)],
    [Module(1), Module(0), Module(1), Module(1), Module(0), Module(0), Module(6), Module(0), Module(0), Module(1), Module(0), Module(0), Module(0), Module(1), Module(0), Module(0), Module(1), Module(1), Module(1), Module(1), Module(1), Module(0), Module(1), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1)],
    [Module(1), Module(1), Module(0), Module(0), Module(0), Module(0), Module(7), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0), Module(0), Module(0), Module(1), Module(0), Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0), Module(0)],
    [Module(0), Module(0), Module(1), Module(0), Module(1), Module(0), Module(6), Module(0), Module(0), Module(0), Module(1), Module(1), Module(0), Module(0), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(1), Module(1)],
    [Module(0), Module(0), Module(1), Module(0), Module(1), Module(0), Module(7), Module(1), Module(0), Module(0), Module(1), Module(0), Module(1), Module(0), Module(1), Module(1), Module(0), Module(0), Module(1), Module(0), Module(0), Module(1), Module(0), Module(0), Module(0), Module(1), Module(1), Module(0), Module(1)],
    [Module(1), Module(1), Module(1), Module(1), Module(0), Module(0), Module(6), Module(1), Module(0), Module(0), Module(1), Module(0), Module(0), Module(0), Module(1), Module(0), Module(1), Module(1), Module(0), Module(0), Module(1), Module(0), Module(1), Module(1), Module(1), Module(0), Module(0), Module(0), Module(1)],
    [Module(1), Module(1), Module(1), Module(1), Module(1), Module(1), Module(7), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(1), Module(1), Module(0), Module(0), Module(1), Module(0), Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(0), Module(0), Module(1)],
    [Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(6), Module(0), Module(1), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(0), Module(0), Module(0), Module(1), Module(1), Module(1), Module(1), Module(1), Module(0), Module(1), Module(0), Module(0), Module(0), Module(0)],
    [Module(1), Module(0), Module(0), Module(1), Module(0), Module(0), Module(7), Module(0), Module(0), Module(0), Module(1), Module(1), Module(0), Module(1), Module(0), Module(0), Module(1), Module(1), Module(0), Module(1), Module(5), Module(5), Module(5), Module(5), Module(5), Module(0), Module(1), Module(0), Module(0)],
    [Module(14), Module(14), Module(14), Module(14), Module(14), Module(14), Module(14), Module(14), Module(13), Module(0), Module(1), Module(0), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(0), Module(5), Module(4), Module(4), Module(4), Module(5), Module(1), Module(1), Module(0), Module(0)],
    [Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(14), Module(8), Module(0), Module(0), Module(1), Module(1), Module(0), Module(0), Module(0), Module(1), Module(1), Module(1), Module(1), Module(5), Module(4), Module(5), Module(4), Module(5), Module(0), Module(0), Module(1), Module(0)],
    [Module(3), Module(2), Module(2), Module(2), Module(2), Module(2), Module(3), Module(14), Module(9), Module(1), Module(0), Module(1), Module(1), Module(1), Module(1), Module(0), Module(0), Module(0), Module(1), Module(1), Module(5), Module(4), Module(4), Module(4), Module(5), Module(1), Module(0), Module(0), Module(0)],
    [Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3), Module(14), Module(8), Module(1), Module(1), Module(0), Module(1), Module(0), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(5), Module(5), Module(5), Module(5), Module(5), Module(0), Module(0), Module(0), Module(1)],
    [Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3), Module(14), Module(8), Module(1), Module(1), Module(0), Module(0), Module(0), Module(1), Module(0), Module(1), Module(0), Module(1), Module(1), Module(1), Module(0), Module(0), Module(1), Module(0), Module(1), Module(1), Module(1), Module(0)],
    [Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3), Module(14), Module(8), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(1), Module(1), Module(1), Module(1), Module(0), Module(1), Module(1), Module(0)],
    [Module(3), Module(2), Module(2), Module(2), Module(2), Module(2), Module(3), Module(14), Module(8), Module(1), Module(0), Module(1), Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(0), Module(0), Module(1), Module(1), Module(0), Module(0), Module(1), Module(1), Module(1), Module(0), Module(1)],
    [Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(14), Module(8), Module(0), Module(1), Module(1), Module(0), Module(0), Module(1), Module(1), Module(1), Module(1), Module(1), Module(0), Module(0), Module(0), Module(1), Module(0), Module(1), Module(0), Module(1), Module(0), Module(0)]
];

#[rustfmt::skip]
const MAT_FAST_QR_COM_BOOL: [[bool; 29]; 29] = [
    [true, true, true, true, true, true, true, false, true, true, true, true, true, true, true, true, true, true, true, true, false, false, true, true, true, true, true, true, true],
    [true, false, false, false, false, false, true, false, false, false, false, false, false, true, true, false, false, true, true, true, false, false, true, false, false, false, false, false, true],
    [true, false, true, true, true, false, true, false, true, false, true, false, true, true, false, false, false, false, false, true, true, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, false, true, true, true, true, false, true, false, false, true, false, true, false, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, true, false, false, true, false, true, false, false, true, false, false, true, true, false, true, false, true, true, true, false, true],
    [true, false, false, false, false, false, true, false, false, false, true, true, false, true, true, true, true, false, true, false, true, false, true, false, false, false, false, false, true],
    [true, true, true, true, true, true, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true],
    [false, false, false, false, false, false, false, false, true, false, true, false, false, true, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false],
    [false, false, false, false, false, true, true, false, false, false, true, true, true, false, true, false, false, false, true, false, true, false, true, false, true, false, true, false, true],
    [true, true, false, true, true, true, false, true, true, true, true, false, false, false, false, false, false, false, false, false, true, false, false, true, true, false, true, true, false],
    [false, true, false, false, true, false, true, false, true, false, true, false, true, true, false, false, false, true, true, false, true, true, true, true, false, true, false, false, false],
    [false, false, false, true, true, false, false, false, false, false, true, true, true, false, false, true, true, true, false, false, false, false, true, true, false, true, false, false, false],
    [true, true, false, true, true, true, true, false, true, false, true, true, false, false, false, true, true, false, true, true, true, true, true, false, false, false, false, true, false],
    [true, false, true, true, false, false, false, false, false, true, false, false, false, true, false, false, true, true, true, true, true, false, true, false, true, true, false, true, true],
    [true, true, false, false, false, false, true, true, true, false, false, false, false, false, false, true, false, true, false, false, false, false, true, true, false, false, false, false, false],
    [false, false, true, false, true, false, false, false, false, false, true, true, false, false, false, true, true, false, true, true, false, true, true, false, true, true, true, true, true],
    [false, false, true, false, true, false, true, true, false, false, true, false, true, false, true, true, false, false, true, false, false, true, false, false, false, true, true, false, true],
    [true, true, true, true, false, false, false, true, false, false, true, false, false, false, true, false, true, true, false, false, true, false, true, true, true, false, false, false, true],
    [true, true, true, true, true, true, true, true, true, false, false, false, false, true, true, true, false, false, true, false, true, false, false, false, false, true, false, false, true],
    [true, false, false, false, false, true, false, false, true, false, true, true, false, true, true, false, false, false, true, true, true, true, true, false, true, false, false, false, false],
    [true, false, false, true, false, false, true, false, false, false, true, true, false, true, false, false, true, true, false, true, true, true, true, true, true, false, true, false, false],
    [false, false, false, false, false, false, false, false, true, false, true, false, false, true, true, false, true, true, true, false, true, false, false, false, true, true, true, false, false],
    [true, true, true, true, true, true, true, false, false, false, false, true, true, false, false, false, true, true, true, true, true, false, true, false, true, false, false, true, false],
    [true, false, false, false, false, false, true, false, true, true, false, true, true, true, true, false, false, false, true, true, true, false, false, false, true, true, false, false, false],
    [true, false, true, true, true, false, true, false, false, true, true, false, true, false, false, true, true, false, true, true, true, true, true, true, true, false, false, false, true],
    [true, false, true, true, true, false, true, false, false, true, true, false, false, false, true, false, true, false, true, true, true, false, false, true, false, true, true, true, false],
    [true, false, true, true, true, false, true, false, false, true, true, false, false, false, false, true, true, false, true, true, true, true, true, true, true, false, true, true, false],
    [true, false, false, false, false, false, true, false, false, true, false, true, true, true, false, true, true, true, false, false, true, true, false, false, true, true, true, false, true],
    [true, true, true, true, true, true, true, false, false, false, true, true, false, false, true, true, true, true, true, false, false, false, true, false, true, false, true, false, false]
];

#[test]
fn from_bool() {
    let mat = crate::default::create_mat_from_bool(&MAT_FAST_QR_COM_BOOL);

    for (i, row) in mat.iter().enumerate() {
        for (j, elem) in row.iter().enumerate() {
            assert_eq!(elem, &MAT_FAST_QR_COM[i][j], "mat[{i}][{j}]");
        }
    }
}
