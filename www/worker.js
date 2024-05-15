// The worker has its own scope and no direct access to functions/objects of the global scope.
// We import the generated JS file to make `wasm_bindgen` available which we need to initialize
// our Wasm code.
importScripts('./pkg/albwr.js')

console.log('Initializing worker')

// In the worker, we have a different struct that we want to use as in `index.js`.
const { generate_seed } = wasm_bindgen

async function init_wasm_in_worker() {
    // Load the wasm file by awaiting the Promise returned by `wasm_bindgen`.
    await wasm_bindgen('./pkg/albwr_bg.wasm')

    // Set callback to handle messages passed to the worker.
    self.onmessage = async (event) => {
        event.preventDefault();

        console.log("JS Worker - START");

        // By using methods of a struct as reaction to messages passed to the worker, we can
        // preserve our state between messages.
        let settings = event.data;
        let worker_result = generate_seed(settings);

        // Send response back to be handled by callback in main thread.
        console.log("JS Worker - END");
        self.postMessage(worker_result)
    }
}

init_wasm_in_worker()