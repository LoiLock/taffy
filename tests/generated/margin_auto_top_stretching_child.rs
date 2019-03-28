#[test]
fn margin_auto_top_stretching_child() {
    let layout = stretch::compute(
        &stretch::style::Node {
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node {
                    flex_grow: 1.0000,
                    flex_shrink: 1.0000,
                    flex_basis: stretch::style::Dimension::Percent(0.0000),
                    margin: stretch::geometry::Rect { top: stretch::style::Dimension::Auto, ..Default::default() },
                    ..Default::default()
                },
                stretch::style::Node {
                    size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(50.0000),
                        height: stretch::style::Dimension::Points(50.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 200.0000);
    assert_eq!(layout.size.height, 200.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 150.0000);
    assert_eq!(layout.children[0].size.height, 0.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 200.0000);

    assert_eq!(layout.children[1].size.width, 50.0000);
    assert_eq!(layout.children[1].size.height, 50.0000);
    assert_eq!(layout.children[1].location.x, 150.0000);
    assert_eq!(layout.children[1].location.y, 75.0000);
}
