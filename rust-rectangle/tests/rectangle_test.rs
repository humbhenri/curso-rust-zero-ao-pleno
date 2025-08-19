use rust_rectangle::Rectangle;

#[test]
fn test_area() {
    let r = Rectangle::new(1.0, 1.0, 10.0, 10.0);
    assert_eq!(r.area(), 100.0);
}

#[test]
fn test_perimeter() {
    let r = Rectangle::new(1.0, 1.0, 10.0, 10.0);
    assert_eq!(r.perimeter(), 40.0);
}

#[test]
fn test_contains_point() {
    let r = Rectangle::new(1.0, 1.0, 10.0, 10.0);
    assert!(r.contains_point(2.0, 2.0));
    assert_eq!(r.contains_point(12.0, 1.0), false);
}

#[test]
fn test_print() {
    let r = Rectangle::new(1.0, 1.0, 10.0, 10.0);
    assert_eq!(r.print(), "Rectangle [1 1 10 10]");
}
