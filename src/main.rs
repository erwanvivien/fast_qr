#[cfg(test)]
mod tests;

mod bitstring;
mod datamasking;
mod default;
mod encode;
mod hardcode;
mod helpers;
mod placement;
mod polynomials;
mod qrcode;
mod score;
mod vecl;
mod version;

/// Still useless, only test purposes for now.
fn main() {
    const CONTENT: &str = "https://vahan.dev/this_url_doesnt_exist";
    const QRCODE: Option<qrcode::QRCode> =
        qrcode::QRCode::new(CONTENT.as_bytes(), vecl::ECL::H, None);

    if let Some(q) = QRCODE {
        q.print();
    }
}
