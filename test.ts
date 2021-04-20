import init , { source, get_json } from "./wasm.js";

await init(source);

const testJsonString = get_json();
const testJson = JSON.parse(testJsonString);

console.log("json received from rust:");
console.log("   ");
console.log(testJson);
console.log("   ");

