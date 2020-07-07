document.getElementById("run").onclick = function () {
  document.getElementById("result").textContent = "";
  import("../pkg/index.js").then(mod => {
    try {
      let code = document.getElementById("code").value;
      mod.run(code);
    } catch (error) {
      document.getElementById("result").textContent = "Error!"
      console.log(error);
    }
  });
}