use std::time::Duration;

use criterion::*;
use std::hint::black_box;

use fast_qr::convert::svg::SvgBuilder;
use fast_qr::convert::{Builder, Shape};
use fast_qr::{QRBuilder, Version, ECL};

fn bench_fastqr_svg(c: &mut Criterion) {
    let qrcode = QRBuilder::new("https://example.com/").build().unwrap();

    let mut group = c.benchmark_group("svg");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(200);

    group.bench_function("to_str", |b| {
        b.iter(|| {
            let svg = SvgBuilder::default().shape(Shape::Square).to_str(&qrcode);
            black_box(svg);
        })
    });

    group.finish();
}

fn bench_svg_shapes(c: &mut Criterion) {
    let qrcode = QRBuilder::new("https://example.com/").build().unwrap();

    let mut group = c.benchmark_group("svg_shapes");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(100);

    for shape in &[
        Shape::Square,
        Shape::Circle,
        Shape::RoundedSquare,
        Shape::Vertical,
        Shape::Horizontal,
        Shape::Diamond,
    ] {
        let shape_name: &str = (*shape).into();
        group.bench_function(shape_name, |b| {
            b.iter(|| {
                let svg = SvgBuilder::default().shape(*shape).to_str(&qrcode);
                black_box(svg);
            })
        });
    }

    group.finish();
}

fn bench_svg_versions(c: &mut Criterion) {
    for (version, name) in &[
        (Version::V03, "V03"),
        (Version::V10, "V10"),
        (Version::V20, "V20"),
        (Version::V30, "V30"),
        (Version::V40, "V40"),
    ] {
        let qrcode = QRBuilder::new("https://example.com/")
            .ecl(ECL::H)
            .version(*version)
            .build()
            .unwrap();

        let mut group = c.benchmark_group(format!("svg_versions/{}", name));
        group.measurement_time(Duration::from_secs(5));
        group.sample_size(100);

        group.bench_function("square", |b| {
            b.iter(|| {
                let svg = SvgBuilder::default().shape(Shape::Square).to_str(&qrcode);
                black_box(svg);
            })
        });

        group.bench_function("rounded_square", |b| {
            b.iter(|| {
                let svg = SvgBuilder::default()
                    .shape(Shape::RoundedSquare)
                    .to_str(&qrcode);
                black_box(svg);
            })
        });

        group.finish();
    }
}

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
                QRBuilder::new(black_box("https://example.com/"))
                    .ecl(*fast_qr_level)
                    .version(*fast_qr_version)
                    .build()
                    .unwrap()
            })
        });

        group.finish();
    }
}

criterion_group!(
    benches,
    bench_fastqr_qrcode,
    bench_fastqr_svg,
    bench_svg_shapes,
    bench_svg_versions
);
criterion_main!(benches);
