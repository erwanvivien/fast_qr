use crate::module::Module;
use crate::QRCode;

pub(crate) const F: bool = false;
pub(crate) const T: bool = true;

pub(crate) const DARK: fn(bool) -> Module = Module::dark;
pub(crate) const DATA: fn(bool) -> Module = Module::data;
pub(crate) const ALIG: fn(bool) -> Module = Module::alignment;
pub(crate) const FORM: fn(bool) -> Module = Module::format;
pub(crate) const VERS: fn(bool) -> Module = Module::version;
pub(crate) const TIMG: fn(bool) -> Module = Module::timing;
pub(crate) const FIND: fn(bool) -> Module = Module::finder_pattern;
pub(crate) const EMPT: fn(bool) -> Module = Module::empty;

#[test]
fn from_bool_v1() {
    #[rustfmt::skip]
    const MAT_FAST_QR_COM_V1_BOOL: [[bool; 21]; 21] = [[true, true, true, true, true, true, true, false, false, false, true, true, true, false, true, true, true, true, true, true, true],
        [true, false, false, false, false, false, true, false, true, true, true, false, false, false, true, false, false, false, false, false, true],
        [true, false, true, true, true, false, true, false, false, true, false, false, false, false, true, false, true, true, true, false, true],
        [true, false, true, true, true, false, true, false, false, false, true, false, false, false, true, false, true, true, true, false, true],
        [true, false, true, true, true, false, true, false, true, false, true, false, true, false, true, false, true, true, true, false, true],
        [true, false, false, false, false, false, true, false, false, true, false, false, true, false, true, false, false, false, false, false, true],
        [true, true, true, true, true, true, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true],
        [false, false, false, false, false, false, false, false, false, true, false, true, true, false, false, false, false, false, false, false, false],
        [true, false, true, false, true, false, true, false, false, true, true, true, false, false, false, false, true, false, false, true, false],
        [true, true, true, false, true, false, false, false, false, false, true, false, false, false, false, false, false, true, false, true, true],
        [true, false, false, true, false, false, true, true, false, false, true, false, true, true, false, false, true, false, true, false, true],
        [false, true, true, false, false, true, false, true, false, true, true, false, false, true, false, false, false, true, true, false, true],
        [true, false, true, true, false, false, true, false, false, true, false, false, true, false, false, false, false, false, false, false, true],
        [false, false, false, false, false, false, false, false, true, true, false, true, false, true, false, true, true, true, true, true, true],
        [true, true, true, true, true, true, true, false, false, true, false, true, false, true, true, false, false, true, true, false, false],
        [true, false, false, false, false, false, true, false, false, true, false, true, true, true, true, true, false, true, true, false, false],
        [true, false, true, true, true, false, true, false, true, true, false, true, false, false, false, false, false, false, false, true, true],
        [true, false, true, true, true, false, true, false, false, false, true, true, false, false, false, true, true, true, true, true, false],
        [true, false, true, true, true, false, true, false, true, true, true, false, true, true, false, false, false, false, false, false, true],
        [true, false, false, false, false, false, true, false, false, true, true, true, false, true, true, true, true, true, false, true, true],
        [true, true, true, true, true, true, true, false, true, true, false, true, false, false, false, true, true, true, true, false, true]
    ];

    #[rustfmt::skip]
        let mat_fast_qr_com_v1: [[Module; 21]; 21] = [
        [FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), EMPT(F), FORM(F), DATA(F), DATA(T), DATA(T), DATA(T), EMPT(F), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T)],
        [FIND(T), FIND(F), FIND(F), FIND(F), FIND(F), FIND(F), FIND(T), EMPT(F), FORM(T), DATA(T), DATA(T), DATA(F), DATA(F), EMPT(F), FIND(T), FIND(F), FIND(F), FIND(F), FIND(F), FIND(F), FIND(T)],
        [FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T), EMPT(F), FORM(F), DATA(T), DATA(F), DATA(F), DATA(F), EMPT(F), FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T)],
        [FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T), EMPT(F), FORM(F), DATA(F), DATA(T), DATA(F), DATA(F), EMPT(F), FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T)],
        [FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T), EMPT(F), FORM(T), DATA(F), DATA(T), DATA(F), DATA(T), EMPT(F), FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T)],
        [FIND(T), FIND(F), FIND(F), FIND(F), FIND(F), FIND(F), FIND(T), EMPT(F), FORM(F), DATA(T), DATA(F), DATA(F), DATA(T), EMPT(F), FIND(T), FIND(F), FIND(F), FIND(F), FIND(F), FIND(F), FIND(T)],
        [FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), EMPT(F), TIMG(T), TIMG(F), TIMG(T), TIMG(F), TIMG(T), EMPT(F), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T)],
        [EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), FORM(F), DATA(T), DATA(F), DATA(T), DATA(T), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F)],
        [FORM(T), FORM(F), FORM(T), FORM(F), FORM(T), FORM(F), TIMG(T), FORM(F), FORM(F), DATA(T), DATA(T), DATA(T), DATA(F), FORM(F), FORM(F), FORM(F), FORM(T), FORM(F), FORM(F), FORM(T), FORM(F)],
        [DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), TIMG(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T)],
        [DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), TIMG(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T)],
        [DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), TIMG(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T)],
        [DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), TIMG(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T)],
        [EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), DARK(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T)],
        [FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), EMPT(F), FORM(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F)],
        [FIND(T), FIND(F), FIND(F), FIND(F), FIND(F), FIND(F), FIND(T), EMPT(F), FORM(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F)],
        [FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T), EMPT(F), FORM(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T)],
        [FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T), EMPT(F), FORM(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F)],
        [FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T), EMPT(F), FORM(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T)],
        [FIND(T), FIND(F), FIND(F), FIND(F), FIND(F), FIND(F), FIND(T), EMPT(F), FORM(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T)],
        [FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), EMPT(F), FORM(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T)]
    ];

    let qr = crate::default::create_mat_from_bool(&MAT_FAST_QR_COM_V1_BOOL);

    for i in 0..qr.size {
        let row = &qr[i];
        for (j, elem) in row.iter().enumerate() {
            assert_eq!(elem, &mat_fast_qr_com_v1[i][j], "mat[{i}][{j}]");
        }
    }
}

#[test]
fn from_bool_v3() {
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

    #[rustfmt::skip]
        let mat_fast_qr_com_v3: [[Module; 29]; 29] = [
        [FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), EMPT(F), FORM(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), EMPT(F), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T)],
        [FIND(T), FIND(F), FIND(F), FIND(F), FIND(F), FIND(F), FIND(T), EMPT(F), FORM(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), EMPT(F), FIND(T), FIND(F), FIND(F), FIND(F), FIND(F), FIND(F), FIND(T)],
        [FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T), EMPT(F), FORM(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), EMPT(F), FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T)],
        [FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T), EMPT(F), FORM(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), EMPT(F), FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T)],
        [FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T), EMPT(F), FORM(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), EMPT(F), FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T)],
        [FIND(T), FIND(F), FIND(F), FIND(F), FIND(F), FIND(F), FIND(T), EMPT(F), FORM(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), EMPT(F), FIND(T), FIND(F), FIND(F), FIND(F), FIND(F), FIND(F), FIND(T)],
        [FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), EMPT(F), TIMG(T), TIMG(F), TIMG(T), TIMG(F), TIMG(T), TIMG(F), TIMG(T), TIMG(F), TIMG(T), TIMG(F), TIMG(T), TIMG(F), TIMG(T), EMPT(F), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T)],
        [EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), FORM(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F)],
        [FORM(F), FORM(F), FORM(F), FORM(F), FORM(F), FORM(T), TIMG(T), FORM(F), FORM(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), FORM(F), FORM(T), FORM(F), FORM(T), FORM(F), FORM(T), FORM(F), FORM(T)],
        [DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), TIMG(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F)],
        [DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), TIMG(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F)],
        [DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), TIMG(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F)],
        [DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), TIMG(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F)],
        [DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), TIMG(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T)],
        [DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), TIMG(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F)],
        [DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), TIMG(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T)],
        [DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), TIMG(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T)],
        [DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), TIMG(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T)],
        [DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), TIMG(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T)],
        [DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), TIMG(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F)],
        [DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), TIMG(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), ALIG(T), ALIG(T), ALIG(T), ALIG(T), ALIG(T), DATA(F), DATA(T), DATA(F), DATA(F)],
        [EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), DARK(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), ALIG(T), ALIG(F), ALIG(F), ALIG(F), ALIG(T), DATA(T), DATA(T), DATA(F), DATA(F)],
        [FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), EMPT(F), FORM(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), ALIG(T), ALIG(F), ALIG(T), ALIG(F), ALIG(T), DATA(F), DATA(F), DATA(T), DATA(F)],
        [FIND(T), FIND(F), FIND(F), FIND(F), FIND(F), FIND(F), FIND(T), EMPT(F), FORM(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), ALIG(T), ALIG(F), ALIG(F), ALIG(F), ALIG(T), DATA(T), DATA(F), DATA(F), DATA(F)],
        [FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T), EMPT(F), FORM(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), ALIG(T), ALIG(T), ALIG(T), ALIG(T), ALIG(T), DATA(F), DATA(F), DATA(F), DATA(T)],
        [FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T), EMPT(F), FORM(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F)],
        [FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T), EMPT(F), FORM(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F)],
        [FIND(T), FIND(F), FIND(F), FIND(F), FIND(F), FIND(F), FIND(T), EMPT(F), FORM(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T)],
        [FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), EMPT(F), FORM(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F)]
    ];

    let qr = crate::default::create_mat_from_bool(&MAT_FAST_QR_COM_BOOL);

    for i in 0..qr.size {
        let row = &qr[i];
        for (j, elem) in row.iter().enumerate() {
            assert_eq!(elem, &mat_fast_qr_com_v3[i][j], "mat[{i}][{j}]");
        }
    }
}

#[test]
fn from_bool_v7() {
    #[rustfmt::skip]
    const MAT_FAST_QR_COM_V7_BOOL: [[bool; 45]; 45] = [
        [true, true, true, true, true, true, true, false, true, false, false, false, false, true, true, false, true, true, false, true, true, true, true, false, false, true, true, true, false, true, true, true, false, false, false, false, true, false, true, true, true, true, true, true, true],
        [true, false, false, false, false, false, true, false, true, false, false, false, false, true, false, false, false, true, false, false, false, false, true, true, false, false, true, false, false, false, false, false, false, true, false, true, false, false, true, false, false, false, false, false, true],
        [true, false, true, true, true, false, true, false, true, false, true, false, true, false, false, true, false, false, false, false, true, true, true, true, true, true, true, true, true, true, false, false, false, false, false, true, false, false, true, false, true, true, true, false, true],
        [true, false, true, true, true, false, true, false, false, true, true, true, false, true, true, true, true, true, true, false, true, false, true, true, true, true, true, false, true, false, false, true, true, false, false, true, true, false, true, false, true, true, true, false, true],
        [true, false, true, true, true, false, true, false, false, false, true, false, true, false, false, false, true, false, false, true, true, true, true, true, true, true, false, false, true, true, false, false, true, true, true, true, true, false, true, false, true, true, true, false, true],
        [true, false, false, false, false, false, true, false, true, false, false, true, true, false, false, true, false, false, true, false, true, false, false, false, true, true, true, false, true, false, true, true, true, true, false, false, false, false, true, false, false, false, false, false, true],
        [true, true, true, true, true, true, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true],
        [false, false, false, false, false, false, false, false, true, false, false, true, true, false, true, false, true, false, false, false, true, false, false, false, true, false, false, true, false, false, true, true, false, false, true, true, false, false, false, false, false, false, false, false, false],
        [false, false, true, true, true, false, true, false, true, true, true, true, false, true, true, true, false, false, false, false, true, true, true, true, true, true, false, true, true, true, true, false, false, false, true, true, true, true, true, true, false, false, true, true, true],
        [true, false, false, false, false, true, false, false, true, true, true, true, true, true, false, false, false, false, false, true, false, true, true, true, false, false, false, true, true, false, true, true, false, true, false, false, true, false, false, false, false, true, false, true, false],
        [false, false, true, true, false, false, true, true, true, true, false, false, false, true, false, true, true, false, true, false, true, true, false, true, false, false, true, false, true, true, true, false, false, true, true, true, true, true, false, true, false, false, false, false, false],
        [false, true, true, true, true, true, false, true, false, true, false, true, false, true, true, false, false, true, false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, false, false, false, true, false, false, true, false, true, false, true, false],
        [false, false, false, true, false, true, true, false, false, true, false, true, true, true, true, true, true, true, false, false, false, false, false, true, true, false, false, false, false, false, false, true, true, true, true, false, true, false, false, false, true, false, true, false, false],
        [true, false, false, false, true, true, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, true, true, true, true, true, false, true, false, true, false, false, true, true, false, true, false, true, false, true, true, false, false, false, false],
        [false, false, true, true, false, false, true, true, false, false, true, false, false, false, true, false, true, false, false, true, true, false, false, false, true, false, false, true, true, false, true, true, true, true, true, false, false, true, true, false, true, false, true, true, true],
        [false, false, true, false, true, false, false, false, true, false, true, false, true, true, true, true, true, true, false, true, false, false, false, true, false, false, true, true, true, false, true, true, false, true, false, true, false, true, true, false, false, true, true, false, true],
        [true, true, true, true, false, true, true, true, true, true, false, false, false, true, false, false, true, true, false, true, true, true, false, true, true, true, true, true, true, true, true, false, false, false, false, true, false, false, true, true, false, true, false, true, true],
        [false, false, false, false, true, false, false, true, true, false, false, false, true, false, false, true, false, true, true, true, true, false, true, false, false, false, false, true, false, false, true, true, false, true, false, true, true, true, false, false, false, false, false, false, true],
        [true, false, true, false, true, false, true, true, false, true, false, false, false, true, true, false, false, true, false, false, false, false, false, false, true, false, false, false, true, true, true, false, false, false, false, true, true, false, true, true, false, true, true, true, false],
        [true, false, true, true, true, false, false, false, true, true, true, true, false, false, false, false, true, true, false, false, false, true, true, false, true, false, false, true, false, true, false, false, true, false, true, true, true, true, true, true, true, true, true, false, true],
        [true, false, false, true, true, true, true, true, true, true, true, true, false, true, true, true, true, false, false, false, true, true, true, true, true, true, false, false, true, false, false, true, false, true, true, false, true, true, true, true, true, false, false, false, true],
        [true, true, true, false, true, false, false, false, true, false, true, false, false, true, true, false, false, true, false, false, true, false, false, false, true, false, false, false, false, true, false, true, false, false, true, true, true, false, false, false, true, false, true, false, true],
        [true, true, true, false, true, false, true, false, true, true, false, false, false, true, false, false, false, true, true, false, true, false, true, false, true, true, false, false, false, false, true, false, true, true, false, false, true, false, true, false, true, true, true, true, false],
        [true, false, true, false, true, false, false, false, true, false, false, false, false, false, true, true, false, true, true, true, true, false, false, false, true, true, false, true, false, false, false, true, false, true, false, false, true, false, false, false, true, false, true, true, false],
        [true, false, true, true, true, true, true, true, true, true, true, false, false, false, false, true, false, true, false, false, true, true, true, true, true, true, true, true, true, true, true, false, false, false, false, false, true, true, true, true, true, true, true, true, false],
        [true, true, false, false, true, false, false, false, true, true, true, true, false, true, false, true, false, true, false, true, false, false, false, true, true, true, true, true, true, true, true, true, false, true, false, true, true, false, true, true, false, true, false, true, false],
        [true, true, false, true, true, true, true, false, false, false, false, false, true, false, false, true, false, true, true, false, true, true, false, false, false, true, false, true, true, true, false, false, false, false, false, true, false, true, true, false, false, false, false, false, false],
        [true, false, true, false, false, true, false, false, true, true, false, false, false, true, false, false, false, false, false, true, false, true, false, true, false, true, false, false, true, false, true, false, false, false, true, false, true, false, false, true, false, true, false, true, false],
        [true, false, false, false, false, true, true, true, false, true, true, true, false, true, true, false, false, false, true, true, false, false, true, false, true, false, false, false, false, false, true, true, true, false, true, false, false, false, false, false, true, false, true, false, false],
        [true, true, true, false, true, false, false, false, true, false, true, false, true, false, false, false, false, true, false, true, false, true, false, false, true, false, false, false, true, true, true, false, true, false, false, false, true, true, false, false, false, false, true, false, false],
        [true, false, true, true, true, true, true, true, true, false, false, true, true, true, true, false, true, true, true, true, false, false, false, true, false, true, false, true, false, false, true, true, true, false, true, false, true, false, false, true, false, true, true, true, true],
        [true, false, false, true, false, false, false, false, true, false, false, false, true, true, false, false, false, false, false, false, true, true, false, true, false, false, false, true, false, false, false, true, false, false, true, false, false, true, false, false, false, false, true, false, true],
        [true, false, true, true, false, true, true, true, false, false, false, false, false, true, true, false, false, true, false, true, true, false, false, false, false, true, false, false, true, true, false, false, false, false, true, true, true, true, true, true, false, true, true, true, true],
        [false, true, true, true, true, true, false, false, false, false, false, true, false, false, true, true, true, false, false, false, false, true, true, true, false, false, false, true, true, true, true, true, false, true, false, true, false, false, true, false, true, true, true, true, false],
        [false, false, false, false, true, false, true, true, false, true, true, true, true, false, true, false, true, true, true, false, true, false, false, false, false, false, true, false, false, true, true, false, false, true, false, true, false, true, true, true, true, false, true, false, false],
        [false, true, true, true, true, false, false, false, true, false, false, true, false, true, false, false, false, false, false, false, false, false, true, false, false, false, true, true, false, false, false, false, false, false, true, false, true, false, false, false, true, true, true, true, false],
        [true, false, false, true, true, false, true, false, false, true, true, false, false, true, false, false, true, false, false, true, true, true, true, true, true, true, true, false, true, false, false, true, true, true, true, false, true, true, true, true, true, true, false, false, false],
        [false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, true, true, true, true, false, true, false, false, false, true, false, false, true, true, false, true, false, true, false, true, true, true, false, false, false, true, false, true, false, true],
        [true, true, true, true, true, true, true, false, false, true, true, false, true, true, false, false, true, true, false, true, true, false, true, false, true, false, false, false, false, false, false, true, true, true, false, false, true, false, true, false, true, false, false, true, false],
        [true, false, false, false, false, false, true, false, false, false, false, true, true, false, false, false, false, true, true, false, true, false, false, false, true, true, false, false, false, true, false, false, true, true, false, false, true, false, false, false, true, true, true, true, true],
        [true, false, true, true, true, false, true, false, true, false, true, false, true, true, true, false, true, false, false, true, true, true, true, true, true, false, true, false, false, false, false, true, false, false, false, false, true, true, true, true, true, false, false, true, true],
        [true, false, true, true, true, false, true, false, true, true, true, true, false, true, true, false, false, true, false, true, true, false, false, true, true, false, true, true, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, true, false],
        [true, false, true, true, true, false, true, false, true, false, true, true, false, true, true, false, true, true, true, true, false, true, false, false, true, false, false, false, false, true, true, false, true, false, false, true, true, true, true, true, false, false, false, false, false],
        [true, false, false, false, false, false, true, false, false, false, false, true, false, false, false, true, false, false, false, false, false, true, true, true, false, true, false, false, true, true, false, false, false, false, true, false, true, false, false, false, false, true, false, false, false],
        [true, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true, true, false, false, false, false, true, true, false, true, true, true, true, true, true, true, true, true, true, true, true, false, false, true, true, false]
    ];

    #[rustfmt::skip]
        let mat_fast_qr_com_v7: [[Module; 45]; 45] = [
        [FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), EMPT(F), FORM(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), VERS(F), VERS(F), VERS(T), EMPT(F), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T)],
        [FIND(T), FIND(F), FIND(F), FIND(F), FIND(F), FIND(F), FIND(T), EMPT(F), FORM(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), VERS(F), VERS(T), VERS(F), EMPT(F), FIND(T), FIND(F), FIND(F), FIND(F), FIND(F), FIND(F), FIND(T)],
        [FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T), EMPT(F), FORM(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), VERS(F), VERS(T), VERS(F), EMPT(F), FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T)],
        [FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T), EMPT(F), FORM(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), VERS(F), VERS(T), VERS(T), EMPT(F), FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T)],
        [FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T), EMPT(F), FORM(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), ALIG(T), ALIG(T), ALIG(T), ALIG(T), ALIG(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), VERS(T), VERS(T), VERS(T), EMPT(F), FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T)],
        [FIND(T), FIND(F), FIND(F), FIND(F), FIND(F), FIND(F), FIND(T), EMPT(F), FORM(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), ALIG(T), ALIG(F), ALIG(F), ALIG(F), ALIG(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), VERS(F), VERS(F), VERS(F), EMPT(F), FIND(T), FIND(F), FIND(F), FIND(F), FIND(F), FIND(F), FIND(T)],
        [FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), EMPT(F), TIMG(T), TIMG(F), TIMG(T), TIMG(F), TIMG(T), TIMG(F), TIMG(T), TIMG(F), TIMG(T), TIMG(F), TIMG(T), TIMG(F), ALIG(T), ALIG(F), ALIG(T), ALIG(F), ALIG(T), TIMG(F), TIMG(T), TIMG(F), TIMG(T), TIMG(F), TIMG(T), TIMG(F), TIMG(T), TIMG(F), TIMG(T), TIMG(F), TIMG(T), EMPT(F), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T)],
        [EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), FORM(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), ALIG(T), ALIG(F), ALIG(F), ALIG(F), ALIG(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F)],
        [FORM(F), FORM(F), FORM(T), FORM(T), FORM(T), FORM(F), TIMG(T), FORM(F), FORM(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), ALIG(T), ALIG(T), ALIG(T), ALIG(T), ALIG(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), FORM(T), FORM(T), FORM(T), FORM(F), FORM(F), FORM(T), FORM(T), FORM(T)],
        [DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), TIMG(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F)],
        [DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), TIMG(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F)],
        [DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), TIMG(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F)],
        [DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), TIMG(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F)],
        [DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), TIMG(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F)],
        [DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), TIMG(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T)],
        [DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), TIMG(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T)],
        [DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), TIMG(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T)],
        [DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), TIMG(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T)],
        [DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), TIMG(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F)],
        [DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), TIMG(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T)],
        [DATA(T), DATA(F), DATA(F), DATA(T), ALIG(T), ALIG(T), ALIG(T), ALIG(T), ALIG(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), ALIG(T), ALIG(T), ALIG(T), ALIG(T), ALIG(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), ALIG(T), ALIG(T), ALIG(T), ALIG(T), ALIG(T), DATA(F), DATA(F), DATA(F), DATA(T)],
        [DATA(T), DATA(T), DATA(T), DATA(F), ALIG(T), ALIG(F), ALIG(F), ALIG(F), ALIG(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), ALIG(T), ALIG(F), ALIG(F), ALIG(F), ALIG(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), ALIG(T), ALIG(F), ALIG(F), ALIG(F), ALIG(T), DATA(F), DATA(T), DATA(F), DATA(T)],
        [DATA(T), DATA(T), DATA(T), DATA(F), ALIG(T), ALIG(F), ALIG(T), ALIG(F), ALIG(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), ALIG(T), ALIG(F), ALIG(T), ALIG(F), ALIG(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), ALIG(T), ALIG(F), ALIG(T), ALIG(F), ALIG(T), DATA(T), DATA(T), DATA(T), DATA(F)],
        [DATA(T), DATA(F), DATA(T), DATA(F), ALIG(T), ALIG(F), ALIG(F), ALIG(F), ALIG(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), ALIG(T), ALIG(F), ALIG(F), ALIG(F), ALIG(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), ALIG(T), ALIG(F), ALIG(F), ALIG(F), ALIG(T), DATA(F), DATA(T), DATA(T), DATA(F)],
        [DATA(T), DATA(F), DATA(T), DATA(T), ALIG(T), ALIG(T), ALIG(T), ALIG(T), ALIG(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), ALIG(T), ALIG(T), ALIG(T), ALIG(T), ALIG(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), ALIG(T), ALIG(T), ALIG(T), ALIG(T), ALIG(T), DATA(T), DATA(T), DATA(T), DATA(F)],
        [DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), TIMG(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F)],
        [DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), TIMG(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F)],
        [DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), TIMG(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F)],
        [DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), TIMG(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F)],
        [DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), TIMG(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F)],
        [DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), TIMG(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T)],
        [DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), TIMG(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T)],
        [DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), TIMG(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T)],
        [DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), TIMG(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F)],
        [VERS(F), VERS(F), VERS(F), VERS(F), VERS(T), VERS(F), TIMG(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F)],
        [VERS(F), VERS(T), VERS(T), VERS(T), VERS(T), VERS(F), TIMG(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F)],
        [VERS(T), VERS(F), VERS(F), VERS(T), VERS(T), VERS(F), TIMG(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), ALIG(T), ALIG(T), ALIG(T), ALIG(T), ALIG(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), ALIG(T), ALIG(T), ALIG(T), ALIG(T), ALIG(T), DATA(T), DATA(F), DATA(F), DATA(F)],
        [EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), EMPT(F), DARK(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), ALIG(T), ALIG(F), ALIG(F), ALIG(F), ALIG(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), ALIG(T), ALIG(F), ALIG(F), ALIG(F), ALIG(T), DATA(F), DATA(T), DATA(F), DATA(T)],
        [FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), EMPT(F), FORM(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), ALIG(T), ALIG(F), ALIG(T), ALIG(F), ALIG(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), ALIG(T), ALIG(F), ALIG(T), ALIG(F), ALIG(T), DATA(F), DATA(F), DATA(T), DATA(F)],
        [FIND(T), FIND(F), FIND(F), FIND(F), FIND(F), FIND(F), FIND(T), EMPT(F), FORM(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), ALIG(T), ALIG(F), ALIG(F), ALIG(F), ALIG(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), ALIG(T), ALIG(F), ALIG(F), ALIG(F), ALIG(T), DATA(T), DATA(T), DATA(T), DATA(T)],
        [FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T), EMPT(F), FORM(T), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), ALIG(T), ALIG(T), ALIG(T), ALIG(T), ALIG(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), ALIG(T), ALIG(T), ALIG(T), ALIG(T), ALIG(T), DATA(F), DATA(F), DATA(T), DATA(T)],
        [FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T), EMPT(F), FORM(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F)],
        [FIND(T), FIND(F), FIND(T), FIND(T), FIND(T), FIND(F), FIND(T), EMPT(F), FORM(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F)],
        [FIND(T), FIND(F), FIND(F), FIND(F), FIND(F), FIND(F), FIND(T), EMPT(F), FORM(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(F), DATA(F), DATA(F)],
        [FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), FIND(T), EMPT(F), FORM(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(T), DATA(F), DATA(F), DATA(T), DATA(T), DATA(F)]
    ];

    let qr = crate::default::create_mat_from_bool(&MAT_FAST_QR_COM_V7_BOOL);

    for i in 0..qr.size {
        let row = &qr[i];
        for (j, elem) in row.iter().enumerate() {
            assert_eq!(elem, &mat_fast_qr_com_v7[i][j], "mat[{i}][{j}]");
        }
    }
}

#[test]
fn transpose() {
    let mut qr = QRCode::default(10);
    for i in 0..100 {
        qr.data[i] = Module(i as u8);
    }

    let transpose = crate::default::transpose(&qr);
    for i in 0..10 {
        for j in 0..10 {
            assert_eq!(
                transpose[j][i], qr[i][j],
                "transpose[{i}][{j}] doesn't match"
            );
        }
    }
}
