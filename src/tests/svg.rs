#[cfg(feature = "svg")]
use crate::convert::svg::SvgBuilder;
use crate::convert::Builder;
use crate::{QRBuilder, ECL};
use base64::{engine::general_purpose, Engine as _};

#[cfg(feature = "svg")]
#[test]
fn it_embeds_an_image_via_data_uri() {
    let image_bytes = std::fs::read("assets/red_square.png").unwrap();
    let image_base64 = general_purpose::STANDARD.encode(&image_bytes);
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
