#[cfg(test)]
mod tests;

mod bitstring;
mod datamasking;
mod default;
mod encode;
mod helpers;
mod placement;
mod polynomials;
mod qrcode;
mod score;
mod vecl;
mod version;

/// Still useless, only test purposes for now.
fn main() {
    let content = String::from("https://vahan.dev/");
    let qrcode = qrcode::QRCode::new(content.as_bytes(), vecl::ECL::H).unwrap();
    qrcode.print();
}
