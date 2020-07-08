const wasm = import("../pkg/index.js");

document.getElementById("run").onclick = function () {
  document.getElementById("result").textContent = "";
  wasm.then(mod => {
    try {
      let code = document.getElementById("code").value;
      mod.run(code);
    } catch (error) {
      document.getElementById("result").textContent = "Error!"
      console.log(error);
    }
  });
}