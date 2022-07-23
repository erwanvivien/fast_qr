# Benchmarking `fast_qr` & `qrcode`

## Benchmark Windows Powershell

| Benchmark    |   Lower   | Estimate  |   Upper   |                          |
| :----------- | :-------: | :-------: | :-------: | ------------------------ |
| V03H/qrcode  | 1.1845 ms | 1.1906 ms | 1.2008 ms |                          |
| V03H/fast_qr | 82.545 us | 83.127 us | 84.054 us | fast_qr is 14.32x faster |
| V10H/qrcode  | 4.9676 ms | 4.9916 ms | 5.0324 ms |                          |
| V10H/fast_qr | 366.74 us | 368.62 us | 370.83 us | fast_qr is 13.54x faster |
| V40H/qrcode  | 49.067 ms | 49.198 ms | 49.362 ms |                          |
| V40H/fast_qr | 3.3137 ms | 3.3294 ms | 3.3557 ms | fast_qr is 14.78x faster |

- System: Windows
- Machine: AMD64
- Processor: Intel64 Family 6 Model 158 Stepping 13, GenuineIntel
- RAM: 64GB

## Benchmark Windows Subsystem Linux

| Benchmark    |   Lower   | Estimate  |   Upper   |                         |
| :----------- | :-------: | :-------: | :-------: | ----------------------- |
| V03H/qrcode  | 627.31 us | 628.28 us | 629.35 us |                         |
| V03H/fast_qr | 62.610 us | 62.895 us | 63.362 us | fast_qr is 9.99x faster |
| V10H/qrcode  | 2.5220 ms | 2.5253 ms | 2.5288 ms |                         |
| V10H/fast_qr | 276.19 us | 277.93 us | 281.06 us | fast_qr is 9.09x faster |
| V40H/qrcode  | 22.014 ms | 22.102 ms | 22.244 ms |                         |
| V40H/fast_qr | 2.5700 ms | 2.5734 ms | 2.5769 ms | fast_qr is 8.59x faster |

- System: Linux
- Machine: x86_64
- Processor: x86_64
- RAM: 64 GB

## Benchmark Linux

| Benchmark    |   Lower   | Estimate  |   Upper   |                          |
| :----------- | :-------: | :-------: | :-------: | ------------------------ |
| V03H/qrcode  | 471.38 us | 472.47 us | 473.57 us |                          |
| V03H/fast_qr | 46.447 us | 46.573 us | 46.710 us | fast_qr is 10.14x faster |
| V10H/qrcode  | 2.0083 ms | 2.0121 ms | 2.0160 ms |                          |
| V10H/fast_qr | 196.96 us | 197.30 us | 197.62 us | fast_qr is 10.20x faster |
| V40H/qrcode  | 17.316 ms | 17.339 ms | 17.361 ms |                          |
| V40H/fast_qr | 1.9863 ms | 1.9898 ms | 1.9934 ms | fast_qr is 8.71x faster  |

- System: Linux
- Machine: x86_64
- Processor:
- RAM: 8GB

Benchmarking powered by [Criterion.rs](https://github.com/bheisler/criterion.rs). \
Feel free to run some benchmarkings yourself!
