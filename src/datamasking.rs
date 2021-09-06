fn mask_0(mat: &mut Vec<Vec<bool>>, mat_full: &Vec<Vec<bool>>) {
    for row in 0..mat.len() {
        for column in 0..mat[0].len() {
            if !mat_full[row][column] && (row + column) % 2 == 0 {
                mat[row][column] = !mat[row][column];
            }
        }
    }
}

fn mask_1(mat: &mut Vec<Vec<bool>>, mat_full: &Vec<Vec<bool>>) {
    for row in 0..mat.len() {
        for column in 0..mat[0].len() {
            if !mat_full[row][column] && (row) % 2 == 0 {
                mat[row][column] = !mat[row][column];
            }
        }
    }
}

fn mask_2(mat: &mut Vec<Vec<bool>>, mat_full: &Vec<Vec<bool>>) {
    for row in 0..mat.len() {
        for column in 0..mat[0].len() {
            if !mat_full[row][column] && (column) % 3 == 0 {
                mat[row][column] = !mat[row][column];
            }
        }
    }
}

fn mask_3(mat: &mut Vec<Vec<bool>>, mat_full: &Vec<Vec<bool>>) {
    for row in 0..mat.len() {
        for column in 0..mat[0].len() {
            if !mat_full[row][column] && (row + column) % 3 == 0 {
                mat[row][column] = !mat[row][column];
            }
        }
    }
}

fn mask_4(mat: &mut Vec<Vec<bool>>, mat_full: &Vec<Vec<bool>>) {
    for row in 0..mat.len() {
        for column in 0..mat[0].len() {
            if !mat_full[row][column] && ((row / 2) + (column / 3)) % 2 == 0 {
                mat[row][column] = !mat[row][column];
            }
        }
    }
}

fn mask_5(mat: &mut Vec<Vec<bool>>, mat_full: &Vec<Vec<bool>>) {
    for row in 0..mat.len() {
        for column in 0..mat[0].len() {
            if !mat_full[row][column] && ((row * column) % 2) + ((row * column) % 3) == 0 {
                mat[row][column] = !mat[row][column];
            }
        }
    }
}

fn mask_6(mat: &mut Vec<Vec<bool>>, mat_full: &Vec<Vec<bool>>) {
    for row in 0..mat.len() {
        for column in 0..mat[0].len() {
            if !mat_full[row][column] && (((row * column) % 2) + ((row * column) % 3)) % 2 == 0 {
                mat[row][column] = !mat[row][column];
            }
        }
    }
}

fn mask_7(mat: &mut Vec<Vec<bool>>, mat_full: &Vec<Vec<bool>>) {
    for row in 0..mat.len() {
        for column in 0..mat[0].len() {
            if !mat_full[row][column] && (((row + column) % 2) + ((row * column) % 3)) % 2 == 0 {
                mat[row][column] = !mat[row][column];
            }
        }
    }
}

const MASKS: [for<'r, 's> fn(&'r mut Vec<Vec<bool>>, &'s Vec<Vec<bool>>); 8] = [
    mask_0, mask_1, mask_2, mask_3, mask_4, mask_5, mask_6, mask_7,
];

pub fn mask(mat: &mut Vec<Vec<bool>>, mask_nb: u8) {
    let version = (mat.len() - 17) / 4;
    let mat_full = crate::default::non_available_matrix_from_version(version);
    MASKS[mask_nb as usize](mat, &mat_full);
}

// 0	(row + column) mod 2 == 0
// 1	(row) mod 2 == 0
// 2	(column) mod 3 == 0
// 3	(row + column) mod 3 == 0
// 4	( floor(row / 2) + floor(column / 3) ) mod 2 == 0
// 5	((row * column) mod 2) + ((row * column) mod 3) == 0
// 6	( ((row * column) mod 2) + ((row * column) mod 3) ) mod 2 == 0
// 7	( ((row + column) mod 2) + ((row * column) mod 3) ) mod 2 == 0
