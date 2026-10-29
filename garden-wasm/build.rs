fn main() {
    wasm_bindgen::bindgen().generate().expect("Unable to generate wasm-bindings");
}
