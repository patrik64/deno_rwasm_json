import init , { source, get_json, set_json } from "./wasm.js";

await init(source);

const testJsonString = get_json();
const testJson = JSON.parse(testJsonString);

console.log("json received from rust:");
console.log("   ");
console.log(testJson);
console.log("   ");
interface Address {
  number:   number;
  street:   string;
  city:     string;
}

const jsonAddress: Address = { "city" : "Vienna", "number" : 1, "street" : "Maxingstraße"};

const strJson = JSON.stringify(jsonAddress);

set_json(strJson);

