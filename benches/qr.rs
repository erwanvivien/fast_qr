use std::time::Duration;

use criterion::*;

use fast_qr::QRBuilder;

fn bench_fastqr_qrcode(c: &mut Criterion) {
    let bytes: &[u8] = b"https://example.com/";

    for (id, fast_qr_version, fast_qr_level, qrocde_version, qrcode_level) in &[
        (
            "V03H",
            fast_qr::Version::V03,
            fast_qr::ECL::H,
            qrcode::Version::Normal(3),
            qrcode::EcLevel::H,
        ),
        (
            "V10H",
            fast_qr::Version::V10,
            fast_qr::ECL::H,
            qrcode::Version::Normal(10),
            qrcode::EcLevel::H,
        ),
        (
            "V40H",
            fast_qr::Version::V40,
            fast_qr::ECL::H,
            qrcode::Version::Normal(40),
            qrcode::EcLevel::H,
        ),
    ] {
        let mut group = c.benchmark_group(*id);
        group.measurement_time(Duration::from_secs(10));
        group.throughput(Throughput::Bytes(bytes.len() as u64));
        group.sample_size(200);

        group.bench_function("qrcode", |b| {
            b.iter(|| {
                qrcode::QrCode::with_version(
                    black_box(b"https://example.com/"),
                    *qrocde_version,
                    *qrcode_level,
                )
                    .unwrap()
            })
        });

        group.bench_function("fast_qr", |b| {
            b.iter(|| {
                QRBuilder::new(black_box("https://example.com/".into()))
                    .ecl(*fast_qr_level)
                    .version(*fast_qr_version)
                    .build()
                    .unwrap()
            })
        });

        group.finish();
    }
}

fn bench_mask(c: &mut Criterion) {
    let mut group = c.benchmark_group("mask");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(200);

    use fast_qr::datamasking::Mask;

    let mut mat = black_box([[fast_qr::module::Module::data(false); 177]; 177]);
    for (mask, id) in [
        (Mask::Checkerboard, "checkerboard"),
        (Mask::HorizontalLines, "horizontal_lines"),
        (Mask::VerticalLines, "vertical_lines"),
        (Mask::DiagonalLines, "diagonal_lines"),
        (Mask::LargeCheckerboard, "large_checkerboard"),
        (Mask::Fields, "fields"),
        (Mask::Diamonds, "diamonds"),
        (Mask::Meadow, "meadow"),
    ] {
        group.bench_function(id, |b| {
            b.iter(|| fast_qr::datamasking::mask(&mut mat, mask))
        });
    }

    group.finish();
}


criterion_group!(benches, bench_fastqr_qrcode);
criterion_main!(benches);
