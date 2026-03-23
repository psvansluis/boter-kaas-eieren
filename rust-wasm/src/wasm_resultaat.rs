use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum WasmResultaat<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> From<Result<T, E>> for WasmResultaat<T, E> {
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(value) => WasmResultaat::Ok(value),
            Err(error) => WasmResultaat::Err(error),
        }
    }
}
