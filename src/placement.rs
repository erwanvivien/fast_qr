pub fn place_on_matrix(
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
            mat[y as usize][x as usize] = structure_bytes_tmp.next() == Some('0');
        }
        if !mat_full[y as usize][x as usize - 1] {
            mat[y as usize][x as usize - 1] = structure_bytes_tmp.next() == Some('0');
        }

        y += direction as i32;
    }
}
