// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import init from "./testwasm.js";

console.log("well")
const runWasm = async () => {
    console.log("in")
    // Instantiate our wasm module
    const helloWorld = await init();
    console.log("after")

    // Call the Add function export from wasm, save the result
    console.log(helloWorld.test_me)
    const value = helloWorld.test_me(10, 11);
    console.log("ahat")
    console.log(value)
};
runWasm();