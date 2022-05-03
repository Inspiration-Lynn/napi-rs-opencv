use napi_derive::napi;
use napi::bindgen_prelude::*;

use opencv::prelude::*;
use opencv::core::*;
use opencv::imgcodecs::*;

#[napi]
fn fibonacci(n: u32) -> u32 {
  match n {
    1 | 2 => 1,
    _ => fibonacci(n - 1) + fibonacci(n - 2),
  }
}


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

  // #[napi(getter)]
  // pub fn get_version(&self) -> &str {
  //   self.version.as_str()
  // }

  // // Class method
  // #[napi]
  // pub fn im_read(&self, env: Env, filename: String) -> Object {
  //   let img = imread(&filename, 1);
  //   // Mat in Rust: src
  //   let src = r.ok().expect("imread fail");   // r.ok().unwrap(); 
  //   // create JsObject: obj
  //   let mut obj = env.create_object().expect("create_object fail");
  //   env.wrap(&mut obj, src);
  //   obj
  // }

  // #[napi]
  // pub fn im_write(&self, env: Env, filename: String, obj: Object) {
  //   let src = env.unwrap::<Mat>(&obj).expect("unwrap fail");
  //   imwrite(&filename, src, &Vector::new());
  // }

  #[napi]
    pub fn im_write(&self,env:Env,path:String,obj:Object){
        let src = env.unwrap::<Mat>(&obj).unwrap();
        imwrite(&path.to_string(),src,&Vector::new());
    }

  #[napi]
  pub fn im_read(&self,env:Env,path:String)->Object{
      let r = imread(&path,1);
      let src = r.ok().unwrap(); //this is a mat
      let mut obj = env.create_object().unwrap();
      env.wrap(&mut obj,src);
      obj
  }

}
