use crate::QRCode;

const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
const fn make_crc32_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut i = 0;

    while i < 256 {
        let mut crc = i as u32;
        let mut j = 0;

        while j < 8 {
            if (crc & 1) != 0 {
                crc = (crc >> 1) ^ 0xEDB8_8320;
            } else {
                crc >>= 1;
            }
            j += 1;
        }

        table[i] = crc;
        i += 1;
    }

    table
}

const CRC32_TABLE: [u32; 256] = make_crc32_table();

fn crc32_update(mut crc: u32, bytes: &[u8]) -> u32 {
    for &byte in bytes {
        let idx = ((crc ^ u32::from(byte)) & 0xFF) as usize;
        crc = (crc >> 8) ^ CRC32_TABLE[idx];
    }
    crc
}

fn adler32(bytes: &[u8]) -> u32 {
    const MOD_ADLER: u32 = 65_521;
    let mut a = 1u32;
    let mut b = 0u32;

    for chunk in bytes.chunks(5_552) {
        for &byte in chunk {
            a = (a + u32::from(byte)) % MOD_ADLER;
            b = (b + a) % MOD_ADLER;
        }
    }

    (b << 16) | a
}

fn append_u32_be(output: &mut Vec<u8>, value: u32) {
    output.extend_from_slice(&value.to_be_bytes());
}

fn append_png_chunk(output: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    append_u32_be(output, data.len() as u32);
    output.extend_from_slice(kind);
    output.extend_from_slice(data);

    let mut crc = 0xFFFF_FFFF;
    crc = crc32_update(crc, kind);
    crc = crc32_update(crc, data);
    append_u32_be(output, !crc);
}

fn png_from_idat(size: u32, bit_depth: u8, color_type: u8, compressed: &[u8]) -> Vec<u8> {
    let mut ihdr = [0u8; 13];
    ihdr[..4].copy_from_slice(&size.to_be_bytes());
    ihdr[4..8].copy_from_slice(&size.to_be_bytes());
    ihdr[8] = bit_depth;
    ihdr[9] = color_type;

    let mut png = Vec::with_capacity(PNG_SIGNATURE.len() + 25 + compressed.len() + 12);
    png.extend_from_slice(&PNG_SIGNATURE);
    append_png_chunk(&mut png, b"IHDR", &ihdr);
    append_png_chunk(&mut png, b"IDAT", compressed);
    append_png_chunk(&mut png, b"IEND", &[]);
    png
}

fn zlib_stored_blocks(data: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(data.len() + (data.len() / 65_535 + 1) * 5 + 6);
    output.extend_from_slice(&[0x78, 0x01]);

    let chunks = data.len().div_ceil(65_535);
    for (index, chunk) in data.chunks(65_535).enumerate() {
        let is_final = index + 1 == chunks;
        output.push(u8::from(is_final));

        let len = chunk.len() as u16;
        let nlen = !len;
        output.extend_from_slice(&len.to_le_bytes());
        output.extend_from_slice(&nlen.to_le_bytes());
        output.extend_from_slice(chunk);
    }

    append_u32_be(&mut output, adler32(data));
    output
}

fn blit_packed_bits(dst: &mut [u8], bit_offset: usize, src: &[u8], bit_len: usize) {
    if bit_len == 0 {
        return;
    }

    let byte_offset = bit_offset / 8;
    let shift = bit_offset % 8;
    let full_bytes = bit_len / 8;
    let tail_bits = bit_len % 8;

    if shift == 0 {
        if full_bytes != 0 {
            dst[byte_offset..byte_offset + full_bytes].copy_from_slice(&src[..full_bytes]);
        }

        if tail_bits != 0 {
            let mask = 0xFF << (8 - tail_bits);
            let idx = byte_offset + full_bytes;
            dst[idx] &= !mask;
            dst[idx] |= src[full_bytes] & mask;
        }

        return;
    }

    let mut out = byte_offset;

    for &byte in &src[..full_bytes] {
        dst[out] &= byte >> shift;
        if out + 1 < dst.len() {
            dst[out + 1] &= byte << (8 - shift);
        }
        out += 1;
    }

    if tail_bits != 0 {
        let byte = src[full_bytes];
        let mask = 0xFF << (8 - tail_bits);

        dst[out] &= (byte & mask) >> shift;

        let used_in_next = tail_bits.saturating_sub(8 - shift);
        if used_in_next != 0 && out + 1 < dst.len() {
            let next_mask = 0xFF << (8 - used_in_next);
            dst[out + 1] &= (byte << (8 - shift)) | !next_mask;
        }
    }
}

fn pack_qr_row_scalar<M>(qr_row: &[M], out: &mut [u8], value: impl Fn(&M) -> bool) {
    let mut x = 0usize;
    let mut i = 0usize;

    while x + 8 <= qr_row.len() {
        let mut byte = 0xFFu8;

        if value(&qr_row[x]) {
            byte &= !(1 << 7);
        }
        if value(&qr_row[x + 1]) {
            byte &= !(1 << 6);
        }
        if value(&qr_row[x + 2]) {
            byte &= !(1 << 5);
        }
        if value(&qr_row[x + 3]) {
            byte &= !(1 << 4);
        }
        if value(&qr_row[x + 4]) {
            byte &= !(1 << 3);
        }
        if value(&qr_row[x + 5]) {
            byte &= !(1 << 2);
        }
        if value(&qr_row[x + 6]) {
            byte &= !(1 << 1);
        }
        if value(&qr_row[x + 7]) {
            byte &= !(1 << 0);
        }

        out[i] = byte;
        x += 8;
        i += 1;
    }

    if x < qr_row.len() {
        let mut byte = 0xFFu8;

        for (bit, module) in qr_row[x..].iter().enumerate() {
            if value(module) {
                byte &= !(1 << (7 - bit));
            }
        }

        out[i] = byte;
    }
}

fn encode_qr_to_smol_png_1bit(qr: &QRCode, quiet_zone: u32) -> Vec<u8> {
    let module_count = qr.size as usize;
    let quiet = quiet_zone as usize;
    let size = module_count + quiet * 2;
    let packed_row_len = size.div_ceil(8);
    let row_len = 1 + packed_row_len;

    let mut raw = vec![0xFFu8; row_len * size];
    for y in 0..size {
        raw[y * row_len] = 0;
    }

    let qr_packed_len = module_count.div_ceil(8);
    let mut packed_row = vec![0xFFu8; qr_packed_len];

    for my in 0..module_count {
        packed_row.fill(0xFF);

        let qr_row = &qr.data[my * module_count..(my + 1) * module_count];
        pack_qr_row_scalar(qr_row, &mut packed_row, |m| m.value());

        let out_row = (my + quiet) * row_len;
        let dst = &mut raw[out_row + 1..out_row + row_len];
        if quiet % 8 == 0 {
            let byte_offset = quiet / 8;
            dst[byte_offset..byte_offset + qr_packed_len].copy_from_slice(&packed_row);
        } else {
            blit_packed_bits(dst, quiet, &packed_row, module_count);
        }
    }

    let compressed = zlib_stored_blocks(&raw);
    png_from_idat(size as u32, 1, 0, &compressed)
}

pub fn encode(qr: &QRCode, scale: u32, quiet_zone: u32) -> Vec<u8> {
    if scale == 0 {
        return Vec::new();
    }

    if scale == 1 {
        return encode_qr_to_smol_png_1bit(qr, quiet_zone);
    }

    let module_count = qr.size as u32;
    let image_module_count = module_count + quiet_zone * 2;
    let size = image_module_count * scale;
    let row_length = 1 + size as usize;
    let mut raw = vec![0u8; row_length * size as usize];

    for y in 0..size as usize {
        let row_offset = y * row_length;
        let module_y = y as u32 / scale;

        for x in 0..size as usize {
            let module_x = x as u32 / scale;
            let in_bounds = module_x >= quiet_zone
                && module_x < quiet_zone + module_count
                && module_y >= quiet_zone
                && module_y < quiet_zone + module_count;

            let is_dark = in_bounds
                && qr.data
                    [((module_y - quiet_zone) * module_count + (module_x - quiet_zone)) as usize]
                    .value();

            raw[row_offset + 1 + x] = if is_dark { 0 } else { 255 };
        }
    }

    let compressed = zlib_stored_blocks(&raw);
    png_from_idat(size, 8, 0, &compressed)
}
