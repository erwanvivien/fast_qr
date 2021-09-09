//! Places data on a matrix

use crate::alphanum;
use crate::datamasking;
use crate::default;
use crate::helpers;
use crate::polynomials;
use crate::vecl;

/// Places the data on the matrix
fn place_on_matrix_data(
    mat: &mut Vec<Vec<bool>>,
    structure_as_binarystring: String,
    version: usize,
) {
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
            break;
        }
        if !mat_full[y as usize][x as usize] {
            let c = structure_bytes_tmp.next();
            mat[y as usize][x as usize] = c != Some('0');
        }
        if !mat_full[y as usize][x as usize - 1] {
            let c = structure_bytes_tmp.next();
            mat[y as usize][x as usize - 1] = c != Some('0');
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
    let mut mat = Default::default();
    for i in 0..8 {
        let mask_nb = i;

        mat = default::create_matrix_from_version(version);
        let encoded_generator = vecl::ecm_to_format_information(quality, mask_nb);

        place_on_matrix_data(&mut mat, structure_as_binarystring.clone(), version);
        datamasking::mask(&mut mat, mask_nb as u8);
        place_on_matrix_formatinfo(&mut mat, encoded_generator);
        place_on_matrix_versioninfo(&mut mat, version);

        helpers::print_matrix_with_margin(&mat);
    }
    return mat;
}

pub fn qrcode(content: String, q: Option<vecl::ECL>, v: Option<usize>) -> Vec<Vec<bool>> {
    let version = if v.is_some() { v.unwrap() } else { 1 };
    let quality = if q.is_some() {
        q.unwrap()
    } else {
        vecl::ECL::Q
    };

    let res = alphanum::encode_alphanum(content, version, quality);
    if res.is_none() {
        return Default::default();
    }

    let alnum = res.unwrap();

    let data_codewords = alnum.to_vec();
    let error_codewords = polynomials::GENERATOR_POLYNOMIALS[vecl::ecc_to_ect(quality, version)];

    let structure = polynomials::structure(&data_codewords, &error_codewords, quality, version);
    let structure_as_binarystring = helpers::binary_to_binarystring_version(structure, version);

    let mat = place_on_matrix(structure_as_binarystring, version, quality);
    return mat;
}
