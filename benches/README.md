# Benchmarking `fast_qr` & `qrcode`

## Benchmark Windows Powershell

| Benchmark                | Lower     | Estimate  | Upper     | Ratio                    |
| :--                      | :--:      | :--:      | :--:      | --                       |
| V03H/qrcode              | 1.1646 ms | 1.1717 ms | 1.1804 ms |                          |
| V03H/fast_qr             | 113.58 µs | 113.95 µs | 114.37 µs | fast_qr is 10.28x faster |
| V10H/qrcode              | 4.8396 ms | 4.8459 ms | 4.8524 ms |                          |
| V10H/fast_qr             | 368.77 µs | 369.98 µs | 371.86 µs | fast_qr is 13.10x faster |
| V40H/qrcode              | 46.726 ms | 46.769 ms | 46.815 ms |                          |
| V40H/fast_qr             | 3.2223 ms | 3.2327 ms | 3.2458 ms | fast_qr is 14.47x faster |
- System: Windows
- Machine: AMD64
- Processor: Intel64 Family 6 Model 158 Stepping 13, GenuineIntel

## Benchmark Windows Subsystem Linux

| Benchmark                | Lower     | Estimate  | Upper     | Ratio                    |
| :--                      | :--:      | :--:      | :--:      | --                       |
| V03H/qrcode              | 623.48 µs | 624.84 µs | 626.68 µs |                          |
| V03H/fast_qr             | 103.42 µs | 104.06 µs | 104.75 µs | fast_qr is 6.00x faster  |
| V10H/qrcode              | 2.5158 ms | 2.5174 ms | 2.5193 ms |                          |
| V10H/fast_qr             | 334.00 µs | 334.33 µs | 334.72 µs | fast_qr is 7.53x faster  |
| V40H/qrcode              | 22.188 ms | 22.221 ms | 22.273 ms |                          |
| V40H/fast_qr             | 2.9143 ms | 2.9166 ms | 2.9190 ms | fast_qr is 7.62x faster  |
- System: Linux
- Machine: x86_64
- Processor: x86_64

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
