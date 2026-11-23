
use fafs::fractal::sierpinski::SierpinskiNode;

#[test]
fn test_sierpinski_tree() {
    let mut root = SierpinskiNode::new(1, 0);

    let child = SierpinskiNode::new(2, 1);

    root.add_child(child).unwrap();

    assert!(root.find(2).is_some());
    assert!(root.find(3).is_none());
}

#[test]
fn test_invalid_child_depth() {
    let mut root = SierpinskiNode::new(1, 2);
    let child = SierpinskiNode::new(2, 1);

    assert!(root.add_child(child).is_err());
}


