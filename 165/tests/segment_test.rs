use num_rational::Rational64;
use lib::geometry::Segment;

#[test]
fn test_segment_intersect() {
    let mut two = Rational64::from_integer(2);

    // segments have true intersection point
    let s1 = Segment::new(0, 0, 4, 4);
    let s2 = Segment::new(0, 4, 4, 0);
    let result = s1.intersect(&s2);
    assert_eq!(result, (two, two, true));

    // segments have true intersection point, one segment is vertical
    let s1 = Segment::new(2, 0, 2, 8);
    let s2 = Segment::new(1, 1, 6, 6);
    let result = s1.intersect(&s2);
    assert_eq!(result, (two, two, true));

    // segments have no intersection point
    let s1 = Segment::new(0, 0, 4, 4);
    let s2 = Segment::new(0, 4, 1, 3);
    let result = s1.intersect(&s2);
    assert_eq!(result.2, false);

    // segments have are parallel (so, no true intersection point)
    let s1 = Segment::new(0, 0, 4, 4);
    let s2 = Segment::new(5,5, 7,7);
    let result = s1.intersect(&s2);
    assert_eq!(result.2, false);

    // segments have intersection point, which the end of one segment (no true intersection point)
    let s1 = Segment::new(0, 0, 4, 4);
    let s2 = Segment::new(4, 0, 2, 2);
    let result = s1.intersect(&s2);
    assert_eq!(result.2, false);


}
