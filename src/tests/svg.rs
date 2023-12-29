#[cfg(feature = "svg")]
#[test]
fn it_embeds_an_image_via_data_uri() {
    use crate::convert::svg::SvgBuilder;
    use crate::convert::Builder;
    use crate::{QRBuilder, ECL};

    let image_base64 = "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAIAAACQkWg2AAAAFUlEQVR4AWP4oyVDEhrGGkY1jGoAABACQhA+7XDPAAAAAElFTkSuQmCC";
    let data_uri = format!("data:image/png;base64,{image_base64}");

    let qrcode = QRBuilder::new("https://example.com/")
        .ecl(ECL::H)
        .build()
        .unwrap();

    let svg = SvgBuilder::default()
        .image(data_uri.clone())
        .to_str(&qrcode);

    let expected_href = format!(r#"href="{data_uri}""#);
    assert!(svg.contains(&expected_href));
}

#[cfg(feature = "svg")]
#[test]
fn check_svg_is_not_inverted() {
    use crate::convert::svg::SvgBuilder;
    use crate::convert::Builder;
    use crate::{QRBuilder, Version, ECL};

    let qrcode = QRBuilder::new("Test")
        .ecl(ECL::M)
        .version(Version::V01)
        .build()
        .unwrap();

    const MARGIN: usize = 4;
    let svg = SvgBuilder::default().margin(MARGIN).to_str(&qrcode);

    let size = qrcode.size;
    for y in 0..size {
        for x in 0..size {
            let index = y * size + x;
            let expected = qrcode.data[index];
            if expected.value() {
                let expected = format!(r#"M{x},{y}"#, x = x + MARGIN, y = y + MARGIN);
                assert!(svg.contains(&expected));
            }
        }
    }
}
