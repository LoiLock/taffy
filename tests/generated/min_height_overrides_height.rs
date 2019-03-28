#[test]
fn min_height_overrides_height() {
    let layout = stretch::compute(
        &stretch::style::Node {
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(50.0000),
                    ..Default::default()
                },
                min_size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 0.0000);
    assert_eq!(layout.size.height, 100.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 0.0000);
    assert_eq!(layout.children[0].size.height, 100.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);
}
