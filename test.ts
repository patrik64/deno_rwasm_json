import init , { source, get_json, set_json } from "./wasm.js";

await init(source);

const testJsonString = get_json();
const testJson = JSON.parse(testJsonString);

console.log("json received from rust:");
console.log("   ");
console.log(testJson);
console.log("   ");

const jsonAddress = { "city" : "Vienna", "number" : 23, "street" : "Hietzing"};

const strJson = JSON.stringify(jsonAddress);

set_json(strJson);

