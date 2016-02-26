const WINDOW_SIZE: usize = 0x1000;
const WINDOW_MASK: usize = WINDOW_SIZE - 1;
const THRESHOLD: usize = 3;
const INPLACE_THRESHOLD: usize = 0xA;
const LOOK_RANGE: usize = 0x200;
const MAX_LEN: usize = 0xF + THRESHOLD;
const MAX_BUFFER: usize = 0x10 + 1;

#[inline]
fn match_current(window: &[u8], pos: usize, max_len: usize, data: &[u8], dpos: usize) -> usize {
    let mut len = 0;
    while dpos + len < data.len() && len < max_len &&
          window[(pos + len) & WINDOW_MASK] == data[dpos + len] && len < MAX_LEN {
        len += 1;
    }
    len
}

#[inline]
fn match_window(window: &[u8], pos: usize, data: &[u8], dpos: usize) -> Option<(usize, usize)> {
    let mut max_pos = 0;
    let mut max_len = 0;
    for i in THRESHOLD..LOOK_RANGE {
        let len = match_current(&window,
                                ((pos as isize - i as isize) & WINDOW_MASK as isize) as usize,
                                i,
                                &data,
                                dpos);
        if len >= INPLACE_THRESHOLD {
            return Some((i, len));
        }
        if len >= THRESHOLD {
            max_pos = i;
            max_len = len;
        }
    }
    if max_len >= THRESHOLD {
        Some((max_pos, max_len))
    } else {
        None
    }
}

/// `lz77_compress` compresses the input bytes. 
///
/// There is no need to set the size of output. This function will adjust
/// as necessary.
///
/// ## Input
///
/// bytes of raw data
///
/// ## Output
///
/// bytes of compressed data
///
pub fn lz77_compress(input: &[u8], output: &mut Vec<u8>) {
    let mut window = [0u8; WINDOW_SIZE];
    let mut current_pos = 0;
    let mut current_window = 0;
    let mut current_buffer: usize;
    let mut flag_byte: u8;
    let mut bit: u8;
    let mut buffer = [0u8; MAX_BUFFER];
    while current_pos < input.len() {
        flag_byte = 0;
        current_buffer = 0;
        for _ in 0..8 {
            if current_pos >= input.len() {
                buffer[current_buffer] = 0;
                window[current_window] = 0;
                current_buffer += 1;
                current_pos += 1;
                current_window += 1;
                bit = 0;
            } else {
                match match_window(&window, current_window, input, current_pos) {
                    Some((pos, len)) if len >= THRESHOLD => {
                        let byte1 = (pos >> 4) as u8;
                        let byte2 = (((pos & 0x0F) << 4) | ((len - THRESHOLD) & 0x0F)) as u8;
                        buffer[current_buffer] = byte1;
                        buffer[current_buffer + 1] = byte2;
                        current_buffer += 2;
                        bit = 0;
                        for _ in 0..len {
                            window[current_window & WINDOW_MASK] = input[current_pos];
                            current_pos += 1;
                            current_window += 1;
                        }
                    }
                    _ => {
                        buffer[current_buffer] = input[current_pos];
                        window[current_window] = input[current_pos];
                        current_pos += 1;
                        current_window += 1;
                        current_buffer += 1;
                        bit = 1;
                    }
                }
            }
            flag_byte = (flag_byte >> 1) | ((bit & 1u8) << 7);
            current_window = current_window & WINDOW_MASK;

            assert!(current_buffer < MAX_BUFFER,
                    format!("current buffer {} > max buffer {}",
                            current_buffer,
                            MAX_BUFFER));
        }
        output.push(flag_byte);
        for i in 0..current_buffer {
            output.push(buffer[i]);
        }
    }
    output.push(0u8);
    output.push(0u8);
    output.push(0u8);
}

/// `lz77_compress_dummy` makes the `input` valid for decompress without really compressing the data.
///
/// Technically speaking, this function only inserts the `raw byte flag` to the input for every 8
/// bytes. There is no need to set the size of output. This function will adjust
/// as necessary.
///
/// ## Input
///
/// bytes of raw data
///
/// ## Output
///
/// bytes of raw data with 0xFF every 8 bytes.
///
pub fn lz77_compress_dummy(input: &[u8], output: &mut Vec<u8>) {
    for i in 0..input.len() / 8 {
        output.push(0xFF);
        for j in 0..8 {
            output.push(input[8 * i + j]);
        }
    }
    if input.len() % 8 == 0 {
        output.extend_from_slice(&[0u8, 0u8, 0u8, 0u8]);
        output.extend_from_slice(&[0u8, 0u8, 0u8, 0u8]);
    } else {
        let extra_bytes = input.len() % 8;
        output.push(0xFFu8 >> (8 - extra_bytes));
        for i in input.len() - extra_bytes..input.len() {
            output.push(input[i]);
        }
        output.push(0u8);
        output.push(0u8);
        output.push(0u8);
        output.push(0u8);
    }
}

/// `lz77_decompress` decompresses a compressed `input` array to raw bytes.
///
/// There is no need to set the size of output. This function will adjust
/// as necessary.
///
/// ## Input
///
/// bytes of compressed data.
///
/// ## Output
///
/// bytes of raw data
///
pub fn lz77_decompress(input: &[u8], output: &mut Vec<u8>) {
    let mut cur_byte = 0;
    let data_size = input.len();
    let mut window = [0u8; WINDOW_SIZE];
    let mut window_cursor = 0;
    while cur_byte < data_size {
        let flag = input[cur_byte];
        cur_byte += 1;

        for i in 0..8 {
            if (flag >> i) & 1u8 == 1u8 {
                output.push(input[cur_byte]);
                window[window_cursor] = input[cur_byte];
                window_cursor = (window_cursor + 1) & WINDOW_MASK;
                cur_byte += 1;
            } else {
                let w = (input[cur_byte] as usize) << 8 | (input[cur_byte + 1] as usize);
                if w == 0 {
                    return;
                }
                cur_byte += 2;
                let mut position =
                    ((window_cursor as isize - (w >> 4) as isize) & WINDOW_MASK as isize) as usize;
                let length = (w & 0x0F) + THRESHOLD;

                for _ in 0..length {
                    let b = window[position & WINDOW_MASK];
                    output.push(b);
                    window[window_cursor] = b;
                    window_cursor = (window_cursor + 1) & WINDOW_MASK;
                    position = position + 1;
                }
            }
        }
    }
}
