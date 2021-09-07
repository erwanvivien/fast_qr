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

const fn encode_level(quality: crate::vecl::ECL) -> u8 {
    return match quality {
        crate::vecl::ECL::L => 0b01,
        crate::vecl::ECL::M => 0b00,
        crate::vecl::ECL::Q => 0b11,
        crate::vecl::ECL::H => 0b10,
    };
}

const fn encode_mask(mask_nb: u8) -> u8 {
    return match mask_nb {
        0 => 0b000,
        1 => 0b001,
        2 => 0b010,
        3 => 0b011,
        4 => 0b100,
        5 => 0b101,
        6 => 0b110,
        7 => 0b111,
        _ => u8::MAX,
    };
}

pub fn place_on_matrix_formatinfo(mat: &mut Vec<Vec<bool>>, formatinfo: u16) {
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

pub fn place_on_matrix_versioninfo(mat: &mut Vec<Vec<bool>>, version: usize) {
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

pub fn place_on_matrix(
    mat: &mut Vec<Vec<bool>>,
    structure_as_binarystring: String,
    version: usize,
    quality: crate::vecl::ECL,
) {
    let mask_nb = 0;
    let encoded_generator = crate::vecl::ecm_to_format_information(quality, mask_nb);

    println!("{:b}", encoded_generator);

    place_on_matrix_data(mat, structure_as_binarystring, version);
    crate::datamasking::mask(mat, mask_nb as u8);
    place_on_matrix_formatinfo(mat, encoded_generator);
    place_on_matrix_versioninfo(mat, version);
}