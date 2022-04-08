// TODO: Keep an eye on this: https://github.com/rustwasm/wasm-bindgen/issues/2774
// #![allow(clippy::unused_unit)]
// Keep serde_wasm_bindgen in mind if you need to send fancy types to JS
// trace_macros!(true);
macro_rules! wasmify_impl {
    ($impl_type:ty => $wrapper:ident, {$([$($tt:tt)+])+}) => {
        pub struct $wrapper($impl_type);
        impl $wrapper {
            $(
                wasmify_impl!($impl_type, $wrapper, $($tt)+);
            )+
        }
    };
    ($i:ty, $w:ident, $name:ident(&self) -> Result<$t:ty, $e:ty>) => {
        pub fn $name(&self) -> Result<$t, $e> {
            Ok($w(self.0.$name().map_err(|e| e.to_string())?))
        }
    };
    ($i:ty, $w:ident, $name:ident(&self) -> $out:ty) => {
        pub fn $name(&self) -> $out {
            self.0.$name()
        }
    };
    ($i:ty, $w:ident, $name:ident($($arg:ident: $ty:ty),+) -> Result<$t:ty, $e:ty>) => {
        pub fn $name($($arg: $ty),+) -> Result<$t, $e> {
            Ok($w(<$i>::$name($($arg),+).map_err(|e| e.to_string())?))
        }
    };
}

// TODO: Could make this look more like a trait definition with a proc macro
wasmify_impl!(syn_zeug::seq::Seq => Seq, {
    [new(seq: String) -> Result<Seq, String>]
    [dna(seq: String) -> Result<Seq, String>]
    [rna(seq: String) -> Result<Seq, String>]
    [protein(seq: String) -> Result<Seq, String>]
    [len(&self) -> usize]
    [is_empty(&self) -> bool]
    // TODO: Add `count_elements`
    [reverse_complement(&self) -> Result<Seq, String>]
    // TODO: Add `convert`
    [to_string(&self) -> String]
});

// wasmify_impl!(fn reverse_complement(&self) -> Result<Seq,String>;);

// use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// pub struct Seq(syn_zeug::seq::Seq);

// #[wasm_bindgen]
// impl Seq {
//     // #[wasm_bindgen(constructor)]
//     pub fn new(seq: String) -> Result<Seq, String> {
//         Ok(Seq(syn_zeug::seq::Seq::new(seq).map_err(|e| e.to_string())?))
//     }
//
//     pub fn reverse_complement(&self) -> Result<Seq, String> {
//         Ok(Seq(self
//             .0
//             .reverse_complement()
//             .map_err(|e| e.to_string())?))
//     }
//
//     // #[allow(clippy::inherent_to_string)]
//     pub fn to_string(&self) -> String {
//         self.0.to_string()
//     }
// }
