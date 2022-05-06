const Aruco = require("./libmarker_detection.node");

var aruco = new Aruco.MarkerDetection();

// console.log("[before imRead]");
// var src = marker_detect.imRead("ball.jpg");
// console.log("[after imRead]");
// marker_detect.imWrite("./out/output.jpg", src);

aruco.detectMarkers("marker.png", "./out/marker_detection_out.png", 10);

aruco.poseEstimation(
  "marker.png",
  "./out/pose_estimation_out.png",
  10,
  "./camera_params.yml"
);
