use rust_point::Point;

#[test]
fn test_distance() {
    let p1 = Point::new(1.0, 1.0);
    let p2 = Point::new(0.0, 0.0);
    let e = 0.001f32;
    let expected = 1.4142f32;
    assert!(p1.distance(&p2) - expected < e);
}

#[test]
fn test_move_to() {
    let mut p1 = Point::new(1.0, 1.0);
    p1.move_to(2.0, 2.0);
    assert_eq!(p1.x, 2.0);
    assert_eq!(p1.y, 2.0);
}

#[test]
fn test_print() {
    let p1 = Point::new(1.0, 1.0);
    let output = p1.print();
    assert_eq!(output, "Point [1 1]");
}
