use crate::vision8b::license_plate::detect_license_plate;
use crate::vision8b::dice_count::count_eyes;

#[test]
fn test_auto_1(){
    assert_eq!(detect_license_plate("auto1.jpg"), "rv-429-x");
}

#[test]
fn test_auto_2(){
    assert_eq!(detect_license_plate("auto2.jpg"), "x-999-xx");
}

#[test]
fn test_auto_3(){
    assert_eq!(detect_license_plate("auto3.jpg"), "37-jtk-9");
}

#[test]
fn test_auto_4(){
    assert_eq!(detect_license_plate("auto4.jpg"), "69-sx-sx");
}

#[test]
fn test_dice(){
    assert_eq!(count_eyes("13ogen.jpg"), 13);
}

