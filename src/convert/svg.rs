//! Conversion to SVGs

use crate::QRCode;

#[derive(PartialEq, Eq, Ord, PartialOrd)]
/// Different possible Shapes
pub enum SvgShape {
    /// Square Shape
    Square,
    /// Circle Shape
    Circle,
    /// RoundedSquare Shape
    RoundedSquare,
    /// Vertical Shape
    Vertical,
    /// Horizontal Shape
    Horizontal,
    /// Diamond Shape
    Diamond,
}

/// Builder for svg, can set shape, margin, background_color, dot_color
pub struct SvgBuilder {
    shape: SvgShape,
    margin: usize,
    background_color: [u8; 4],
    dot_color: [u8; 4],
}

fn rgba2hex(color: [u8; 4]) -> String {
    let mut s = String::with_capacity(9);

    s.push('#');
    s.push_str(&*format!("{:02x}", color[0]));
    s.push_str(&*format!("{:02x}", color[1]));
    s.push_str(&*format!("{:02x}", color[2]));
    // s.push_str(COLORS[color[3] as usize]);

    s
}

impl SvgBuilder {
    /// Creates a Builder instance
    pub fn new() -> SvgBuilder {
        SvgBuilder {
            background_color: [255; 4],
            dot_color: [0, 0, 0, 255],
            margin: 4,
            shape: SvgShape::Square,
        }
    }

    /// Changes margin (default: 4)
    pub fn margin(&mut self, margin: usize) -> &mut Self {
        self.margin = margin;
        self
    }

    /// Changes margin (default: #000000)
    pub fn dot_color(&mut self, dot_color: [u8; 4]) -> &mut Self {
        self.dot_color = dot_color;
        self
    }

    /// Changes background color (default: #ffffff)
    pub fn background_color(&mut self, background_color: [u8; 4]) -> &mut Self {
        self.background_color = background_color;
        self
    }

    /// Changes shape (default: Square)
    pub fn shape(&mut self, shape: SvgShape) -> &mut Self {
        self.shape = shape;
        self
    }

    /// Generates resulting svg for a matrix
    pub fn build_mat<const N: usize>(&self, mat: &[[bool; N]; N]) -> String {
        let mut out = String::with_capacity(11 * N * N / 2);
        out.push_str(&*format!(
            r#"<svg viewBox="0 0 {0} {0}" xmlns="http://www.w3.org/2000/svg">"#,
            self.margin * 2 + N
        ));

        out.push_str(&*format!(
            r#"<rect width="{0}px" height="{0}px" fill="{1}"/><path d=""#,
            self.margin * 2 + N,
            rgba2hex(self.background_color)
        ));

        for (i, &line) in mat.iter().enumerate() {
            for (j, &cell) in line.iter().enumerate() {
                if !cell {
                    continue;
                }

                let current = match self.shape {
                    SvgShape::Square => format!("M{},{}h1v1h-1", j + self.margin, i + self.margin),
                    SvgShape::Circle => format!(
                        "M{},{}a.5,.5 0 1,1 0,-.1",
                        j + self.margin + 1,
                        (i + self.margin) as f64 + 0.5f64
                    ),
                    SvgShape::RoundedSquare => format!(
                        "M{0}.2,{1}.2 {0}.8,{1}.2 {0}.8,{1}.8 {0}.2,{1}.8z",
                        j + self.margin,
                        i + self.margin,
                    ),
                    SvgShape::Horizontal => {
                        format!("M{}.1,{}h1v.8h-1", j + self.margin, i + self.margin)
                    }
                    SvgShape::Vertical => {
                        format!("M{},{}.1h.8v1h-.8", j + self.margin, i + self.margin)
                    }
                    SvgShape::Diamond => {
                        format!(
                            "M{}.5,{}l.5,.5l-.5,.5l-.5,-.5z",
                            j + self.margin,
                            i + self.margin
                        )
                    }
                };

                out.push_str(&*current);
            }
        }

        if self.shape == SvgShape::RoundedSquare {
            out.push_str(&*format!(
                r##"" stroke-width=".3" stroke-linejoin="round" stroke="{}"##,
                rgba2hex(self.dot_color)
            ));
        }

        out.push_str(&*format!(
            r#"" fill="{}"/></svg>"#,
            rgba2hex(self.dot_color)
        ));

        return out;
    }

    /// Generates resulting svg for a qrcode
    pub fn build_qr(&self, qr: QRCode) -> String {
        match qr {
            QRCode::V01(mat) => self.build_mat(&*mat),
            QRCode::V02(mat) => self.build_mat(&*mat),
            QRCode::V03(mat) => self.build_mat(&*mat),
            QRCode::V04(mat) => self.build_mat(&*mat),
            QRCode::V05(mat) => self.build_mat(&*mat),
            QRCode::V06(mat) => self.build_mat(&*mat),
            QRCode::V07(mat) => self.build_mat(&*mat),
            QRCode::V08(mat) => self.build_mat(&*mat),
            QRCode::V09(mat) => self.build_mat(&*mat),
            QRCode::V10(mat) => self.build_mat(&*mat),
            QRCode::V11(mat) => self.build_mat(&*mat),
            QRCode::V12(mat) => self.build_mat(&*mat),
            QRCode::V13(mat) => self.build_mat(&*mat),
            QRCode::V14(mat) => self.build_mat(&*mat),
            QRCode::V15(mat) => self.build_mat(&*mat),
            QRCode::V16(mat) => self.build_mat(&*mat),
            QRCode::V17(mat) => self.build_mat(&*mat),
            QRCode::V18(mat) => self.build_mat(&*mat),
            QRCode::V19(mat) => self.build_mat(&*mat),
            QRCode::V20(mat) => self.build_mat(&*mat),
            QRCode::V21(mat) => self.build_mat(&*mat),
            QRCode::V22(mat) => self.build_mat(&*mat),
            QRCode::V23(mat) => self.build_mat(&*mat),
            QRCode::V24(mat) => self.build_mat(&*mat),
            QRCode::V25(mat) => self.build_mat(&*mat),
            QRCode::V26(mat) => self.build_mat(&*mat),
            QRCode::V27(mat) => self.build_mat(&*mat),
            QRCode::V28(mat) => self.build_mat(&*mat),
            QRCode::V29(mat) => self.build_mat(&*mat),
            QRCode::V30(mat) => self.build_mat(&*mat),
            QRCode::V31(mat) => self.build_mat(&*mat),
            QRCode::V32(mat) => self.build_mat(&*mat),
            QRCode::V33(mat) => self.build_mat(&*mat),
            QRCode::V34(mat) => self.build_mat(&*mat),
            QRCode::V35(mat) => self.build_mat(&*mat),
            QRCode::V36(mat) => self.build_mat(&*mat),
            QRCode::V37(mat) => self.build_mat(&*mat),
            QRCode::V38(mat) => self.build_mat(&*mat),
            QRCode::V39(mat) => self.build_mat(&*mat),
            QRCode::V40(mat) => self.build_mat(&*mat),
        }
    }
}

mod tests {
    use crate::convert::svg::{SvgBuilder, SvgShape};

    #[test]
    pub fn small_square() {
        let svg = SvgBuilder::new()
            .shape(SvgShape::Square)
            .build_mat(&[[true, false], [false, true]]);

        let expected = concat!(
            r#"<svg viewBox="0 0 10 10" xmlns="http://www.w3.org/2000/svg">"#,
            r##"<rect width="10px" height="10px" fill="#ffffff"/>"##,
            r##"<path d="M4,4h1v1h-1M5,5h1v1h-1" fill="#000000"/>"##,
            r#"</svg>"#
        );

        assert_eq!(&svg, expected)
    }

    #[test]
    pub fn small_circle() {
        let svg = SvgBuilder::new()
            .shape(SvgShape::Circle)
            .build_mat(&[[true, false], [false, true]]);

        let expected = concat!(
            r#"<svg viewBox="0 0 10 10" xmlns="http://www.w3.org/2000/svg">"#,
            r##"<rect width="10px" height="10px" fill="#ffffff"/>"##,
            r##"<path d="M5,4.5a.5,.5 0 1,1 0,-.1M6,5.5a.5,.5 0 1,1 0,-.1" fill="#000000"/>"##,
            r#"</svg>"#,
        );

        assert_eq!(&svg, expected)
    }

    #[test]
    pub fn small_rounded_square() {
        let svg = SvgBuilder::new()
            .shape(SvgShape::RoundedSquare)
            .build_mat(&[[true, false], [false, true]]);

        let expected = concat!(
            r#"<svg viewBox="0 0 10 10" xmlns="http://www.w3.org/2000/svg">"#,
            r##"<rect width="10px" height="10px" fill="#ffffff"/>"##,
            r##"<path d="M4.2,4.2 4.8,4.2 4.8,4.8 4.2,4.8zM5.2,5.2 5.8,5.2 5.8,5.8 5.2,5.8z" stroke-width=".3" stroke-linejoin="round" stroke="#000000" fill="#000000"/>"##,
            r#"</svg>"#,
        );

        assert_eq!(&svg, expected)
    }
}
