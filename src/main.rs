#[cfg(test)]
mod tests;

mod bitstack;
mod bitstorage;
mod datamasking;
mod default;
mod encode;
mod encoding;
mod helpers;
mod placement;
mod polynomials;
mod score;
mod vecl;
mod version;

/// Still useless, only test purposes for now.
fn main() {
    let content = String::from("901823093");
    let version = Some(1);
    let quality = None;

    let mat = placement::qrcode(content, quality, version);
    helpers::print_matrix_with_margin(&mat);
}
