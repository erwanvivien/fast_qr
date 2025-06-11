// #[cfg(feature = "image")]
// #[test]
// fn it_can_output_to_bytes_from_image() {
//     use base64::engine::general_purpose;
//     use base64::Engine;

//     use crate::convert::image::ImageBuilder;
//     use crate::{QRBuilder, ECL};

//     // Expected
//     let image_base64 = "iVBORw0KGgoAAAANSUhEUgAAACUAAAAlCAYAAADFniADAAAEVklEQVR4Ae3AA6AkWZbG8f937o3IzKdyS2Oubdu2bdu2bdu2bWmMnpZKr54yMyLu+Xa3anqmhztr1U+2zf8cAMi2+Z8DANk2/3MAINvmfw4AZNs8gCReENtIAsA295OEbSRhGwBJ2EYStpHEC2KbZwJAts0DSMI2z00StpHE/WwDIAnbSMI2AJIAsA2AJGzz3CRhm2cCQLbNA0jCNpK4n20kYRtJ2EYStpGEbSQBYJv7SQLANpKwjSTuZxtJ2OaZAJBt8wCSsI0k7mcbSdgGQBK2kYRtACRhGwBJ2EYStgGQhG0kcT/bSMI2zwSAbJsHkIRtJHE/20jCNs9NEs/NNpK4n20kYRtJ3M82krDNMwEg2+YBJGGb5yYJ20jiudkGQBK2AZCEbe4nCds8N0nY5pkAkG3zAJJ4QWwjCdtIwjaSsI0kbCMJ20jCNpKwjSReENs8EwCybf4VJAFgG0k8N9v8OwAg2+b5kMT9bCMJ20jCNi+IJGwjiefHNgCSuJ9tngkA2Tb/AknYRhLPj20kYRtJ2OaBJPH82EYStnkmAGTbPIAkbCMJANtIAsA2kviX2EYStpHEA9nmhQBAts0DSMI2krCNJABsI4n72UYSALaRhG0kcT/bAEjifraRhG0kYZsHAEC2zXORhG0kYRtJPJBtJGEbAEnY5vmRBIBtJGEbAEnczzbPBIBsm+ciCQDbSALANgCSsM39JGEbSdhGErYBkMQD2UYSALaRBIBtngkA2TYPIAkA20jCNpIAsI0k7mcbSdgGQBIAtpGEbSRhG0kA2OaFAEC2zQNIwjaSuJ9t7icJANsASMI2DyQJANtIwjb3kwSAbSRhmwcAQLbNc5EEgG0kcT/bAEgCwDaSsA2AJABs80CSsM39JHE/2zwAALJtHkAStpHE/WxzP0nYRhK2kQSAbSQBYJv7SeL5sY0kAGzzTADItnkASdgGQBIAtpGEbQAkYRtJ2EYStpHE/WwjCdsASALANpKwzfMBgGybB5CEbSRhGwBJ2EYStpHEA9kGQBL3s40kAGwjCdtIwjYAkrDNAwAg2+YBJGEbSQDYRhIAtpGEbSTxgthGEi+IbSRhm+cCgGybfwVJ2OZ+krCNJGwjifvZRhK2uZ8kAGwDIAnbPBMAsm0eQBIviG3uJwnbSOKBbCOJB7KNJGwjCdu8AADItnkASdjmuUnCNgCSeG62kQSAbSRxP9vcTxIAtnk+AJBt8wCSsI0k7mcbSdhGEgC2AZCEbe4nifvZRhIviG0kYZtnAkC2zQNIwjaSuJ9tJGEbSQDYRhIPZJv7SeKBbCMJ20gCwDbPBQDZNg8gCdtI4n62kYRtJHE/20jCNpKwjST+JbaRBIBtHgAA2TYPIAnbPDdJ2AZAErZ5IEk8kG0eSBK2kQSAbSRhmwcAQLbNA0jiBbHN/STx/NgGQBK2uZ8kbCMJ20jCNs8FANk2/3MAINvmfw4AZNv8zwGAbJv/OQD4R25RML8XljFMAAAAAElFTkSuQmCC";
//     let expected_data_uri = format!("data:image/png;base64,{image_base64}");

//     // Source
//     let qrcode = QRBuilder::new("https://example.com/")
//         .ecl(ECL::H)
//         .build()
//         .unwrap();

//     // As bytes
//     let png_bytes = ImageBuilder::default().to_bytes(&qrcode).unwrap();

//     // As base64
//     let png_base64 = general_purpose::STANDARD.encode(png_bytes);
//     let data_uri = format!("data:image/png;base64,{png_base64}");

//     // Verify
//     assert_eq!(data_uri, expected_data_uri);
// }
#[cfg(feature = "image")]
#[test]
fn it_can_fit_an_image() {
    use crate::convert::image::ImageBuilder;
    use crate::{QRBuilder, ECL};

    // Expected
    let data = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sample.png"));

    // Source
    let qrcode = QRBuilder::new("https://example.com/")
        .ecl(ECL::H)
        .build()
        .unwrap();

    // As bytes
    let png_bytes = ImageBuilder::default()
        .fit_height(1000)
        .fit_width(800)
        .to_bytes(&qrcode)
        .unwrap();

    // Verify
    assert_eq!(
        &png_bytes, data,
        "Fitted image doesn't match expected result"
    );
}
