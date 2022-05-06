const Opencv = require("./libjsopencv.node");
const Aruco = require("./libmarker_detection.node");

var cv = new Opencv.OpenCv();
var aruco = new Aruco.MarkerDetection();

var src = cv.imRead("ball.jpg");
cv.imWrite("./out/output.jpg", src);

aruco.detectMarkers("marker.png", "./out/marker_detection_out.png", 10);

aruco.poseEstimation(
  "marker.png",
  "./out/pose_estimation_out.png",
  10,
  "./camera_params.yml"
);
