use core::slice::{self};
use std::{fs::File, hint::black_box, io::Read, time::SystemTime};

use sha2::{
    compress256,
    digest::{generic_array::GenericArray, typenum::U64},
};

const H256_256_U8: [u8; 32] = [
    103, 230, 9, 106, 133, 174, 103, 187, 114, 243, 110, 60, 58, 245, 79, 165, 127, 82, 14, 81,
    140, 104, 5, 155, 171, 217, 131, 31, 25, 205, 224, 91,
];

fn main() {
    let mut f = File::open("/dev/random").unwrap();
    let mut buf = vec![0u8; 128 << 20];
    f.read_exact(&mut buf).unwrap();

    let start = SystemTime::now();
    let mut one: GenericArray<u8, U64> = GenericArray::default();
    let mut two: GenericArray<u8, U64> = GenericArray::default();
    for chunk in buf.chunks_exact_mut(64) {
        one.copy_from_slice(chunk);
        for _ in 0..400 {
            two[..32].copy_from_slice(H256_256_U8.as_slice());
            let state = unsafe {
                let (_, state, _) = two.align_to_mut::<u32>();
                state
            };
            compress256((&mut state[..8]).try_into().unwrap(), slice::from_ref(&one));
            for i in 0..8 {
                state[i] = state[i].to_be();
            }
            (one, two) = (two, one);
        }
    }
    black_box(one);
    black_box(two);
    let elapsed = start.elapsed().unwrap();
    let n = buf.len() / 64;
    println!(
        "elapsed={:?} chunks={:?} per second={:?}",
        elapsed,
        n,
        n as f64 / elapsed.as_secs() as f64,
    )
}
