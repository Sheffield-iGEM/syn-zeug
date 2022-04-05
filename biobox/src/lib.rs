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

    pub fn reverse_complement(&self) -> Result<Seq, String> {
        Ok(Seq(self.0.reverse_complement().map_err(|e| e.to_string())?))
    }

    // TODO: I don't know if I need this – maybe I can just implement Display?
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}
