#[cfg(feature = "image")]
#[test]
fn it_can_output_to_bytes_from_image() {
    use base64::engine::general_purpose;
    use base64::Engine;

    use crate::convert::image::ImageBuilder;
    use crate::{QRBuilder, ECL};

    // Expected
    let image_base64 = "iVBORw0KGgoAAAANSUhEUgAAACUAAAAlCAYAAADFniADAAACxElEQVR4Ae2W224UQQxEc/7/o2HOOrV2z/Qs0fJAkIKo2K4qX6YBCX4dvz6+2a+fo776B/J/vBRw+0H+9YPSzWMEPqyhojxUDh3ld7B38peXghoyTeZQPNRRchkGrU1u55GbgOpduGPIr4X4NMG6HKoZ7qNzjnmGB6BmyEH3PcTjx+SP8vn79qWgBuo8N0MvUNMDxc0c9pweYS+0R068dZSNAfTx4bLsXL/i4zXeHqU4AfVFsD9CL5TnnO9qOQHdYy22RynskC99N+5myjnPGFyOinAXoV7KQVD59MrP+p389ijohS6CemaoeLcMSofun15nWUPr4eTF7VGKAfx5EbTnsuTQMmtGfVB9k78cBWWC+pI02jRz6zvElzh9crPe5S+PcgBcj3PQTptcPEaoGebxzCg/cTlKEfq10iwfhDPKQfnNz4A6SC+0D4rXr2YMbo/SoBmq2VwOevCsoXioGM0YOAN6HnQej/FyFLQRagGsnI0iS4zWsPrk4TpD7ytsj8qwNFonh14sB7XUPID2wKrDvfbsPxYu/0tQgLVRThxewwes+o5/GD9/wP4w5fSaB2+9lIOgFsF6oIPVjQJKN5/QA6WZT+32KE3QTVBHhHcQFAcd1cXUrWE/S+2M26OgFtkAlUNH+cADzKEWm8tB1cln1AM1z3zi5VEaM2jmk5M/46u6vnOv9eUoyVeA9eugauiYfpdC8eGgX08OVv3BHY3Lvz6oJsUzDu+TghoGq18P7LloxuegTXJ5KahlZy80D+tSvS6C4mcezSigPdY73B4F1WxTliSGM0Ifm9oopt/6jOjGqf3VUdCHO3QOh6sG9QFQ2vTbH7x9lAMcCr0otdorTJ/52Xt71MX4uVwe6hDzAOrrU5+XQfVA+dShuPQkbo+KeI4OCgc1PHViPLAuhKphjemb8XLUFP9V/nPUV1/+W77Ub25RML9l49+tAAAAAElFTkSuQmCC";
    let expected_data_uri = format!("data:image/png;base64,{image_base64}");

    // Source
    let qrcode = QRBuilder::new("https://example.com/")
        .ecl(ECL::H)
        .build()
        .unwrap();

    // As bytes
    let png_bytes = ImageBuilder::default().to_bytes(&qrcode).unwrap();

    // As base64
    let png_base64 = general_purpose::STANDARD.encode(png_bytes);
    let data_uri = format!("data:image/png;base64,{png_base64}");

    // Verify
    assert_eq!(data_uri, expected_data_uri);
}
