//! Places data on a matrix

use crate::bitstorage;
use crate::datamasking;
use crate::default;
use crate::encoding::alphanum;
use crate::encoding::byte;
use crate::encoding::numeric;
use crate::helpers;
use crate::polynomials;
use crate::score;
use crate::vecl;

/// Places the data on the matrix
fn place_on_matrix_data(
    mat: &mut Vec<Vec<bool>>,
    structure_as_binarystring: String,
    version: usize,
) {
    const some_0: Option<char> = Some('0');
    let mat_full = default::non_available_matrix_from_version(version);

    let mut direction: i8 = -1;

    let dimension = version * 4 + 17;
    let [mut x, mut y]: [i32; 2] = [dimension as i32 - 1, dimension as i32 - 1];

    let mut structure_bytes_tmp = structure_as_binarystring.chars();

    loop {
        if y < 0 {
            y = 0;
            direction = 1;
            x -= 2;
        }
        if y >= dimension as i32 {
            y = dimension as i32 - 1;
            direction = -1;
            x -= 2;
        }
        if x == 6 {
            x -= 1;
        }

        if x < 0 {
            // DBG: Only in dev mode
            assert_eq!(structure_bytes_tmp.next(), None);
            break;
        }
        if !mat_full[y as usize][x as usize] {
            let c = structure_bytes_tmp.next();
            mat[y as usize][x as usize] = c != some_0;
        }
        if !mat_full[y as usize][x as usize - 1] {
            let c = structure_bytes_tmp.next();
            mat[y as usize][x as usize - 1] = c != some_0;
        }

        y += direction as i32;
    }
}

/// Placement the format information for all QRCodes
fn place_on_matrix_formatinfo(mat: &mut Vec<Vec<bool>>, formatinfo: u16) {
    let length = mat.len();

    for i in (0..=5).rev() {
        let shift = 1 << (i + 9);
        let value = (formatinfo & shift) != 0;
        mat[8][i] = value;
        mat[length - 6 + i][8] = value;
    }

    for i in 0..=5 {
        let shift = 1 << i;
        let value = (formatinfo & shift) != 0;
        mat[i][8] = value;
        mat[8][length - i - 1] = value;
    }

    let mut shift;
    let mut value;

    shift = 1 << 6;
    value = (formatinfo & shift) != 0;
    // Six on left
    mat[8][7] = value;
    // Six on bottom
    mat[length - 7][7] = value;

    shift = 1 << 7;
    value = (formatinfo & shift) != 0;
    // Seven on left
    mat[8][8] = value;
    // Seven on right
    mat[8][length - 8] = value;

    shift = 1 << 8;
    value = (formatinfo & shift) != 0;
    // Height on left
    mat[7][8] = value;
    // Height on right
    mat[8][length - 7] = value;
}

/// Places version information for QRCodes larger and equal to version 7
fn place_on_matrix_versioninfo(mat: &mut Vec<Vec<bool>>, version: usize) {
    if version < 7 {
        return;
    }

    let length = mat.len();

    let version_info = vecl::VERSION_INFORMATION[version];
    for i in 0..=2 {
        for j in 0..=5 {
            let shift: u32 = 1 << (j * 3 + i);
            mat[j][length - 11 + i] = (version_info & shift) != 0;
            mat[length - 11 + i][j] = (version_info & shift) != 0;
        }
    }
}

/// Main function to place everything in the QRCode, returns a valid matrix
pub fn place_on_matrix(
    structure_as_binarystring: String,
    version: usize,
    quality: vecl::ECL,
) -> Vec<Vec<bool>> {
    let mut best_score = u32::MAX;
    let mut best_mask = u8::MAX;

    let mut mat = default::create_matrix_from_version(version);
    place_on_matrix_data(&mut mat, structure_as_binarystring, version);
    place_on_matrix_versioninfo(&mut mat, version);

    for mask_nb in 0..8 {
        datamasking::mask(&mut mat, mask_nb as u8);
        let matrix_score = score::matrix_score(&mat);
        if matrix_score < best_score {
            best_score = matrix_score;
            best_mask = mask_nb as u8;
        }
        datamasking::mask(&mut mat, mask_nb as u8);
    }

    let encoded_format_info = vecl::ecm_to_format_information(quality, best_mask as usize);
    place_on_matrix_formatinfo(&mut mat, encoded_format_info);
    datamasking::mask(&mut mat, best_mask as u8);

    println!("mask:{}\n", best_mask);
    return mat;
}

pub fn qrcode(content: String, q: Option<vecl::ECL>, v: Option<usize>) -> Vec<Vec<bool>> {
    let version = v.unwrap_or(1);
    let quality = q.unwrap_or(vecl::ECL::Q);

    print!("V{}:{} | ", version, quality);

    const POSSIBLE_ENCODINGS: [(
        fn(&String, usize, vecl::ECL) -> Option<bitstorage::BitStorage>,
        &str,
    ); 3] = [
        (numeric::encode, "numeric"),
        (alphanum::encode, "alphanum"),
        (byte::encode, "byte"),
    ];

    let mut res: Option<bitstorage::BitStorage> = None;
    for f in POSSIBLE_ENCODINGS {
        res = f.0(&content, version, quality);
        if res.is_some() {
            print!("encoding:{} | ", f.1);
            break;
        }
    }
    if res.is_none() {
        return Default::default();
    }

    let alnum = res.unwrap();

    let data_codewords = alnum.to_vec();
    let error_codewords = polynomials::GENERATOR_POLYNOMIALS[vecl::ecc_to_ect(quality, version)];

    let structure = polynomials::structure(&data_codewords, &error_codewords, quality, version);
    let structure_as_binarystring = helpers::binary_to_binarystring_version(&structure, version);

    let mat = place_on_matrix(structure_as_binarystring, version, quality);
    return mat;
}
