const { fibonacci } = require("./libmarker_detection.node");
const a = require("./libmarker_detection.node");

console.log("fib: ", fibonacci(11));

var opcv = new a.OpenCv();

console.log("[before imRead]");
var src = opcv.imRead("ball.jpg");
console.log("[after imRead]");
opcv.imWrite("./out/output.jpg", src);

opcv.detectMarkers("marker.png", "./out/marker_out1.png");
