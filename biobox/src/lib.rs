use syn_zeug::seq::Seq as SZSeq;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Seq(SZSeq);

#[wasm_bindgen]
impl Seq {
    #[wasm_bindgen(constructor)]
    pub fn new(seq: String) -> Result<Seq, String> {
        Ok(Seq(SZSeq::new(seq).map_err(|e| e.to_string())?))
    }

    pub fn reverse_complement(&self) -> String {
        // TODO: Surely there is a better way to do this?
        match self.0.reverse_complement() {
            Ok(s) => s.to_string(),
            Err(e) => e.to_string(),
        }
    }
}
