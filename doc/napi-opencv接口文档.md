# napi-opencv接口文档

## 动态库 - libjsopencv.node

### 类 - OpenCv

#### 创建opencv对象

##### Scalar

```javascript
function createScalar(v0: Number,v1: Number,v2: Number,v3: Number): scalar: Object
```

##### Point

```javascript
function createPoint(x: Number, y: Number): point: Object
```
##### Kernel

```javascript
function createKernel(shape: Number, size: Object, anchor: Object): kernel: Object
```
##### Size

```javascript
function createSize(width: Number, height: Number): size: Object
```



#### 枚举

##### ColorConversionCodes

jsCvtColor()



#### 类方法

##### imRead：从文件中读图片

```javascript
function imRead(path: String): img: Object
```

- Parameters：

  - path: Name of file to be loaded

- returns:

  - img（object wrapped）： Mat


##### imWrite：将图片保存在文件中

```javascript
function imWrite(path: String, obj: Object)
```

- Parameters：

  - path: Name of the file.
  - obj（object wrapped）： (Mat) Image to be saved.

##### cvtColor：转换图片色彩空间

```javascript
function cvtColor(src: Object, code: Number, dst_cn: Number): dst: Object
```

- Parameters：

  - src（object wrapped）: Source image with CV_8U , CV_16U , or CV_32F depth and 1, 3, or 4 channels.
  - code: Color space conversion code. For details, see cvtColor .
  - dst_cn: Number of channels in the destination image. If the parameter is 0, the number of the channels is derived automatically from src and the code .

- returns:

  - dst（object wrapped）: Destination image


##### gaussianBlur：使用高斯滤波模糊图片

```javascript
function gaussianBlur(src: Object, size: Object, sigma_x: Number, sigma_y: Number, border_type: Number): dst: Object
```

- Parameters：

  - src（object wrapped）: input image; the image can have any number of channels, which are processed independently, but the depth should be CV_8U, CV_16U, CV_16S, CV_32F or CV_64F.
  - size（object wrapped）: Gaussian kernel size. ksize.width and ksize.height can differ but they both must be positive and odd. Or, they can be zero’s and then they are computed from sigma.
  - sigma_x（float）: Gaussian kernel standard deviation in X direction.
  - sigma_y（float）: Gaussian kernel standard deviation in Y direction; if sigmaY is zero, it is set to be equal to sigmaX, if both sigmas are zeros, they are computed from ksize.width and ksize.height, respectively (see #getGaussianKernel for details); to fully control the result regardless of possible future modifications of all this semantics, it is recommended to specify all of ksize, sigmaX, and sigmaY.
  - border_type: pixel extrapolation method, see #BorderTypes. #BORDER_WRAP is not supported.

- returns:

  - dst（object wrapped）: output image of the same size and type as src.

##### inRange：二值化

```javascript
function inRange(src: Object, lowerb: Object, upperb: Object): dst: Object
```

- Parameters：

  - src（object wrapped）: first input array.
  - lowerb（object wrapped）: inclusive lower boundary array or a scalar.
  - upperb（object wrapped）: inclusive upper boundary array or a scalar.

- returns:

  - dst（object wrapped）: output array of the same size as src and CV_8U type.

##### erode：图片腐蚀

```javascript
function erode(src: Object, kernel: Object, point: Object, iterations: Number, border_type: Number, border_value: Object): dst: Object
```

- Parameters：

  - src（object wrapped）: input image; the number of channels can be arbitrary, but the depth should be one of CV_8U, CV_16U, CV_16S, CV_32F or CV_64F.
  - kernel（object wrapped）: structuring element used for dilation; if elemenat=Mat(), a 3 x 3 rectangular structuring element is used. Kernel can be created using #getStructuringElement
  - anchor（object wrapped）: position of the anchor within the element; default value (-1, -1) means that the anchor is at the element center.
  - iterations: number of times dilation is applied.
  - borderType: pixel extrapolation method, see #BorderTypes. #BORDER_WRAP is not suported.
  - borderValue（object wrapped）: border value in case of a constant border

- returns:

  - dst（object wrapped）: output image of the same size and type as src.


##### dilate：图片膨胀

```javascript
function dilate(src: Object, kernel: Object, anchor: Object, iterations: Number, border_type: Number, border_value: Object): dst: Object
```

- Parameters：

  - src（object wrapped）: input image; the number of channels can be arbitrary, but the depth should be one of CV_8U, CV_16U, CV_16S, CV_32F or CV_64F.
  - kernel（object wrapped）: structuring element used for dilation; if elemenat=Mat(), a 3 x 3 rectangular structuring element is used. Kernel can be created using #getStructuringElement
  - anchor（object wrapped）: position of the anchor within the element; default value (-1, -1) means that the anchor is at the element center.
  - iterations: number of times dilation is applied.
  - borderType: pixel extrapolation method, see #BorderTypes. #BORDER_WRAP is not suported.
  - borderValue（object wrapped）: border value in case of a constant border
- returns:

  - dst（object wrapped）: output image of the same size and type as src.

##### findContours：寻找轮廓


```javascript
function findContours(src: Object, mode: Number, method: Number): contours: Object
```

- Parameters：

  - src（object wrapped）: Source, an 8-bit single-channel image. Non-zero pixels are treated as 1’s. Zero pixels remain 0’s, so the image is treated as binary .

  - mode: Contour retrieval mode, see #RetrievalModes
  - method: Contour approximation method, see #ContourApproximationModes

- returns:

  - contours（object wrapped）: Detected contours. Each contour is stored as a vector of points (e.g. std::vector< std::vector< cv::Point> >).


##### contourArea：计算轮廓的面积

```javascript
function contourArea(src: Object): area: Number
```

- Parameters：
  - src（object wrapped）: Input vector of 2D points (contour vertices), stored in std::vector or Mat. .

- returns:
  - area：area of a contour

##### getAreaMaxContour：寻找最大轮廓（自定义函数）

```javascript
function getAreaMaxContour(contours: Object,threshold: Number): max_contour: Object
```

- Parameters：
  - contours（object wrapped）
  - threshold（float）：min contour counts

- returns:
  - max_contour（object wrapped ）：biggest contour in contours

##### minEnclosingCircle：寻找包含二维点集的最小圆

```javascript
function jsMinEnclosingCircle(points: Object): location: Object
```

- Parameters：
  - points: Input vector of 2D points, stored in std::vector<> or Mat

- returns:
  - location（object wrapped ）：properties include : x, y, r

##### imDecode：从内存缓冲区中读取图片

```javascript
function imDecode(size: Number, ptr: Number): image: Object
```

- Parameters：

  - size
  - ptr
- returns:

  - dst（object wrapped）: Destination image

用法（摄像头）：

```javascript
var ph = cm.capture(480,320);
var cmsize = ph.size;
var cmptr = ph.buffer;
var src = opcv.imDecode(cmsize,cmptr);
```

## 动态库 - libmarker_detection.node

### 类 - MarkerDetection

#### 枚举

##### PREDEFINED_DICTIONARY_NAME

#### 类方法

##### detectMarkers：标记检测

```javascript
function detectMarkers(filename_in: String, filename_out: String, dict: Number)
```

- Parameters：

  - filename_in：待检测图片路径

  - filename_out：检测完成图片输出路径

  - dict：字典类型，见枚举PREDEFINED_DICTIONARY_NAME

##### poseEstimation：姿态检测

```javascript
function poseEstimation(filename_in: String, filename_out: String, dict: i32, camera_params: String)
```

- Parameters：

  - filename_in：待检测图片路径

  - filename_out：检测完成图片输出路径

  - dict：字典类型，见枚举PREDEFINED_DICTIONARY_NAME
  - camera_params：相机参数yaml/yml文件路径
