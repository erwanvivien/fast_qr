//! Places data on a matrix

/// Places the data on the matrix
fn place_on_matrix_data(
    mat: &mut Vec<Vec<bool>>,
    structure_as_binarystring: String,
    version: usize,
) {
    let mat_full = crate::default::non_available_matrix_from_version(version);

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

    let version_info = crate::vecl::VERSION_INFORMATION[version];
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
    quality: crate::vecl::ECL,
) -> Vec<Vec<bool>> {
    let mut mat = Default::default();
    for i in 0..8 {
        let mask_nb = i;

        mat = crate::default::create_matrix_from_version(version);
        let encoded_generator = crate::vecl::ecm_to_format_information(quality, mask_nb);

        place_on_matrix_data(&mut mat, structure_as_binarystring.clone(), version);
        crate::datamasking::mask(&mut mat, mask_nb as u8);
        place_on_matrix_formatinfo(&mut mat, encoded_generator);
        place_on_matrix_versioninfo(&mut mat, version);

        crate::helpers::print_matrix_with_margin(&mat);
    }
    return mat;
}

pub fn qrcode(
    content: String,
    quality: Option<crate::vecl::ECL>,
    version: Option<usize>,
) -> Vec<Vec<bool>> {
    const VERSION: usize = 1;
    const QUALITY: crate::vecl::ECL = crate::vecl::ECL::Q;
    const STRING_TO_ENCODE: &[u8] = b"HELLO WORLD";

    let res = crate::alphanum::encode_alphanum(STRING_TO_ENCODE, VERSION, QUALITY);
    let data_codewords = crate::helpers::binarystring_to_binary(&res);
    let error_codewords =
        crate::polynomials::GENERATOR_POLYNOMIALS[crate::vecl::ecc_to_ect(QUALITY, VERSION)];

    let structure =
        crate::polynomials::structure(&data_codewords, &error_codewords, QUALITY, VERSION);
    let structure_as_binarystring =
        crate::helpers::binary_to_binarystring_version(structure, VERSION);

    let mat = crate::placement::place_on_matrix(structure_as_binarystring, VERSION, QUALITY);
    return mat;
}
