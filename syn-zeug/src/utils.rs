use phf::Map;

pub fn expand_iupac(seq: impl AsRef<[u8]>, map: &Map<u8, &[u8]>) -> Vec<Vec<u8>> {
    let seq = seq.as_ref();
    let mut expansions = vec![seq.to_vec()];

    for i in 0..seq.len() {
        let es: Vec<_> = expansions
            .iter()
            .flat_map(|e| {
                map.get(&seq[i])
                    .unwrap_or(&b"".as_slice())
                    .iter()
                    .map(|&a| [&e[..i], &[a], &e[i + 1..]].concat())
            })
            .collect();

        expansions.extend(es);
    }

    expansions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{IUPAC_DNA, IUPAC_PROTEIN, IUPAC_RNA};

    #[test]
    fn expand_one_n_dna() {
        let mut expansion = expand_iupac(b"N", &IUPAC_DNA);

        // Length Check
        assert_eq!(expansion.len(), 15);

        // Uniqueness Check
        let e_len = expansion.len();
        expansion.sort_unstable();
        expansion.dedup();
        assert_eq!(e_len, expansion.len());

        // Values Check
        let all_codes: Vec<_> = b"ABCDGHKMNRSTVWY".iter().map(|&c| vec![c]).collect();
        assert_eq!(expansion, all_codes);
    }

    #[test]
    fn expand_one_n_rna() {
        let mut expansion = expand_iupac(b"N", &IUPAC_RNA);

        // Length Check
        assert_eq!(expansion.len(), 15);

        // Uniqueness Check
        let e_len = expansion.len();
        expansion.sort_unstable();
        expansion.dedup();
        assert_eq!(e_len, expansion.len());

        // Values Check
        let all_codes: Vec<_> = b"ABCDGHKMNRSUVWY".iter().map(|&c| vec![c]).collect();
        assert_eq!(expansion, all_codes);
    }

    #[test]
    fn expand_one_n_protein() {
        // Test B ==================================================================================
        let mut expansion = expand_iupac(b"B", &IUPAC_PROTEIN);

        // Length Check
        assert_eq!(expansion.len(), 3);

        // Uniqueness Check
        let e_len = expansion.len();
        expansion.sort_unstable();
        expansion.dedup();
        assert_eq!(e_len, expansion.len());

        // Values Check
        let all_codes: Vec<_> = b"BDN".iter().map(|&c| vec![c]).collect();
        assert_eq!(expansion, all_codes);

        // Test Z ==================================================================================
        let mut expansion = expand_iupac(b"Z", &IUPAC_PROTEIN);

        // Length Check
        assert_eq!(expansion.len(), 3);

        // Uniqueness Check
        let e_len = expansion.len();
        expansion.sort_unstable();
        expansion.dedup();
        assert_eq!(e_len, expansion.len());

        // Values Check
        let all_codes: Vec<_> = b"EQZ".iter().map(|&c| vec![c]).collect();
        assert_eq!(expansion, all_codes);

        // Test X ==================================================================================
        let mut expansion = expand_iupac(b"X", &IUPAC_PROTEIN);

        // Length Check
        assert_eq!(expansion.len(), 23);

        // Uniqueness Check
        let e_len = expansion.len();
        expansion.sort_unstable();
        expansion.dedup();
        assert_eq!(e_len, expansion.len());

        // Values Check
        let all_codes: Vec<_> = b"ABCDEFGHIKLMNPQRSTVWXYZ"
            .iter()
            .map(|&c| vec![c])
            .collect();
        assert_eq!(expansion, all_codes);
    }

    #[test]
    fn expand_k_n() {
        let expansion = expand_iupac(b"N", &IUPAC_DNA);
        assert_eq!(expansion.len(), 15);
        let expansion = expand_iupac(b"NN", &IUPAC_DNA);
        assert_eq!(expansion.len(), 225);
        let expansion = expand_iupac(b"NNN", &IUPAC_DNA);
        assert_eq!(expansion.len(), 3375);
    }

    #[test]
    fn stop_codons() {
        let mut expansion = expand_iupac(b"UAR", &IUPAC_RNA);
        assert_eq!(expansion.len(), 3);
        expansion.extend(expand_iupac(b"URA", &IUPAC_RNA));
        expansion.sort_unstable();
        expansion.dedup();
        assert_eq!(expansion.len(), 5);
        let stop_codons = [b"UAA", b"UAG", b"UAR", b"UGA", b"URA"];
        assert_eq!(expansion, stop_codons);
    }
    // expansion.sort_unstable();
    // println!("{}", expansion.len());
    // let expansion: Vec<u8> = expansion.join(b" ".as_slice());
    // println!("{}", std::str::from_utf8(&expansion).unwrap());
    // panic!()
}
