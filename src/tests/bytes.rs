#[cfg(feature = "image")]
#[test]
fn it_can_output_to_bytes_from_image() {
    use base64::engine::general_purpose;
    use base64::Engine;

    use crate::convert::image::ImageBuilder;
    use crate::{QRBuilder, ECL};

    // Expected
    let image_base64 = "iVBORw0KGgoAAAANSUhEUgAAACUAAAAlCAYAAADFniADAAACsklEQVR4Ae2SUW4bQQxD8+5/6NQvAjsaza7rNB9NgRQgJFEUh96U98e/t2/27yfUq3+Q/+NLAbc/qP/3g9KFgzVD9TFSAzuXndW9NTi+FPA2RYrhnncvvIN7nXt1HXDqb0PB+mWaQR3DqprPnZwI3/vOTd45+FKoPJKq6VUfLlWdyGx1Dv46lAawvprGsL6uezmrgKV1Fu6heOfgNlQEqbCOYX88mvkIlG7y0Vth+TqLy1AurtDNoczg9XrlKaevNThCZfGswgqibprKCSid/WdwhIL65JrkMVic/DN4AysM1K28d1CzfTj7jttQHkCZw1ljos4e9seg5uynxln0vbM4Qn2QwxD2B2DNcB3Yx2DX6S1vhf1OLrgM9Xs5DmEZRZNHMqdCBXKOBtb95NQFt6FgGURs1Qz2B2HNasTUyU1EY+27IxTUAxFChfMIzh4Wp0bA8oDaw8lFm7ecxRFKUkCZ2HsENfe+7+SdJ6Du5KOBxXXeXhyhYB1oAjWnt34cPvj0fYbSywXqoHj78HBy7o5Qkh2wPr+GUEbRdK732af23eyjSX0aCipQxLPCvoc1Q4WfAaA0sPaH7+PovZOwi6FmNQ/tG1ybQunUqA2g+My9qoXy6/zll4LdyOMcwYXJDZcb76E8Zx9Nr0coqEfh9doN82hq381ezeScj1CSzwD1i6cmD0Dt+5zeG6i9feedgyMUrKOIUjWBfd85e7VwrXEP6y+gVshbg8tQU6QYTjN5oR5q7wwVavJ9Z+/eOnEbCsrYA49hf1TOnYDawVndT3gLu3/XfCkUPA/h4z4GS+cs3EHxzh2fDgX1CzXVCHZj+PPsndADdr38bSiXHVDHUNUdVED7PGAvMludJ6Bur/aXoaZBZg1gN4N9Vgsr+Jxh37mfOEJNwb+Yf0K9+tW/5Zf6BY5MJL8MrSO7AAAAAElFTkSuQmCC";
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
