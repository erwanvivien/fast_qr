use parking_lot::const_mutex;
use parking_lot::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

const MAX: usize = 3000;

/// Used in the ring
const LOG: [u8; 256] = [
    1, 2, 4, 8, 16, 32, 64, 128, 29, 58, 116, 232, 205, 135, 19, 38, 76, 152, 45, 90, 180, 117,
    234, 201, 143, 3, 6, 12, 24, 48, 96, 192, 157, 39, 78, 156, 37, 74, 148, 53, 106, 212, 181,
    119, 238, 193, 159, 35, 70, 140, 5, 10, 20, 40, 80, 160, 93, 186, 105, 210, 185, 111, 222, 161,
    95, 190, 97, 194, 153, 47, 94, 188, 101, 202, 137, 15, 30, 60, 120, 240, 253, 231, 211, 187,
    107, 214, 177, 127, 254, 225, 223, 163, 91, 182, 113, 226, 217, 175, 67, 134, 17, 34, 68, 136,
    13, 26, 52, 104, 208, 189, 103, 206, 129, 31, 62, 124, 248, 237, 199, 147, 59, 118, 236, 197,
    151, 51, 102, 204, 133, 23, 46, 92, 184, 109, 218, 169, 79, 158, 33, 66, 132, 21, 42, 84, 168,
    77, 154, 41, 82, 164, 85, 170, 73, 146, 57, 114, 228, 213, 183, 115, 230, 209, 191, 99, 198,
    145, 63, 126, 252, 229, 215, 179, 123, 246, 241, 255, 227, 219, 171, 75, 150, 49, 98, 196, 149,
    55, 110, 220, 165, 87, 174, 65, 130, 25, 50, 100, 200, 141, 7, 14, 28, 56, 112, 224, 221, 167,
    83, 166, 81, 162, 89, 178, 121, 242, 249, 239, 195, 155, 43, 86, 172, 69, 138, 9, 18, 36, 72,
    144, 61, 122, 244, 245, 247, 243, 251, 235, 203, 139, 11, 22, 44, 88, 176, 125, 250, 233, 207,
    131, 27, 54, 108, 216, 173, 71, 142, 1,
];

/// Reverses a ring value
const ANTILOG: [u8; 256] = [
    175, 0, 1, 25, 2, 50, 26, 198, 3, 223, 51, 238, 27, 104, 199, 75, 4, 100, 224, 14, 52, 141,
    239, 129, 28, 193, 105, 248, 200, 8, 76, 113, 5, 138, 101, 47, 225, 36, 15, 33, 53, 147, 142,
    218, 240, 18, 130, 69, 29, 181, 194, 125, 106, 39, 249, 185, 201, 154, 9, 120, 77, 228, 114,
    166, 6, 191, 139, 98, 102, 221, 48, 253, 226, 152, 37, 179, 16, 145, 34, 136, 54, 208, 148,
    206, 143, 150, 219, 189, 241, 210, 19, 92, 131, 56, 70, 64, 30, 66, 182, 163, 195, 72, 126,
    110, 107, 58, 40, 84, 250, 133, 186, 61, 202, 94, 155, 159, 10, 21, 121, 43, 78, 212, 229, 172,
    115, 243, 167, 87, 7, 112, 192, 247, 140, 128, 99, 13, 103, 74, 222, 237, 49, 197, 254, 24,
    227, 165, 153, 119, 38, 184, 180, 124, 17, 68, 146, 217, 35, 32, 137, 46, 55, 63, 209, 91, 149,
    188, 207, 205, 144, 135, 151, 178, 220, 252, 190, 97, 242, 86, 211, 171, 20, 42, 93, 158, 132,
    60, 57, 83, 71, 109, 65, 162, 31, 45, 67, 216, 183, 123, 164, 118, 196, 23, 73, 236, 127, 12,
    111, 246, 108, 161, 59, 82, 41, 157, 85, 170, 251, 96, 134, 177, 187, 204, 62, 90, 203, 89, 95,
    176, 156, 169, 160, 81, 11, 245, 22, 235, 122, 117, 44, 215, 79, 174, 213, 233, 230, 231, 173,
    232, 116, 214, 244, 234, 168, 80, 88, 175,
];

pub fn generator(nb: u16) -> Vec<u8> {
    let nb_usize = nb as usize;
    // Remove all calls greater to 3000
    if nb_usize >= MAX {
        panic!(
            "No QR-Code has more than {} Error Codewords. Current: {}",
            MAX, nb
        );
    }

    // Creates the array of vec (empty)
    const V: Vec<u8> = Vec::new();
    static GENERATED_POLYNOMIALS: Mutex<[Vec<u8>; MAX]> = const_mutex([V; MAX]);
    static LAST_GENERATED: AtomicUsize = AtomicUsize::new(0);

    // Gets the polynomial (vector)
    let mut polys = GENERATED_POLYNOMIALS.lock();
    // Add missing items if first call
    if polys[0].len() == 0 {
        polys[0].push(0);
    }

    // If element exists, return it
    if polys[nb_usize].len() != 0 {
        return polys[nb_usize].clone();
    }

    // Get the last generated polynomial
    let last = LAST_GENERATED.load(Ordering::Relaxed);

    for i in last + 1..=nb_usize {
        polys[i] = polys[i - 1].clone();

        for j in 1..=i - 1 {
            let left = polys[i - 1][j] as usize;
            let right = i - 1 + polys[i - 1][j - 1] as usize;

            polys[i][j] = LOG[left % 255] ^ LOG[right % 255];
            polys[i][j] = ANTILOG[polys[i][j] as usize];
        }

        let last: u16 = (polys[i - 1][i - 1] as usize + i - 1) as u16;
        polys[i].push((last % 255) as u8);
    }

    LAST_GENERATED.store(std::cmp::max(nb_usize, last), Ordering::Relaxed);
    return polys[nb_usize].clone();
}

pub fn generated_to_string(poly: &Vec<u8>) -> String {
    let mut s = String::new();
    let length = poly.len();

    for (i, item) in poly.iter().enumerate() {
        s.push_str(&format!(
            "α{}{}",
            item,
            &match length - i - 1 {
                0 => String::new(),
                1 => String::from("x + "),
                n => format!("x{} + ", n),
            },
        ));
    }

    return s;
}
