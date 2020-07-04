const js = import("../wasm/koto");
document.getElementById("run").onclick = function () {
  js.then(js => {
    js.runs();
  });
}
