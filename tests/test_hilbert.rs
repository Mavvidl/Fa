
use fafs::fractal::hilbert::HilbertCurve;
use fafs::fractal::Point2D;

#[test]
fn test_hilbert_encode_decode() {
    let curve = HilbertCurve::new(4).unwrap();

    for x in 0..16 {
        for y in 0..16 {
            let point = Point2D { x, y };

            let encoded = curve.encode(point).unwrap();
            let decoded = curve.decode(encoded).unwrap();

            assert_eq!(point, decoded);
        }
    }
}

#[test]
fn test_invalid_hilbert_order() {
    assert!(HilbertCurve::new(0).is_err());
    assert!(HilbertCurve::new(17).is_err());
}

