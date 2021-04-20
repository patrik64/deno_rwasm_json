use wasm_bindgen::prelude::wasm_bindgen;

extern crate serde;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize)]
struct Address {
    number: Option<u32>,
    street: String,
    city: String,
}

#[wasm_bindgen]
pub fn get_json() -> String {
    let data = json!({
        "addresses": vec![
            Address {
                number: Some(10),
                street: "Downing Street".to_owned(),
                city: "London".to_owned(),
            },
            Address {
                number: None,
                street: "Covent Garden".to_owned(),
                city: "London".to_owned(),
            },
        ],
        "someNumber": 17,
        "aListOfStrings": [
            "string 1",
            "string 2",
        ],
    });
    serde_json::to_string(&data).unwrap()
}