const a = require("./libjsopencv.node");

var opcv = new a.OpenCv();

console.log("[before imRead]");
var src = opcv.imRead("zuqiu.jpg");
console.log("[after imRead]");

var Chest_Ball_hsv = opcv.jsCvtColor(src,40,0);
opcv.imWrite("./out/1cvtcolor.jpg",Chest_Ball_hsv);

var size = opcv.createSize(3,3);
var Chest_Ball_gaus = opcv.jsGaussianBlur(Chest_Ball_hsv, size,2.0,0.0,4);
opcv.imWrite("./out/2gaus.jpg",Chest_Ball_gaus);

var lower = opcv.createScalar(10.0,100.0,100.0,0.0);
var upper = opcv.createScalar(180.0,180.0,180.0,255.0);
var Chest_Ball_Imask = opcv.jsInRange(Chest_Ball_gaus,lower,upper);
opcv.imWrite("./out/3inrange.jpg",Chest_Ball_Imask);

var point = opcv.createPoint(-1,-1);
var kernel = opcv.createKernel(0,size,point);
var Chest_Ball_erode = opcv.jsErode(Chest_Ball_Imask,kernel,point,1,0,lower);
// var Chest_Ball_erode = opcv.jsErode(Chest_Ball_Imask,kernel,point,1,0);
opcv.imWrite("./out/4erode.jpg",Chest_Ball_erode);

var Chest_Ball_dilate = opcv.jsDilate(Chest_Ball_erode,kernel,point,1,0,lower);
// var Chest_Ball_dilate = opcv.jsDilate(Chest_Ball_erode,kernel,point,1,0);
opcv.imWrite("./out/5dilate.jpg",Chest_Ball_dilate);


var cnts2 = opcv.findContours(Chest_Ball_dilate,0,2);

var max_contour = opcv.getAreaMaxContour(cnts2,10.0);

var coordinate = opcv.jsMinEnclosingCircle(max_contour);

console.log(coordinate);