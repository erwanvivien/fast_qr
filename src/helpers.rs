//! Matrix helpers functions

/// Used to print a ` `
const EMPTY: &str = " ";
/// Used to print a `█`
const BLOCK: &str = "█";

/// Prints a matrix
pub fn _print_matrix(mat: &Vec<Vec<bool>>) {
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
pub fn print_matrix_with_margin(mat: &Vec<Vec<bool>>) {
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
