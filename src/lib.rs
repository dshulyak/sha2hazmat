#[cfg(test)]
mod tests {
    use sha2::{
        compress256,
        digest::{generic_array::GenericArray, typenum::U64},
        Digest, Sha256,
    };

    const H256_256: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];

    

    #[test]
    fn hazmat() {
        let mut hash = Sha256::default();
        let mut input = [0u8; 64];
        
        let lth: u64 = 55;
        hash.update(&input[..lth as usize]);
        let out1 = hash.finalize();

        input[lth as usize] = 0x80;
        input[56..64].copy_from_slice(&(lth << 3).to_be_bytes());

        let blocks = [*GenericArray::<u8, U64>::from_slice(&input)];
        let mut out2 = H256_256.clone();
        compress256(&mut out2, &blocks);

        for i in 0..8 {
            out2[i] = out2[i].to_be();
        }
        let out2 = unsafe {
            let (_, out2, _) = out2.align_to::<u8>();
            out2
        };
        assert_eq!(out1.as_slice(), out2);
    }
}
