#[cfg(test)]
mod tests;

/// Contains how to encode ALNUM data
mod alphanum;
/// Contains how Error Correction Level (ECC) works
mod ecc;

mod generator_polynomial;

use ecc::ECC;

/// Used to print a ` `
const EMPTY: &str = " ";
/// Used to print a `█`
const BLOCK: &str = "█";

/// Size of FIP (Finder Patterns)
const POSITION_SIZE: usize = 7;

/// For each version, it's the information string we need to add
const VERSION_INFORMATION_STRING_ARRAY: [u32; 41] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0b000111110010010100,
    0b001000010110111100,
    0b001001101010011001,
    0b001010010011010011,
    0b001011101111110110,
    0b001100011101100010,
    0b001101100001000111,
    0b001110011000001101,
    0b001111100100101000,
    0b010000101101111000,
    0b010001010001011101,
    0b010010101000010111,
    0b010011010100110010,
    0b010100100110100110,
    0b010101011010000011,
    0b010110100011001001,
    0b010111011111101100,
    0b011000111011000100,
    0b011001000111100001,
    0b011010111110101011,
    0b011011000010001110,
    0b011100110000011010,
    0b011101001100111111,
    0b011110110101110101,
    0b011111001001010000,
    0b100000100111010101,
    0b100001011011110000,
    0b100010100010111010,
    0b100011011110011111,
    0b100100101100001011,
    0b100101010000101110,
    0b100110101001100100,
    0b100111010101000001,
    0b101000110001101001,
];

/// For each version, it's where the alignments are placed
const ALIGNMENT_PATTERNS_GRID: [&'static [u8]; 41] = [
    &[],
    &[],
    &[6, 18],
    &[6, 22],
    &[6, 26],
    &[6, 30],
    &[6, 34],
    &[6, 22, 38],
    &[6, 24, 42],
    &[6, 26, 46],
    &[6, 28, 50],
    &[6, 30, 54],
    &[6, 32, 58],
    &[6, 34, 62],
    &[6, 26, 46, 66],
    &[6, 26, 48, 70],
    &[6, 26, 50, 74],
    &[6, 30, 54, 78],
    &[6, 30, 56, 82],
    &[6, 30, 58, 86],
    &[6, 34, 62, 90],
    &[6, 28, 50, 72, 94],
    &[6, 26, 50, 74, 98],
    &[6, 30, 54, 78, 102],
    &[6, 28, 54, 80, 106],
    &[6, 32, 58, 84, 110],
    &[6, 30, 58, 86, 114],
    &[6, 34, 62, 90, 118],
    &[6, 26, 50, 74, 98, 122],
    &[6, 30, 54, 78, 102, 126],
    &[6, 26, 52, 78, 104, 130],
    &[6, 30, 56, 82, 108, 134],
    &[6, 34, 60, 86, 112, 138],
    &[6, 30, 58, 86, 114, 142],
    &[6, 34, 62, 90, 118, 146],
    &[6, 30, 54, 78, 102, 126, 150],
    &[6, 24, 50, 76, 102, 128, 154],
    &[6, 28, 54, 80, 106, 132, 158],
    &[6, 32, 58, 84, 110, 136, 162],
    &[6, 26, 54, 82, 110, 138, 166],
    &[6, 30, 58, 86, 114, 142, 170],
];

/** Ranges from 1-9 then 10-26 then 27-40 (included)
 * Numeric mode: 10 bits
 * Alphanumeric mode: 9 bits
 * Byte mode: 8 bits
 * Japanese mode: 8 bits
**/
const _CHARACTER_COUNT_INDICATOR_SIZE: [[u8; 4]; 3] =
    [[10, 9, 8, 8], [12, 11, 16, 10], [14, 13, 16, 12]];

/// Adds the 3 needed squares
pub fn create_matrix_pattern(mat: &mut Vec<Vec<bool>>) {
    let length = mat.len();
    let offsets = [
        (0, 0),
        (length - POSITION_SIZE, 0),
        (0, length - POSITION_SIZE),
    ];

    // Required pattern (4.1 Positions)
    for off in offsets {
        let (y, x) = off;

        // Border
        for i in 0..=6 {
            mat[0 + y][i + x] = true;
            mat[6 + y][i + x] = true;

            mat[i + y][0 + x] = true;
            mat[i + y][6 + x] = true;
        }
        // Inside
        for i in 2..=4 {
            mat[i + y][2 + x] = true;
            mat[i + y][3 + x] = true;
            mat[i + y][4 + x] = true;
        }
    }
}

/// Adds the two lines of Timing patterns
fn create_matrix_timing(mat: &mut Vec<Vec<bool>>) {
    let length = mat.len();
    // Required pattern (4.3 Timing)
    for i in (POSITION_SIZE + 1..length - POSITION_SIZE).step_by(2) {
        mat[POSITION_SIZE - 1][i] = true;
        mat[i][POSITION_SIZE - 1] = true;
    }
}

/// Adds the forever present pixel
fn create_matrix_black_module(mat: &mut Vec<Vec<bool>>, version: usize) {
    // https://www.thonky.com/qr-code-tutorial/format-version-information
    // Dark module
    mat[4 * version + 9][8] = true;
}

/// Adds the smaller squares if needed
fn create_matrix_alignments(mat: &mut Vec<Vec<bool>>, version: usize) {
    let alignment_patterns = ALIGNMENT_PATTERNS_GRID[version];
    // Alignments (smaller cubes)
    if version == 1 {
        return;
    }

    let max = alignment_patterns.len() - 1;

    for (i, alignment_y) in alignment_patterns.iter().map(|x| *x as usize).enumerate() {
        for (j, alignment_x) in alignment_patterns.iter().map(|x| *x as usize).enumerate() {
            // Removes top-left, bottom-left and top-right
            if (i == 0 && j == 0) || (i == max && j == 0) || (i == 0 && j == max) {
                continue;
            }

            // Center
            mat[alignment_y][alignment_x] = true;

            // Borders
            for x in -2..=2i16 {
                for y in -2..=2i16 {
                    if x != -2 && x != 2 && y != -2 && y != 2 {
                        continue;
                    }

                    mat[(alignment_y as i16 + y) as usize][(alignment_x as i16 + x) as usize] =
                        true;
                }
            }
        }
    }
}

/// Adds all the required patterns of a specific version
pub fn create_matrix_from_version(version: usize) -> Vec<Vec<bool>> {
    // https://en.wikipedia.org/wiki/QR_code#Standards
    let length = 17 + version * 4;
    let mut mat = vec![vec![false; length]; length];

    let _version_information_string = VERSION_INFORMATION_STRING_ARRAY[version];

    create_matrix_pattern(&mut mat);
    create_matrix_timing(&mut mat);
    create_matrix_black_module(&mut mat, version);
    create_matrix_alignments(&mut mat, version);

    return mat;
}

/// Prints a matrix
fn _print_matrix(mat: &Vec<Vec<bool>>) {
    for line in mat {
        for &cell in line {
            if cell == true {
                print!("{0}{0}", BLOCK);
            } else {
                print!("{0}{0}", EMPTY);
            }
        }
        println!();
    }
}

/// Prints a matrix with margins
fn print_matrix_with_margin(mat: &Vec<Vec<bool>>) {
    for _ in 0..2 {
        println!();
    }

    for line in mat {
        print!(">  ");
        for &cell in line {
            if cell == true {
                print!("{0}{0}", BLOCK);
            } else {
                print!("{0}{0}", EMPTY);
            }
        }
        println!("  <");
    }

    for _ in 0..2 {
        println!();
    }
}

fn main() {
    let version = 1;
    let mat = create_matrix_from_version(version);

    let string_to_encode = b"HELLO WORLD";
    // print_matrix(&mat);
    print_matrix_with_margin(&mat);

    let res = alphanum::encode_alphanum(string_to_encode, version, ECC::Q);
    println!("{}", res);
}
