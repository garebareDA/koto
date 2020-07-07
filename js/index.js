document.getElementById("run").onclick = function () {
  document.getElementById("result").textContent = "";
  import("../pkg/index.js").then(mod => {
    try {
      let code = document.getElementById("code").value;
      console.log(code);
      mod.run(code);
    } catch (error) {
      console.log(error);
    }
  });
}