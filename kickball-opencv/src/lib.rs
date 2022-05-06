use napi_derive::napi;

use napi::{bindgen_prelude::*, JsGlobal, JsNull, JsUndefined};
use opencv::prelude::*;
use opencv::core::*;
use opencv::imgcodecs::*;
use opencv::imgproc::*;
use opencv::types::*;
use core::ffi::c_void;
use std::iter::IntoIterator;

#[napi]
pub enum ColorConversionCodes {
    ColorBGR2BGRA,
    ColorBGRA2BGR,
    ColorBGR2RGBA,
    ColorRGBA2BGR,
    ColorBGR2RGB,
    ColorBGRA2RGBA,
    ColorBGR2GRAY,
    ColorRGB2GRAY,
    ColorGRAY2BGR,
    ColorGRAY2BGRA,
    ColorBGRA2GRAY,
    ColorRGBA2GRAY,
    ColorBGR2BGR565,
    ColorRGB2BGR565,
    ColorBGR5652BGR,
    ColorBGR5652RGB,
    ColorBGRA2BGR565,
    ColorRGBA2BGR565,
    ColorBGR5652BGRA,
    ColorBGR5652RGBA,
    ColorGRAY2BGR565,
    ColorBGR5652GRAY,
    ColorBGR2BGR555,
    ColorRGB2BGR555,
    ColorBGR5552BGR,
    ColorBGR5552RGB,
    ColorBGRA2BGR555,
    ColorRGBA2BGR555,
    ColorBGR5552BGRA,
    ColorBGR5552RGBA,
    ColorGRAY2BGR555,
    ColorBGR5552GRAY,
    ColorBGR2XYZ,
    ColorRGB2XYZ,
    ColorXYZ2BGR,
    ColorXYZ2RGB,
    ColorBGR2YCrCb,
    ColorRGB2YCrCb,
    ColorYCrCb2BGR,
    ColorYCrCb2RGB,
    ColorBGR2HSV,
    ColorRGB2HSV,
    ColorBGR2Lab,
    ColorRGB2Lab,
    ColorBGR2Luv,
    ColorRGB2Luv,
    ColorBGR2HLS,
    ColorRGB2HLS,
    ColorHSV2BGR,
    ColorHSV2RGB,
    ColorLab2BGR,
    ColorLab2RGB,
    ColorLuv2BGR,
    ColorLuv2RGB,
    ColorHLS2BGR,
    ColorHLS2RGB,
    ColorBGR2HSVFULL,
    ColorRGB2HSVFULL,
    ColorBGR2HLSFULL,
    ColorRGB2HLSFULL,
    ColorHSV2BGRFULL,
    ColorHSV2RGBFULL,
    ColorHLS2BGRFULL,
    ColorHLS2RGBFULL,
    ColorLBGR2Lab,
    ColorLRGB2Lab,
    ColorLBGR2Luv,
    ColorLRGB2Luv,
    ColorLab2LBGR,
    ColorLab2LRGB,
    ColorLuv2LBGR,
    ColorLuv2LRGB,
    ColorBGR2YUV,
    ColorRGB2YUV,
    ColorYUV2BGR,
    ColorYUV2RGB,
    ColorYUV2RGBNV12,
    ColorYUV2BGRNV12,
    ColorYUV2RGBNV21,
    ColorYUV2BGRNV21,
    ColorYUV2RGBANV12,
    ColorYUV2BGRANV12,
    ColorYUV2RGBANV21,
    ColorYUV2BGRANV21,
    ColorYUV2RGBYV12,
    ColorYUV2BGRYV12,
    ColorYUV2RGBIYUV,
    ColorYUV2BGRIYUV,
    ColorYUV2RGBAYV12,
    ColorYUV2BGRAYV12,
    ColorYUV2RGBAIYUV,
    ColorYUV2BGRAIYUV,
    ColorYUV2GRAY420,
    ColorYUV2RGBUYVY,
    ColorYUV2BGRUYVY,
    ColorYUV2RGBAUYVY,
    ColorYUV2BGRAUYVY,
    ColorYUV2RGBYUY2,
    ColorYUV2BGRYUY2,
    ColorYUV2RGBYVYU,
    ColorYUV2BGRYVYU,
    ColorYUV2RGBAYUY2,
    ColorYUV2BGRAYUY2,
    ColorYUV2RGBAYVYU,
    ColorYUV2BGRAYVYU,
    ColorYUV2GRAYUYVY,
    ColorYUV2GRAYYUY2,
    ColorRGBA2mRGBA,
    ColormRGBA2RGBA,
    ColorRGB2YUVI420,
    ColorBGR2YUVI420,
    ColorRGBA2YUVI420,
    ColorBGRA2YUVI420,
    ColorRGB2YUVYV12,
    ColorBGR2YUVYV12,
    ColorRGBA2YUVYV12,
    ColorBGRA2YUVYV12,
    ColorBayerBG2BGR,
    ColorBayerGB2BGR,
    ColorBayerRG2BGR,
    ColorBayerGR2BGR,
    ColorBayerBG2GRAY,
    ColorBayerGB2GRAY,
    ColorBayerRG2GRAY,
    ColorBayerGR2GRAY,
    ColorBayerBG2BGRVNG,
    ColorBayerGB2BGRVNG,
    ColorBayerRG2BGRVNG,
    ColorBayerGR2BGRVNG,
    ColorBayerBG2BGREA,
    ColorBayerGB2BGREA,
    ColorBayerRG2BGREA,
    ColorBayerGR2BGREA,
    ColorBayerBG2BGRA,
    ColorBayerGB2BGRA,
    ColorBayerRG2BGRA,
    ColorBayerGR2BGRA,
    ColorColorCVTMAX,
}

#[napi]
pub struct OpenCv
{
    version: String,
}

#[napi]
impl OpenCv{
    #[napi(constructor)]
    pub fn new()->Self{
        OpenCv{version:"4.5.5".to_string(),}
    }


    #[napi]
    pub fn create_kernel(&self,env:Env,shape:i32,size:Object,anchor:Object)->Object
    {
        let mut obj = env.create_object().unwrap();
        let ksize = env.unwrap::<Size>(&size).unwrap();
        let anchor = env.unwrap::<Point>(&anchor).unwrap();
        let kernel = get_structuring_element(shape,*ksize,*anchor).unwrap();
        let result = env.wrap(&mut obj,kernel);
        obj
    }

    #[napi]
    pub fn create_size(&self,env:Env,width: i32,height:i32)->Object
    {
        let mut obj = env.create_object().unwrap();
        let size = Size::new(width,height);
        let result = env.wrap(&mut obj,size);
        obj
    }

    #[napi]
    pub fn create_point(&self,env:Env,x:i32,y:i32)->Object
    {
        let mut obj = env.create_object().unwrap();
        let point = Point::new(x,y);
        let result = env.wrap(&mut obj,point);
        obj
    }


    #[napi]
    pub fn create_scalar(&self,env:Env,v0:f64,v1:f64,v2:f64,v3:f64)->Object
    {
        let mut obj = env.create_object().unwrap();
        let s = Scalar::new(v0,v1,v2,v3);
        env.wrap(&mut obj,s);
        obj
    }

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

    #[napi]
    pub fn im_decode(&self,env:Env,size:i32,ptr:i64)->Object{
        let data = ptr as *mut c_void;
        unsafe{
        let raw = opencv::core::Mat::new_rows_cols_with_data(1,size,CV_8UC1,data,Mat_AUTO_STEP ).expect("im_decode error");
        let r= imdecode(&raw, IMREAD_UNCHANGED);
        let src = r.ok().unwrap(); //this is a mat
        let mut obj = env.create_object().unwrap();
        env.wrap(&mut obj,src);
        obj
        }
    }


    #[napi]
    pub fn cvt_color(&self,env:Env,src:Object,code: i32, dst_cn: i32)->Object
    {
        let src = env.unwrap::<Mat>(&src).unwrap();
        let mut dst:Mat = Mat::default(); //= Mat::new_rows_cols_with_default(src.rows(),src.cols(),src.typ(),Scalar::new(0.0,0.0,0.0,0.0)).ok().unwrap();
        cvt_color(src,&mut dst,code,dst_cn);
        let mut obj = env.create_object().unwrap();
        env.wrap(&mut obj,dst);
        obj
    }

    #[napi]
    pub fn gaussian_blur(&self,env:Env,src:Object,size:Object,sigma_x: f64, sigma_y: f64, border_type: i32)->Object
    {
        let src = env.unwrap::<Mat>(&src).unwrap();
        let size = env.unwrap::<Size>(&size).unwrap();
        let mut dst:Mat = Mat::default(); //= Mat::new_rows_cols_with_default(src.rows(),src.cols(),src.typ(),Scalar::new(0.0,0.0,0.0,0.0)).ok().unwrap();
        gaussian_blur(src,&mut dst,*size,sigma_x,sigma_y,border_type);
        let mut obj = env.create_object().unwrap();
        env.wrap(&mut obj,dst);
        obj
    }

    #[napi]
    pub fn in_range(&self,env:Env,src:Object,lowerb:Object,upperb:Object)->Object
    {
        let src = env.unwrap::<Mat>(&src).unwrap();
        let lower = env.unwrap::<Scalar>(&lowerb).unwrap();
        let upper = env.unwrap::<Scalar>(&upperb).unwrap();
        let mut dst:Mat = Mat::default(); //= Mat::new_rows_cols_with_default(src.rows(),src.cols(),src.typ(),Scalar::new(0.0,0.0,0.0,0.0)).ok().unwrap();
        in_range(src,lower,upper,&mut dst);
        let mut obj = env.create_object().unwrap();
        env.wrap(&mut obj,dst);
        obj
    }

    #[napi]
    pub fn erode(&self,env:Env,src:Object,kernel:Object,point:Object,iterations:i32,border_type:i32,border_value:Object)->Object
    {

        let defaultBorderValue = morphology_default_border_value().ok().unwrap();
        let src = env.unwrap::<Mat>(&src).unwrap();
        let point = env.unwrap::<Point>(&point).unwrap();
        let borderValue = env.unwrap::<Scalar>(&border_value).unwrap();
        let kernel = env.unwrap::<Mat>(&kernel).unwrap(); //get_structuring_element(0,Size::new(3,3),Point::new(-1,-1)).ok().unwrap();
        let mut dst:Mat = Mat::default(); //= Mat::new_rows_cols_with_default(src.rows(),src.cols(),src.typ(),Scalar::new(0.0,0.0,0.0,0.0)).ok().unwrap();
        erode(src,&mut dst,kernel,*point,iterations,border_type,defaultBorderValue);
        let mut obj = env.create_object().unwrap();
        env.wrap(&mut obj,dst);
        obj
    }

    #[napi]
    pub fn dilate(&self,env:Env,src:Object,kernel:Object,anchor:Object,iterations:i32,border_type:i32,border_value:Object) -> Object{

        let defaultBorderValue = morphology_default_border_value().ok().unwrap();
        let src = env.unwrap::<Mat>(&src).unwrap();
        let point = env.unwrap::<Point>(&anchor).unwrap();
        let borderValue = env.unwrap::<Scalar>(&border_value).unwrap();
        let kernel = env.unwrap::<Mat>(&kernel).unwrap();

        let mut dst:Mat = Mat::default();
        dilate(src,&mut dst,kernel,*point,iterations,border_type,defaultBorderValue);
        let mut obj = env.create_object().unwrap();
        env.wrap(&mut obj,dst);
        obj
    }

    #[napi]
    pub fn find_contours(&self,env:Env,src:Object,mode:i32,method:i32) -> Object{
        let src = env.unwrap::<Mat>(&src).unwrap();
        let mut contours:VectorOfVectorOfPoint =VectorOfVectorOfPoint::default();
        let offset:Point=Point::default();
        find_contours(src,&mut contours,mode,method,offset);
        let mut obj = env.create_object().unwrap();
        env.wrap(&mut obj,contours);
        obj
    }

    #[napi]
    pub fn contour_area(&self,env:Env,src:Object) -> f64{
        let src = env.unwrap::<VectorOfPoint>(&src).unwrap();
        let r = contour_area(src, false);
        let area_float:f64 = r.ok().unwrap();
        area_float
    }

    #[napi]
    pub fn get_area_max_contour(&self,env:Env,contours:Object,threshold:f64) -> Object{
        let contours = env.unwrap::<VectorOfVectorOfPoint>(&contours).unwrap();
        let contours_iter = contours.iter();
        let mut max_area_float:f64 = 0.0;
        let mut max_contour:VectorOfPoint = VectorOfPoint::default();
        for contour in contours_iter {
            // println!("{:?}", contour);
            let r = contour_area(&contour, false);
            let contour_area_temp:f64 = r.ok().unwrap();
            //println!("{}\n", contour_area_temp);
            if contour_area_temp < threshold {
                continue;
            }
            if contour_area_temp > max_area_float {
                max_area_float = contour_area_temp;
                max_contour = contour;
            }
        }
        // println!("max contour area is {}\n", max_area_float);
        let mut obj = env.create_object().unwrap();
        env.wrap(&mut obj ,max_contour);
        obj
    }

    #[napi]
    pub fn min_enclosing_circle(&self,env:Env,points:Object) -> Object{
        let src = env.unwrap::<VectorOfPoint>(&points).unwrap();
        let mut center:Point2f=Point2f::default();
        let mut radius:f32 = 0.0;
        min_enclosing_circle(src, &mut center, &mut radius);
        let mut obj = env.create_object().unwrap();
        let centerx = env.create_double(center.x.into()).unwrap();
        let centery = env.create_double(center.y.into()).unwrap();
        let r = env.create_double(radius.into()).unwrap();
        obj.set_named_property("x", centerx).unwrap();
        obj.set_named_property("y", centery).unwrap();
        obj.set_named_property("r", r).unwrap();
        obj
    }
}

#[napi]
pub struct JsPoint{
}

#[napi]
impl JsPoint{
   #[napi(constructor)]
    pub fn new(env:Env,x:i32,y:i32)->Object{
        let mut obj = env.create_object().unwrap();
        let a =  Point::new(x,y);
        env.wrap(&mut obj,a);
        obj
    }
}

#[napi]
pub struct JsMat
{
    mat: Mat,
    rows: i32,
    cols: i32,
    typ: i32,
}

#[napi]
impl JsMat{
   #[napi(constructor)]
    pub fn new(rows:i32,cols:i32,typ:i32)->Self{
        JsMat{
             mat: Mat::new_rows_cols_with_default(rows,cols,typ,Scalar::new(0.0,0.0,0.0,0.0)).ok().unwrap(),
            rows:rows,
            cols:cols,
            typ:typ,
        }
    }

   #[napi(getter)]
    pub fn get_rows(&self)->i32{
        self.rows
    }
     #[napi(getter)]
    pub fn get_cols(&self)->i32{
        self.cols
    }
 #[napi(getter)]
    pub fn get_typ(&self)->i32{
        self.typ
    }

}

#[napi]
fn list_obj_keys(obj: Object) -> Vec<String> {
  Object::keys(&obj).unwrap()
}

static mut MODULE: sys::napi_module = sys::napi_module {
    nm_version: 1,
    nm_flags: 0,
    nm_filename: "lib.rs\u{0}".as_ptr() as *const _,
    nm_register_func: Some(napi_register_module_v1),
    nm_modname: concat!(env!("CARGO_PKG_NAME"), "\u{0}").as_ptr() as *const _,
    nm_priv: std::ptr::null_mut(),
    reserved: [std::ptr::null_mut(); 4],
};

#[napi::bindgen_prelude::ctor]
fn _my_module_init() {
    unsafe { sys::napi_module_register(&mut MODULE as *mut _) }
}

extern "C" {
    fn napi_register_module_v1(env: sys::napi_env, exports: sys::napi_value) -> sys::napi_value;
}
