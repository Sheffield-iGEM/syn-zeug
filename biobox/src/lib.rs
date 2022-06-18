use std::collections::HashMap;

use syn_zeug::seq::Seq as SZSeq;
use wasm_bindgen::prelude::*;

macro_rules! wrap_res {
    ($e:expr) => {
        Ok(Seq($e.map_err(|e| e.to_string())?))
    };
}

#[wasm_bindgen]
pub struct Seq(SZSeq);

#[wasm_bindgen]
impl Seq {
    #[wasm_bindgen(constructor)]
    pub fn new(seq: String) -> Result<Seq, String> {
        wrap_res!(SZSeq::new(seq))
    }

    pub fn dna(seq: String) -> Result<Seq, String> {
        wrap_res!(SZSeq::dna(seq))
    }

    pub fn dna_n(seq: String) -> Result<Seq, String> {
        wrap_res!(SZSeq::dna_n(seq))
    }

    pub fn dna_iupac(seq: String) -> Result<Seq, String> {
        wrap_res!(SZSeq::dna_iupac(seq))
    }

    pub fn rna(seq: String) -> Result<Seq, String> {
        wrap_res!(SZSeq::rna(seq))
    }

    pub fn rna_n(seq: String) -> Result<Seq, String> {
        wrap_res!(SZSeq::rna_n(seq))
    }

    pub fn rna_iupac(seq: String) -> Result<Seq, String> {
        wrap_res!(SZSeq::rna_iupac(seq))
    }

    pub fn protein(seq: String) -> Result<Seq, String> {
        wrap_res!(SZSeq::protein(seq))
    }

    pub fn protein_iupac(seq: String) -> Result<Seq, String> {
        wrap_res!(SZSeq::protein_iupac(seq))
    }

    pub fn kind(&self) -> String {
        self.0.kind().to_string()
    }

    pub fn alphabet(&self) -> String {
        self.0.alphabet().to_string()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn rev(&self) -> Seq {
        Seq(self.0.rev())
    }

    // TODO: Should I swap out the String for serde_wasm_bindgen::Error?
    pub fn count_elements(&self) -> Result<JsValue, String> {
        let map: HashMap<char, _> = self.0.count_elements().to_hashmap(|_, &x| x != 0);
        serde_wasm_bindgen::to_value(&map).map_err(|e| e.to_string())
    }

    pub fn reverse_complement(&self) -> Result<Seq, String> {
        wrap_res!(self.0.reverse_complement())
    }

    pub fn convert(&self, kind: JsValue) -> Result<Seq, String> {
        let kind = serde_wasm_bindgen::from_value(kind).map_err(|e| e.to_string())?;
        wrap_res!(self.0.convert(kind))
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}
