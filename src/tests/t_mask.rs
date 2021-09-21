#[test]
fn mask_0_test() {
    let mut mat = Vec::new();
    for _ in 0..10 {
        mat.push(vec![false; 10]);
    }

    let mat_full = vec![vec![false; mat[0].len()]; mat.len()];

    crate::datamasking::mask(&mut mat, 0, &mat_full);
    assert_eq!(
        mat,
        Vec::from([
            Vec::from([true, false, true, false, true, false, true, false, true, false]),
            Vec::from([false, true, false, true, false, true, false, true, false, true]),
            Vec::from([true, false, true, false, true, false, true, false, true, false]),
            Vec::from([false, true, false, true, false, true, false, true, false, true]),
            Vec::from([true, false, true, false, true, false, true, false, true, false]),
            Vec::from([false, true, false, true, false, true, false, true, false, true]),
            Vec::from([true, false, true, false, true, false, true, false, true, false]),
            Vec::from([false, true, false, true, false, true, false, true, false, true]),
            Vec::from([true, false, true, false, true, false, true, false, true, false]),
            Vec::from([false, true, false, true, false, true, false, true, false, true]),
        ])
    )
}
#[test]
fn mask_1_test() {
    let mut mat = Vec::new();
    for _ in 0..10 {
        mat.push(vec![false; 10]);
    }

    let mat_full = vec![vec![false; mat[0].len()]; mat.len()];

    crate::datamasking::mask(&mut mat, 1, &mat_full);
    assert_eq!(
        mat,
        Vec::from([
            Vec::from([true, true, true, true, true, true, true, true, true, true]),
            Vec::from([false, false, false, false, false, false, false, false, false, false]),
            Vec::from([true, true, true, true, true, true, true, true, true, true]),
            Vec::from([false, false, false, false, false, false, false, false, false, false]),
            Vec::from([true, true, true, true, true, true, true, true, true, true]),
            Vec::from([false, false, false, false, false, false, false, false, false, false]),
            Vec::from([true, true, true, true, true, true, true, true, true, true]),
            Vec::from([false, false, false, false, false, false, false, false, false, false]),
            Vec::from([true, true, true, true, true, true, true, true, true, true]),
            Vec::from([false, false, false, false, false, false, false, false, false, false]),
        ])
    )
}
#[test]
fn mask_2_test() {
    let mut mat = Vec::new();
    for _ in 0..10 {
        mat.push(vec![false; 10]);
    }

    let mat_full = vec![vec![false; mat[0].len()]; mat.len()];

    crate::datamasking::mask(&mut mat, 2, &mat_full);

    assert_eq!(
        mat,
        Vec::from([
            Vec::from([true, false, false, true, false, false, true, false, false, true]),
            Vec::from([true, false, false, true, false, false, true, false, false, true]),
            Vec::from([true, false, false, true, false, false, true, false, false, true]),
            Vec::from([true, false, false, true, false, false, true, false, false, true]),
            Vec::from([true, false, false, true, false, false, true, false, false, true]),
            Vec::from([true, false, false, true, false, false, true, false, false, true]),
            Vec::from([true, false, false, true, false, false, true, false, false, true]),
            Vec::from([true, false, false, true, false, false, true, false, false, true]),
            Vec::from([true, false, false, true, false, false, true, false, false, true]),
            Vec::from([true, false, false, true, false, false, true, false, false, true]),
        ])
    )
}
#[test]
fn mask_3_test() {
    let mut mat = Vec::new();
    for _ in 0..10 {
        mat.push(vec![false; 10]);
    }

    let mat_full = vec![vec![false; mat[0].len()]; mat.len()];

    crate::datamasking::mask(&mut mat, 3, &mat_full);

    assert_eq!(
        mat,
        Vec::from([
            Vec::from([true, false, false, true, false, false, true, false, false, true]),
            Vec::from([false, false, true, false, false, true, false, false, true, false]),
            Vec::from([false, true, false, false, true, false, false, true, false, false]),
            Vec::from([true, false, false, true, false, false, true, false, false, true]),
            Vec::from([false, false, true, false, false, true, false, false, true, false]),
            Vec::from([false, true, false, false, true, false, false, true, false, false]),
            Vec::from([true, false, false, true, false, false, true, false, false, true]),
            Vec::from([false, false, true, false, false, true, false, false, true, false]),
            Vec::from([false, true, false, false, true, false, false, true, false, false]),
            Vec::from([true, false, false, true, false, false, true, false, false, true]),
        ])
    )
}
#[test]
fn mask_4_test() {
    let mut mat = Vec::new();
    for _ in 0..10 {
        mat.push(vec![false; 10]);
    }

    let mat_full = vec![vec![false; mat[0].len()]; mat.len()];

    crate::datamasking::mask(&mut mat, 4, &mat_full);

    assert_eq!(
        mat,
        Vec::from([
            Vec::from([true, true, true, false, false, false, true, true, true, false]),
            Vec::from([true, true, true, false, false, false, true, true, true, false]),
            Vec::from([false, false, false, true, true, true, false, false, false, true]),
            Vec::from([false, false, false, true, true, true, false, false, false, true]),
            Vec::from([true, true, true, false, false, false, true, true, true, false]),
            Vec::from([true, true, true, false, false, false, true, true, true, false]),
            Vec::from([false, false, false, true, true, true, false, false, false, true]),
            Vec::from([false, false, false, true, true, true, false, false, false, true]),
            Vec::from([true, true, true, false, false, false, true, true, true, false]),
            Vec::from([true, true, true, false, false, false, true, true, true, false]),
        ])
    )
}
#[test]
fn mask_5_test() {
    let mut mat = Vec::new();
    for _ in 0..10 {
        mat.push(vec![false; 10]);
    }

    let mat_full = vec![vec![false; mat[0].len()]; mat.len()];

    crate::datamasking::mask(&mut mat, 5, &mat_full);

    assert_eq!(
        mat,
        Vec::from([
            Vec::from([true, true, true, true, true, true, true, true, true, true]),
            Vec::from([true, false, false, false, false, false, true, false, false, false]),
            Vec::from([true, false, false, true, false, false, true, false, false, true]),
            Vec::from([true, false, true, false, true, false, true, false, true, false]),
            Vec::from([true, false, false, true, false, false, true, false, false, true]),
            Vec::from([true, false, false, false, false, false, true, false, false, false]),
            Vec::from([true, true, true, true, true, true, true, true, true, true]),
            Vec::from([true, false, false, false, false, false, true, false, false, false]),
            Vec::from([true, false, false, true, false, false, true, false, false, true]),
            Vec::from([true, false, true, false, true, false, true, false, true, false]),
        ])
    )
}
#[test]
fn mask_6_test() {
    let mut mat = Vec::new();
    for _ in 0..10 {
        mat.push(vec![false; 10]);
    }

    let mat_full = vec![vec![false; mat[0].len()]; mat.len()];

    crate::datamasking::mask(&mut mat, 6, &mat_full);

    assert_eq!(
        mat,
        Vec::from([
            Vec::from([true, true, true, true, true, true, true, true, true, true]),
            Vec::from([true, true, true, false, false, false, true, true, true, false]),
            Vec::from([true, true, false, true, true, false, true, true, false, true]),
            Vec::from([true, false, true, false, true, false, true, false, true, false]),
            Vec::from([true, false, true, true, false, true, true, false, true, true]),
            Vec::from([true, false, false, false, true, true, true, false, false, false]),
            Vec::from([true, true, true, true, true, true, true, true, true, true]),
            Vec::from([true, true, true, false, false, false, true, true, true, false]),
            Vec::from([true, true, false, true, true, false, true, true, false, true]),
            Vec::from([true, false, true, false, true, false, true, false, true, false])
        ])
    )
}
#[test]
fn mask_7_test() {
    let mut mat = Vec::new();
    for _ in 0..10 {
        mat.push(vec![false; 10]);
    }

    let mat_full = vec![vec![false; mat[0].len()]; mat.len()];

    crate::datamasking::mask(&mut mat, 7, &mat_full);

    assert_eq!(
        mat,
        Vec::from([
            Vec::from([true, false, true, false, true, false, true, false, true, false]),
            Vec::from([false, false, false, true, true, true, false, false, false, true]),
            Vec::from([true, false, false, false, true, true, true, false, false, false]),
            Vec::from([false, true, false, true, false, true, false, true, false, true]),
            Vec::from([true, true, true, false, false, false, true, true, true, false]),
            Vec::from([false, true, true, true, false, false, false, true, true, true]),
            Vec::from([true, false, true, false, true, false, true, false, true, false]),
            Vec::from([false, false, false, true, true, true, false, false, false, true]),
            Vec::from([true, false, false, false, true, true, true, false, false, false]),
            Vec::from([false, true, false, true, false, true, false, true, false, true]),
        ])
    )
}
