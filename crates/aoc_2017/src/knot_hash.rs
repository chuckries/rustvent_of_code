const SALT: [u8; 5] = [17, 31, 73, 47, 23];

pub fn knot_hash(s: &str) -> [u8; 16] {
    let mut lengths: Vec<u8> = Vec::with_capacity(s.len() + SALT.len());
    lengths.extend_from_slice(s.as_bytes());
    lengths.extend_from_slice(&SALT);
    let lengths = lengths;

    let mut position = 0;
    let mut skip_size = 0;
    let mut list = [0; 256];
    for (idx, i) in list.iter_mut().enumerate() {
        *i = idx as u8;
    }

    for _ in 0..64 {
        knot_hash_round(&mut list, &lengths, &mut position, &mut skip_size);
    }

    let mut dense = [0; 16];
    let mut sparse_idx = 0;
    for i in 0..16 {
        for _ in 0..16 {
            dense[i] ^= list[sparse_idx];
            sparse_idx += 1;
        }
    }

    dense
}

pub fn knot_hash_round(list: &mut [u8], lengths: &[u8], position: &mut usize, skip_size: &mut usize) {
    for l in lengths {
        // reverse
        {
            let mut start_idx = *position;
            let mut end_idx = (start_idx + *l as usize - 1) % 256;

            for _ in 0..l / 2 {
                list.swap(start_idx, end_idx);
                start_idx += 1;
                if start_idx == 256 {
                    start_idx = 0;
                }
                if end_idx == 0 {
                    end_idx = 255
                } else {
                    end_idx -= 1;
                }
            }
        }

        *position += *l as usize + *skip_size;
        *position %= 256;

        *skip_size += 1;
    }
}