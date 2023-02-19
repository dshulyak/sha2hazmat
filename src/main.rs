use std::{fs::File, io::Read, time::SystemTime};

use sha2::{
    compress256,
    digest::{generic_array::GenericArray, typenum::U64},
};

const H256_256: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

fn main() {
    let mut f = File::open("/dev/random").unwrap();
    let mut buf = vec![0u8; 128 << 20];
    f.read_exact(&mut buf).unwrap();

    let start = SystemTime::now();
    for chunk in buf.chunks_exact_mut(64) {
        let blocks = [*GenericArray::<u8, U64>::from_slice(chunk)];
        
        for _ in 0..400 {
            let mut state = H256_256.clone();
            compress256(&mut state, &blocks);
            for i in 0..8 {
                state[i] = state[i].to_be();
            }
            let aligned = unsafe {
                let (_, aligned, _) = state.align_to::<u8>();
                aligned
            };
            chunk[..32].copy_from_slice(aligned);
        }
    }
    let elapsed = start.elapsed().unwrap();
    let n = buf.len() / 64;
    println!(
        "elapsed={:?} chunks={:?} per second={:?}",
        elapsed,
        n,
        n as f64 / elapsed.as_secs() as f64,
    )
}
