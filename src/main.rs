#[cfg(test)]
mod tests;

mod alphanum;
mod bitstorage;
mod datamasking;
mod default;
mod helpers;
mod placement;
mod polynomials;
mod vecl;

/// Still useless, only test purposes for now.
fn main() {
    let content = String::from("HELLO WORLD");
    let version = None;
    let quality = None;

    let mat = placement::qrcode(content, quality, version);
    helpers::print_matrix_with_margin(&mat);
}

// println!("const DEFAULT_MATRIX: [&[u16]; 41] = [\n\t&[],");

// for version in 1..=10 {
//     const QUALITY: vecl::ECL = vecl::ECL::H;
//     const STRING_TO_ENCODE: &[u8] = b"H";
//     let vec = alphanum::encode_alphanum(STRING_TO_ENCODE, version, QUALITY);
//     let data_codewords = helpers::binarystring_to_binary(&vec);
//     let error_codewords =
//         polynomials::GENERATOR_POLYNOMIALS[vecl::ecc_to_ect(QUALITY, version)];

//     let structure =
//         polynomials::structure(&data_codewords, &error_codewords, QUALITY, version);

//     let structure_as_string = helpers::binary_to_binarystring_version(structure, version);

//     let mut mat = default::non_available_matrix_from_version(version, true);

//     print!("\t&[");
//     let max = (16 + version * 4) * (16 + version * 4);
//     let mut direction: i8 = -1;

//     let dimension = version * 4 + 17;
//     let [mut x, mut y]: [i32; 2] = [dimension as i32 - 1, dimension as i32 - 1];

//     let mut i = 0;
//     while i < structure_as_string.len() {
//         if y < 0 {
//             y = 0;
//             direction = 1;
//             x -= 2;
//         }
//         if y >= dimension as i32 {
//             y = dimension as i32 - 1;
//             direction = -1;
//             x -= 2;
//         }
//         if x == 6 {
//             x -= 1;
//         }

//         let tmp = y * dimension as i32 + x;

//         if !mat[y as usize][x as usize] || !mat[y as usize][x as usize - 1] {
//             if !mat[y as usize][x as usize] {
//                 mat[y as usize][x as usize] = true;
//                 i += 1;
//             }
//             if !mat[y as usize][x as usize - 1] {
//                 mat[y as usize][x as usize - 1] = true;
//                 i += 1;
//             }

//             print!("{},", tmp);
//         }

//         y += direction as i32;
//     }
//     println!("],");
// }

// println!("];");
