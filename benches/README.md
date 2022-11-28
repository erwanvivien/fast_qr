# Benchmarking `fast_qr` & `qrcode`

## Benchmark Windows Powershell

| Benchmark    |   Lower   | Estimate  |   Upper   |                          |
| :----------- | :-------: | :-------: | :-------: | ------------------------ |
| V03H/qrcode  | 844.82 us | 852.66 us | 861.60 us |                          |
| V03H/fast_qr | 93.206 us | 94.412 us | 95.800 us | fast_qr is 9.03x faster  |
| V10H/qrcode  | 3.5491 ms | 3.5924 ms | 3.6396 ms |                          |
| V10H/fast_qr | 323.40 us | 328.35 us | 333.86 us | fast_qr is 10.94x faster |
| V40H/qrcode  | 34.524 ms | 34.915 ms | 35.338 ms |                          |
| V40H/fast_qr | 2.8168 ms | 2.8595 ms | 2.9046 ms | fast_qr is 12.21x faster |

- System: Windows
- Machine: AMD64
- Processor: Intel64 Family 6 Model 158 Stepping 13, GenuineIntel
- RAM: 64GB

## Benchmark Windows Subsystem Linux

| Benchmark    |   Lower   | Estimate  |   Upper   |                         |
| :----------- | :-------: | :-------: | :-------: | ----------------------- |
| V03H/qrcode  | 616.82 us | 622.46 us | 626.98 us |                         |
| V03H/fast_qr | 101.38 us | 102.63 us | 103.73 us | fast_qr is 6.07x faster |
| V10H/qrcode  | 2.4195 ms | 2.4480 ms | 2.4748 ms |                         |
| V10H/fast_qr | 356.47 us | 359.39 us | 361.76 us | fast_qr is 6.81x faster |
| V40H/qrcode  | 21.245 ms | 21.491 ms | 21.723 ms |                         |
| V40H/fast_qr | 2.9404 ms | 2.9801 ms | 3.0177 ms | fast_qr is 7.21x faster |

- System: Linux
- Machine: x86_64
- Processor: x86_64
- RAM: 64 GB

## Benchmark Linux

| Benchmark    |   Lower   | Estimate  |   Upper   |                         |
| :----------- | :-------: | :-------: | :-------: | ----------------------- |
| V03H/qrcode  | 524.30 us | 535.02 us | 547.13 us |                         |
| V03H/fast_qr | 82.079 us | 82.189 us | 82.318 us | fast_qr is 6.51x faster |
| V10H/qrcode  | 2.1105 ms | 2.1145 ms | 2.1186 ms |                         |
| V10H/fast_qr | 268.70 us | 269.28 us | 269.85 us | fast_qr is 7.85x faster |
| V40H/qrcode  | 18.000 ms | 18.037 ms | 18.074 ms |                         |
| V40H/fast_qr | 2.4313 ms | 2.4362 ms | 2.4411 ms | fast_qr is 7.40x faster |

- System: Linux
- Machine: x86_64
- Processor:
- RAM: 8GB

---

| Benchmark    |   Lower   | Estimate  |   Upper   |                         |
| :----------- | :-------: | :-------: | :-------: | ----------------------- |
| V03H/qrcode  | 1.0524 ms | 1.0714 ms | 1.0915 ms |                         |
| V03H/fast_qr | 184.70 us | 187.05 us | 189.85 us | fast_qr is 5.73x faster |
| V10H/qrcode  | 3.9165 ms | 3.9448 ms | 3.9761 ms |                         |
| V10H/fast_qr | 579.63 us | 584.54 us | 589.93 us | fast_qr is 6.75x faster |
| V40H/qrcode  | 35.741 ms | 36.093 ms | 36.476 ms |                         |
| V40H/fast_qr | 5.0615 ms | 5.1513 ms | 5.2476 ms | fast_qr is 7.01x faster |

- System: Linux
- Machine: x86_64
- Processor:
- RAM: 5GB

Benchmarking powered by [Criterion.rs](https://github.com/bheisler/criterion.rs). \
Feel free to run some benchmarking yourself!
