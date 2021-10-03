//! Places data on a matrix
#![deny(unsafe_code)]
#![warn(missing_docs)]

use crate::datamasking;
use crate::default;
use crate::helpers;
use crate::polynomials;
use crate::score;
use crate::vecl;

/// Places the data on the matrix
fn place_on_matrix_data<const N: usize>(
    mut mat: [[bool; N]; N],
    structure_as_binarystring: String,
    version: usize,
) -> [[bool; N]; N] {
    const SOME_0: Option<char> = Some('0');
    let mat_full: [[bool; N]; N] = default::non_available_matrix_from_version(version);

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
            debug_assert!(structure_bytes_tmp.next() == None);
            break;
        }
        if !mat_full[y as usize][x as usize] {
            let c = structure_bytes_tmp.next();
            mat[y as usize][x as usize] = c != SOME_0;
        }
        if !mat_full[y as usize][x as usize - 1] {
            let c = structure_bytes_tmp.next();
            mat[y as usize][x as usize - 1] = c != SOME_0;
        }

        y += direction as i32;
    }

    mat
}

/// Placement the format information for all QRCodes
fn place_on_matrix_formatinfo<const N: usize>(
    mut mat: [[bool; N]; N],
    formatinfo: u16,
) -> [[bool; N]; N] {
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

    {
        let shift = 1 << 6;
        let value = (formatinfo & shift) != 0;
        // Six on left
        mat[8][7] = value;
        // Six on bottom
        mat[length - 7][7] = value;
    }
    {
        let shift = 1 << 7;
        let value = (formatinfo & shift) != 0;
        // Seven on left
        mat[8][8] = value;
        // Seven on right
        mat[8][length - 8] = value;
    }
    {
        let shift = 1 << 8;
        let value = (formatinfo & shift) != 0;
        // Height on left
        mat[7][8] = value;
        // Height on right
        mat[8][length - 7] = value;
    }

    mat
}

/// Places version information for QRCodes larger and equal to version 7
fn place_on_matrix_versioninfo<const N: usize>(
    mut mat: [[bool; N]; N],
    version: usize,
) -> [[bool; N]; N] {
    if version < 7 {
        return mat;
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

    mat
}

/// Main function to place everything in the QRCode, returns a valid matrix
pub fn place_on_matrix<const N: usize>(
    mat: [[bool; N]; N],
    structure_as_binarystring: String,
    version: usize,
    quality: vecl::ECL,
) -> Vec<Vec<bool>> {
    let mut best_score = u32::MAX;
    let mut best_mask = u8::MAX;

    let mat = default::create_matrix_pattern(mat);
    let mat = default::create_matrix_timing(mat);
    let mat = default::create_matrix_black_module(mat, version);
    let mat = default::create_matrix_alignments(mat, version);
    let mat = place_on_matrix_data(mat, structure_as_binarystring, version);
    let mut mat = place_on_matrix_versioninfo(mat, version);

    // Taken out from mask, that is used 8*2 + 1 times in this function
    let mat_full = default::non_available_matrix_from_version(version);

    let mut mask_nb = 0;

    while mask_nb < 8 {
        mat = datamasking::mask(mat, mask_nb, &mat_full);
        let matrix_score = score::matrix_score(&mat);
        if matrix_score < best_score {
            best_score = matrix_score;
            best_mask = mask_nb;
        }
        mat = datamasking::mask(mat, mask_nb, &mat_full);

        mask_nb += 1;
    }

    let encoded_format_info = vecl::ecm_to_format_information(quality, best_mask as usize);
    mat = place_on_matrix_formatinfo(mat, encoded_format_info);
    mat = datamasking::mask(mat, best_mask, &mat_full);

    let mat = array_to_vec_matrix(mat);

    return mat;
}

/**
 * Takes a string and an optionnal quality and version
 * Tries to figure the QRCode out of those parameters
*/
pub fn qrcode(content: String, q: Option<vecl::ECL>) -> Vec<Vec<bool>> {
    use crate::encode;
    use crate::version::Version;
    let quality = q.unwrap_or(vecl::ECL::Q);

    let mode = encode::best_encoding(content.as_bytes());
    let version = Version::get(mode, quality, content.len()).unwrap() as usize;

    let data_codewords = encode::encode(content.as_bytes(), quality, mode)
        .unwrap()
        .convert();

    let error_codewords = polynomials::GENERATOR_POLYNOMIALS[vecl::ecc_to_ect(quality, version)];

    let structure =
        polynomials::structure(&data_codewords.to_vec(), &error_codewords, quality, version);
    let structure_as_binarystring = helpers::binary_to_binarystring_version(&structure, version);

    match version {
        1 => place_on_matrix(
            [[false; 21]; 21],
            structure_as_binarystring,
            version,
            quality,
        ),
        2 => place_on_matrix(
            [[false; 25]; 25],
            structure_as_binarystring,
            version,
            quality,
        ),
        3 => place_on_matrix(
            [[false; 29]; 29],
            structure_as_binarystring,
            version,
            quality,
        ),
        4 => place_on_matrix(
            [[false; 33]; 33],
            structure_as_binarystring,
            version,
            quality,
        ),
        5 => place_on_matrix(
            [[false; 37]; 37],
            structure_as_binarystring,
            version,
            quality,
        ),
        6 => place_on_matrix(
            [[false; 41]; 41],
            structure_as_binarystring,
            version,
            quality,
        ),
        7 => place_on_matrix(
            [[false; 45]; 45],
            structure_as_binarystring,
            version,
            quality,
        ),
        8 => place_on_matrix(
            [[false; 49]; 49],
            structure_as_binarystring,
            version,
            quality,
        ),
        9 => place_on_matrix(
            [[false; 53]; 53],
            structure_as_binarystring,
            version,
            quality,
        ),
        10 => place_on_matrix(
            [[false; 57]; 57],
            structure_as_binarystring,
            version,
            quality,
        ),
        11 => place_on_matrix(
            [[false; 61]; 61],
            structure_as_binarystring,
            version,
            quality,
        ),
        12 => place_on_matrix(
            [[false; 65]; 65],
            structure_as_binarystring,
            version,
            quality,
        ),
        13 => place_on_matrix(
            [[false; 69]; 69],
            structure_as_binarystring,
            version,
            quality,
        ),
        14 => place_on_matrix(
            [[false; 73]; 73],
            structure_as_binarystring,
            version,
            quality,
        ),
        15 => place_on_matrix(
            [[false; 77]; 77],
            structure_as_binarystring,
            version,
            quality,
        ),
        16 => place_on_matrix(
            [[false; 81]; 81],
            structure_as_binarystring,
            version,
            quality,
        ),
        17 => place_on_matrix(
            [[false; 85]; 85],
            structure_as_binarystring,
            version,
            quality,
        ),
        18 => place_on_matrix(
            [[false; 89]; 89],
            structure_as_binarystring,
            version,
            quality,
        ),
        19 => place_on_matrix(
            [[false; 93]; 93],
            structure_as_binarystring,
            version,
            quality,
        ),
        20 => place_on_matrix(
            [[false; 97]; 97],
            structure_as_binarystring,
            version,
            quality,
        ),
        21 => place_on_matrix(
            [[false; 101]; 101],
            structure_as_binarystring,
            version,
            quality,
        ),
        22 => place_on_matrix(
            [[false; 105]; 105],
            structure_as_binarystring,
            version,
            quality,
        ),
        23 => place_on_matrix(
            [[false; 109]; 109],
            structure_as_binarystring,
            version,
            quality,
        ),
        24 => place_on_matrix(
            [[false; 113]; 113],
            structure_as_binarystring,
            version,
            quality,
        ),
        25 => place_on_matrix(
            [[false; 117]; 117],
            structure_as_binarystring,
            version,
            quality,
        ),
        26 => place_on_matrix(
            [[false; 121]; 121],
            structure_as_binarystring,
            version,
            quality,
        ),
        27 => place_on_matrix(
            [[false; 125]; 125],
            structure_as_binarystring,
            version,
            quality,
        ),
        28 => place_on_matrix(
            [[false; 129]; 129],
            structure_as_binarystring,
            version,
            quality,
        ),
        29 => place_on_matrix(
            [[false; 133]; 133],
            structure_as_binarystring,
            version,
            quality,
        ),
        30 => place_on_matrix(
            [[false; 137]; 137],
            structure_as_binarystring,
            version,
            quality,
        ),
        31 => place_on_matrix(
            [[false; 141]; 141],
            structure_as_binarystring,
            version,
            quality,
        ),
        32 => place_on_matrix(
            [[false; 145]; 145],
            structure_as_binarystring,
            version,
            quality,
        ),
        33 => place_on_matrix(
            [[false; 149]; 149],
            structure_as_binarystring,
            version,
            quality,
        ),
        34 => place_on_matrix(
            [[false; 153]; 153],
            structure_as_binarystring,
            version,
            quality,
        ),
        35 => place_on_matrix(
            [[false; 157]; 157],
            structure_as_binarystring,
            version,
            quality,
        ),
        36 => place_on_matrix(
            [[false; 161]; 161],
            structure_as_binarystring,
            version,
            quality,
        ),
        37 => place_on_matrix(
            [[false; 165]; 165],
            structure_as_binarystring,
            version,
            quality,
        ),
        38 => place_on_matrix(
            [[false; 169]; 169],
            structure_as_binarystring,
            version,
            quality,
        ),
        39 => place_on_matrix(
            [[false; 173]; 173],
            structure_as_binarystring,
            version,
            quality,
        ),
        40 => place_on_matrix(
            [[false; 177]; 177],
            structure_as_binarystring,
            version,
            quality,
        ),
        _ => unreachable!(),
    }
}

fn array_to_vec_matrix<const N: usize>(arrays: [[bool; N]; N]) -> Vec<Vec<bool>> {
    let mut mat = Vec::new();

    for array in arrays {
        mat.push(array.to_vec());
    }

    mat
}
