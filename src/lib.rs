use wasm_bindgen::prelude::wasm_bindgen;

extern crate serde;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize, Debug)]
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

#[wasm_bindgen]
pub fn set_json(json_string_input: &str) {
    let address: Address = serde_json::from_str(&json_string_input).unwrap();
    let fmt_address = format!("{:?}", &address);
    log("json received from deno as a string, deserialized to a struct and printed from within rust: ");
    log(&fmt_address);
    let str = serde_json::to_string(&address).unwrap();
    log("the same rust struct serialized and printed from within rust: ");
    log(&str);
}