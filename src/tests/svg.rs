use crate::convert::svg::{Shape, SvgBuilder};
use crate::module::Module;

// const SMALL_MAT: Matrix<2> = [
//     [Module::empty(true), Module::empty(false)],
//     [Module::empty(false), Module::empty(true)],
// ];
//
// #[test]
// pub fn small_square() {
//     let svg = SvgBuilder::new().shape(Shape::Square).build_mat(&SMALL_MAT);
//
//     let expected = concat!(
//         r#"<svg viewBox="0 0 10 10" xmlns="http://www.w3.org/2000/svg">"#,
//         r##"<rect width="10px" height="10px" fill="#ffffff"/>"##,
//         r##"<path d="M4,4h1v1h-1M5,5h1v1h-1" fill="#000000"/>"##,
//         r#"</svg>"#
//     );
//
//     assert_eq!(&svg, expected)
// }
//
// #[test]
// pub fn small_circle() {
//     let svg = SvgBuilder::new().shape(Shape::Circle).build_mat(&SMALL_MAT);
//
//     let expected = concat!(
//         r#"<svg viewBox="0 0 10 10" xmlns="http://www.w3.org/2000/svg">"#,
//         r##"<rect width="10px" height="10px" fill="#ffffff"/>"##,
//         r##"<path d="M5,4.5a.5,.5 0 1,1 0,-.1M6,5.5a.5,.5 0 1,1 0,-.1" fill="#000000"/>"##,
//         r#"</svg>"#,
//     );
//
//     assert_eq!(&svg, expected)
// }
//
// #[test]
// pub fn small_rounded_square() {
//     let svg = SvgBuilder::new()
//         .shape(Shape::RoundedSquare)
//         .build_mat(&SMALL_MAT);
//
//     let expected = concat!(
//         r#"<svg viewBox="0 0 10 10" xmlns="http://www.w3.org/2000/svg">"#,
//         r##"<rect width="10px" height="10px" fill="#ffffff"/>"##,
//         r##"<path d="M4.2,4.2 4.8,4.2 4.8,4.8 4.2,4.8zM5.2,5.2 5.8,5.2 5.8,5.8 5.2,5.8z" stroke-width=".3" stroke-linejoin="round" stroke="#000000" fill="#000000"/>"##,
//         r#"</svg>"#,
//     );
//
//     assert_eq!(&svg, expected)
// }
