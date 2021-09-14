#[cfg(test)]
mod tests;

mod bitstorage;
mod datamasking;
mod default;
mod encoding;
mod helpers;
mod placement;
mod polynomials;
mod score;
mod vecl;

/// Still useless, only test purposes for now.
fn main() {
    let content = String::from("salut");
    let version = Some(1);
    let quality = None;

    let mat = placement::qrcode(content, quality, version);
    helpers::print_matrix_with_margin(&mat);
}
