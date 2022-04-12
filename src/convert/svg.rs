use crate::convert::svg::SvgShape::Square;
use crate::QRCode;

#[derive(PartialEq, Eq, Ord, PartialOrd)]
pub enum SvgShape {
    Square,
    Circle,
    RoundedSquare,
    Vertical,
    Horizontal,
    Diamond,
}

pub struct SvgBuilder {
    shape: SvgShape,
    margin: usize,
    background_color: [u8; 4],
    dot_color: [u8; 4],
}

fn rgba2hex(color: [u8; 4]) -> String {
    const COLORS: [&'static str; 256] = [
        "00", "01", "02", "03", "04", "05", "06", "07", "08", "09", "0a", "0b", "0c", "0d", "0e",
        "0f", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "1a", "1b", "1c", "1d",
        "1e", "1f", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "2a", "2b", "2c",
        "2d", "2e", "2f", "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "3a", "3b",
        "3c", "3d", "3e", "3f", "40", "41", "42", "43", "44", "45", "46", "47", "48", "49", "4a",
        "4b", "4c", "4d", "4e", "4f", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59",
        "5a", "5b", "5c", "5d", "5e", "5f", "60", "61", "62", "63", "64", "65", "66", "67", "68",
        "69", "6a", "6b", "6c", "6d", "6e", "6f", "70", "71", "72", "73", "74", "75", "76", "77",
        "78", "79", "7a", "7b", "7c", "7d", "7e", "7f", "80", "81", "82", "83", "84", "85", "86",
        "87", "88", "89", "8a", "8b", "8c", "8d", "8e", "8f", "90", "91", "92", "93", "94", "95",
        "96", "97", "98", "99", "9a", "9b", "9c", "9d", "9e", "9f", "a0", "a1", "a2", "a3", "a4",
        "a5", "a6", "a7", "a8", "a9", "aa", "ab", "ac", "ad", "ae", "af", "b0", "b1", "b2", "b3",
        "b4", "b5", "b6", "b7", "b8", "b9", "ba", "bb", "bc", "bd", "be", "bf", "c0", "c1", "c2",
        "c3", "c4", "c5", "c6", "c7", "c8", "c9", "ca", "cb", "cc", "cd", "ce", "cf", "d0", "d1",
        "d2", "d3", "d4", "d5", "d6", "d7", "d8", "d9", "da", "db", "dc", "dd", "de", "df", "e0",
        "e1", "e2", "e3", "e4", "e5", "e6", "e7", "e8", "e9", "ea", "eb", "ec", "ed", "ee", "ef",
        "f0", "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "fa", "fb", "fc", "fd", "fe",
        "ff",
    ];

    let mut s = String::with_capacity(9);

    s.push('#');
    s.push_str(COLORS[color[0] as usize]);
    s.push_str(COLORS[color[1] as usize]);
    s.push_str(COLORS[color[2] as usize]);
    // s.push_str(COLORS[color[3] as usize]);

    s
}

impl SvgBuilder {
    pub fn new() -> SvgBuilder {
        SvgBuilder {
            background_color: [255; 4],
            dot_color: [0, 0, 0, 255],
            margin: 4,
            shape: Square,
        }
    }

    pub fn margin(&mut self, margin: usize) -> &mut Self {
        self.margin = margin;
        self
    }

    pub fn dot_color(&mut self, dot_color: [u8; 4]) -> &mut Self {
        self.dot_color = dot_color;
        self
    }

    pub fn background_color(&mut self, background_color: [u8; 4]) -> &mut Self {
        self.background_color = background_color;
        self
    }

    pub fn shape(&mut self, shape: SvgShape) -> &mut Self {
        self.shape = shape;
        self
    }

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
