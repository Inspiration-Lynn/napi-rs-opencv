use napi_derive::napi;
use napi::bindgen_prelude::*;

use opencv::prelude::*;
use opencv::core::*;
use opencv::imgcodecs::*;
use opencv::aruco::*;
use opencv::types::*;


#[napi]
fn fibonacci(n: u32) -> u32 {
  match n {
    1 | 2 => 1,
    _ => fibonacci(n - 1) + fibonacci(n - 2),
  }
}

pub fn read_camera_parameters(filename: String) -> (Mat, Mat){
  // read camera parameters from yml/yaml
  let mut camera_matrix: Mat = Mat::default();
  let mut dist_coeffs: Mat = Mat::default();
  let fs: FileStorage = FileStorage::new(&filename, 0, &String::new()).expect("FileStorage::new fail");
  camera_matrix = fs.get_node("camera_matrix").expect("get_node fail").mat().expect("mat fail");
  dist_coeffs = fs.get_node("distortion_coefficients").expect("get_node fail").mat().expect("mat fail");
  println!("camera_matrix: {:?}", camera_matrix);
  println!("dist_coeffs: {:?}", dist_coeffs);

  (camera_matrix, dist_coeffs)
}

// #[napi]
// pub enum PREDEFINED_DICTIONARY_NAME {
//   DICT_4X4_50,
//   DICT_4X4_100,
//   DICT_4X4_250,
//   DICT_4X4_1000,
//   DICT_5X5_50,
//   DICT_5X5_100,
//   DICT_5X5_250,
//   DICT_5X5_1000,
//   DICT_6X6_50,
//   DICT_6X6_100,
//   DICT_6X6_250,
//   DICT_6X6_1000,
//   DICT_7X7_50,
//   DICT_7X7_100,
//   DICT_7X7_250,
//   DICT_7X7_1000,
//   DICT_ARUCO_ORIGINAL,
//   DICT_APRILTAG_16h5,
//   DICT_APRILTAG_25h9,
//   DICT_APRILTAG_36h10,
//   DICT_APRILTAG_36h11,
// }

// Class OpenCv in JS
#[napi]
pub struct OpenCv {
  version: String,
}

#[napi]
impl OpenCv {
  // This is the constructor
  #[napi(constructor)]
  pub fn new() -> Self {
    OpenCv { version: "4.5.5".to_string() }
  }

  #[napi(getter)]
  pub fn get_version(&self) -> &str {
    self.version.as_str()
  }

  // Class method
  #[napi]
  pub fn im_read(&self, env: Env, filename: String) -> Object {
    let img_result = imread(&filename, IMREAD_UNCHANGED);
    // Mat in Rust: img
    let img = img_result.expect("imread fail");   // r.ok().unwrap(); 
    // create JsObject: obj
    let mut obj = env.create_object().expect("create_object fail");
    env.wrap(&mut obj, img).expect("wrap fail");
    obj
  }

  #[napi]
  pub fn im_write(&self, env: Env, filename: String, obj: Object) {
    let src = env.unwrap::<Mat>(&obj).expect("unwrap fail");
    imwrite(&filename, src, &Vector::new()).expect("imwrite fail");
  }


  #[napi]
  pub fn detect_markers(&self, env: Env, filename_in: String, filename_out: String, camera_params: String) {
    // marker detection
    let image = imread(&filename_in, 1).expect("imread fail");
    let dictionary: Ptr<Dictionary> = get_predefined_dictionary_i32(DICT_6X6_250).expect("get_predefined_dictionary fail");
    let mut corners: VectorOfVectorOfPoint2f = VectorOfVectorOfPoint2f::default();
    let mut ids: VectorOfi32 = VectorOfi32::default();
    let parameters: Ptr<DetectorParameters> = DetectorParameters::create().expect("DetectorParameters::create fail");
    let mut rejected_img_points: VectorOfVectorOfPoint2f = VectorOfVectorOfPoint2f::default();

    detect_markers(&image, &dictionary, &mut corners, &mut ids, &parameters, &mut rejected_img_points, &no_array(), &no_array()).expect("detect_marker fail");
    // println!("corners: {:?}", corners);

    let mut clone_img: Mat = opencv::core::Mat::clone(&image);
    let border_color: Scalar = Scalar::new(0.0, 255.0, 0.0, 0.0);
    draw_detected_markers(&mut clone_img, &corners, &ids, border_color).expect("draw_detected_markers fail");
    imwrite(&filename_out.to_string(), &clone_img, &Vector::new()).expect("imwrite fail");

    // read camera parameters from yml/yaml
    let mut camera_matrix: Mat = Mat::default();
    let mut dist_coeffs: Mat = Mat::default();
    // let fs: FileStorage = FileStorage::new(&camera_params, 0, &String::new()).expect("FileStorage::new fail");
    // camera_matrix = fs.get_node("camera_matrix").expect("get_node fail").mat().expect("mat fail");
    // dist_coeffs = fs.get_node("distortion_coefficients").expect("get_node fail").mat().expect("mat fail");
    // println!("camera_matrix: {:?}", camera_matrix);
    // println!("dist_coeffs: {:?}", dist_coeffs);
    (camera_matrix, dist_coeffs) = read_camera_parameters(camera_params);

    // pose estimation
    // if at least one marker detected
    if ids.len() > 0 {
      let mut rvecs: VectorOfPoint3d = VectorOfPoint3d::default();
      let mut tvecs: VectorOfPoint3d = VectorOfPoint3d::default();
      estimate_pose_single_markers(&corners, 0.05, &camera_matrix, &dist_coeffs, &mut rvecs, &mut tvecs, &mut no_array());
      println!("rvecs: {:?}", rvecs);
      println!("tvecs: {:?}", tvecs);

      let image2 = imread(&filename_in, IMREAD_GRAYSCALE).expect("imread fail");
      let mut clone_img: Mat = opencv::core::Mat::clone(&image2);
      // println!("len: {:?}", ids.len());
    }


  }



}


