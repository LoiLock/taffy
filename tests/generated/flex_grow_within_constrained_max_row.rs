#[test]
fn flex_grow_within_constrained_max_row() {
    let layout = stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(200.0000), ..Default::default() },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                max_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                children: vec![
                    stretch::style::Node {
                        flex_shrink: 1.0000,
                        flex_basis: stretch::style::Dimension::Points(100.0000),
                        ..Default::default()
                    },
                    stretch::style::Node {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(50.0000),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 200.0000);
    assert_eq!(layout.size.height, 100.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 100.0000);
    assert_eq!(layout.children[0].size.height, 100.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);

    assert_eq!(layout.children[0].children[0].size.width, 67.0000);
    assert_eq!(layout.children[0].children[0].size.height, 100.0000);
    assert_eq!(layout.children[0].children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].children[0].location.y, 0.0000);

    assert_eq!(layout.children[0].children[1].size.width, 33.0000);
    assert_eq!(layout.children[0].children[1].size.height, 100.0000);
    assert_eq!(layout.children[0].children[1].location.x, 67.0000);
    assert_eq!(layout.children[0].children[1].location.y, 0.0000);
}
