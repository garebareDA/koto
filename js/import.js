export function output_result(val) {
  const bforeVal = document.getElementById("result").textContent;
  if (bforeVal === "") {
    const afterVal = val;
    document.getElementById("result").textContent = afterVal;
  } else {
    const afterVal = bforeVal + "\n" + val;
    document.getElementById("result").textContent = afterVal;
  }
}