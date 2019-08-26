use rand::Rng;
const LETTER_BYTES: &'static str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LETTER_IDX_BITS: u32 = 6;
const LETTER_IDX_MASK: u64 = (1 << LETTER_IDX_BITS) - 1;
const LETTER_IDX_MAX: u32 = 63 / LETTER_IDX_BITS;

pub fn rand_string_bytes_mask_impr(n: i32) -> String {
    let mut b: Vec<char> = vec!['0'; 5];

    let (mut i, mut cache, mut remain) = (n - 1, rand::thread_rng().gen::<u64>(), LETTER_IDX_MAX);
    while i >= 0 {
        if remain == 0 {
            cache = rand::thread_rng().gen::<u64>();
            remain = LETTER_IDX_MAX;
        };
        let idx = (cache & LETTER_IDX_MASK) as i64;
        if idx < 36 {
            b[i as usize] = LETTER_BYTES.chars().nth(idx as usize).unwrap();
            i -= 1;
        }
        cache >>= LETTER_IDX_BITS;
        remain -= 1;
    }
    b.iter().cloned().collect::<String>()
}
