use macroex::*;
use macroex_extras::*;
use quote::quote;

macro_rules! test_color {
    ($tt: tt == $color: tt) => {
        let Rgba(color) = Rgba::<[u8; 4]>::from_one(quote!($tt).into_iter().next().unwrap()).unwrap();
        assert_eq!(color,  $color)
    };
}

#[test]
pub fn main(){
    test_color!([1,2,3] == [1,2,3,255]);
    test_color!([1,2,3,4] == [1,2,3,4]);
    test_color!([1;3] == [1,1,1,255]);
    test_color!([1;4] == [1,1,1,1]);
    test_color!([0.0, 1.0, 1.0] == [0, 255, 255, 255]);
    test_color!([0.0, 1.0, 1.0, 0.0] == [0, 255, 255, 0]);
    test_color!(0x010203 == [1,2,3,255]);
    test_color!(0x01020304 == [1,2,3,4]);
    test_color!("010203" == [1,2,3,255]);
    test_color!("01020304" == [1,2,3,4]);
    test_color!("#010203" == [1,2,3,255]);
    test_color!("#01020304" == [1,2,3,4]);
    test_color!(transparent == [0,0,0,0]);
    test_color!(black == [0,0,0,255]);
    test_color!(white == [255,255,255,255]);
    test_color!(red == [255,0,0,255]);
    test_color!(cyan == [0,255,255,255]);
    test_color!(green == [0,128,0,255]);
    test_color!(Gray700 == [55, 65, 81,255]);
    test_color!(Indigo50 == [238, 242, 255,255]);
    test_color!(Fuchsia300 == [240, 171, 252,255]);
    test_color!(Lime950 == [26, 46, 5,255]);
    test_color!(Amber500 == [245, 158, 11,255]);
}

macro_rules! test_colorf {
    ($tt: tt == $color: tt) => {
        let Rgba(color) = Rgba::<[f32; 4]>::from_one(quote!($tt).into_iter().next().unwrap()).unwrap();
        assert_eq!(color,  $color)
    };
}


#[test]
pub fn mainf(){
    test_colorf!([0.1,0.2,0.3] == [0.1,0.2,0.3,1.0]);
    test_colorf!([0.1,0.2,0.3,0.4] == [0.1,0.2,0.3,0.4]);
    test_colorf!([0.1;3] == [0.1,0.1,0.1,1.0]);
    test_colorf!([0.1;4] == [0.1,0.1,0.1,0.1]);
    test_colorf!([255,0,255] == [1.0,0.0,1.0,1.0]);
    test_colorf!([1,0,1,0] == [1.0,0.0,1.0,0.0]);
}
