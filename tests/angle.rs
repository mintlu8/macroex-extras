use macroex::*;
use macroex_extras::*;
use quote::quote;
use std::f32::consts::PI;

macro_rules! angle {
    ($angle: tt) => {
        {
            let Angle(angle) = Angle::from_one(quote!($angle).into_iter().next().unwrap()).unwrap();
            angle
        }
    };
}

#[test]
pub fn main(){
    assert_eq!(angle!(0), 0.0);
    assert_eq!(angle!(pi), PI);
    assert_eq!(angle!([pi]), PI);
    assert_eq!(angle!([2 pi]), 2.0 * PI);
    assert_eq!(angle!([180 deg]), PI);
    assert_eq!(angle!([-90 degrees]), -PI / 2.0);
    assert_eq!(angle!([4 rad]), 4.0);
    assert_eq!(angle!([-1.4 radians]), -1.4);
    assert_eq!(angle!([2/3 pi]), PI * 2.0 / 3.0);
    assert_eq!(angle!([pi/7]), PI / 7.0);

    let Angle(angle) = Angle::from_many(quote!(-12)).unwrap();
    assert_eq!(angle, -12.0);

}
