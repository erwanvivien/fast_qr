#[cfg(test)]
mod tests;

mod bitstring;
mod datamasking;
mod default;
mod ecl;
mod encode;
mod hardcode;
mod helpers;
mod placement;
mod polynomials;
mod qrcode;
mod score;
mod version;

/// Still useless, only test purposes for now.
fn main() {
    const CONTENT: &str = "https://vahan.dev/";
    const MASK: Option<usize> = None;
    const VERSION: Option<version::Version> = None;
    const LEVEL: Option<ecl::ECL> = Some(ecl::ECL::H);

    const QRCODE: Option<qrcode::QRCode> =
        qrcode::QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, MASK);

    if let Some(q) = QRCODE {
        q.print();
    }
}
