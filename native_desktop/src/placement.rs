//! Logical screen-coordinate placement from a tray activation hint.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Edge {
    Left,
    Right,
    Top,
    Bottom,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Placement {
    pub horizontal: Edge,
    pub vertical: Edge,
    pub horizontal_margin: i32,
    pub vertical_margin: i32,
}
/// Points and monitor rectangles use logical coordinates; do not rescale them.
pub fn place(monitor: Rect, point: (i32, i32), size: (i32, i32)) -> Option<Placement> {
    let (x, y) = (
        i64::from(point.0) - i64::from(monitor.x),
        i64::from(point.1) - i64::from(monitor.y),
    );
    let (width, height) = (i64::from(monitor.width), i64::from(monitor.height));
    let (panel_width, panel_height) = (i64::from(size.0), i64::from(size.1));
    if panel_width <= 0
        || panel_height <= 0
        || width < panel_width
        || height < panel_height
        || x < 0
        || y < 0
        || x >= width
        || y >= height
    {
        return None;
    }
    let distances = [
        (x, Edge::Left),
        (width - 1 - x, Edge::Right),
        (y, Edge::Top),
        (height - 1 - y, Edge::Bottom),
    ];
    let nearest = distances
        .into_iter()
        .min_by_key(|(distance, _)| *distance)?
        .1;
    let left = (x - panel_width / 2).clamp(0, width - panel_width);
    let top = (y - panel_height / 2).clamp(0, height - panel_height);
    let (left, top) = match nearest {
        Edge::Left => (0, top),
        Edge::Right => (width - panel_width, top),
        Edge::Top => (left, 0),
        Edge::Bottom => (left, height - panel_height),
    };
    let (horizontal, horizontal_margin) = if left <= width - panel_width - left {
        (Edge::Left, left)
    } else {
        (Edge::Right, width - panel_width - left)
    };
    let (vertical, vertical_margin) = if top <= height - panel_height - top {
        (Edge::Top, top)
    } else {
        (Edge::Bottom, height - panel_height - top)
    };
    Some(Placement {
        horizontal,
        vertical,
        horizontal_margin: i32::try_from(horizontal_margin).ok()?,
        vertical_margin: i32::try_from(vertical_margin).ok()?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn given_each_tray_edge_and_monitor_origin_when_placed_then_logical_panel_stays_inside_target_monitor(
    ) {
        // Arrange / Act / Assert
        for card_height in [170, 219, 336, 497] {
            for (origin_x, origin_y) in [(0, 0), (-1920, 0), (1920, -1080), (i32::MIN, i32::MIN)] {
                let monitor = Rect {
                    x: origin_x,
                    y: origin_y,
                    width: 1920,
                    height: 1080,
                };
                for (point, edge) in [
                    ((0, 540), Edge::Left),
                    ((1919, 540), Edge::Right),
                    ((960, 0), Edge::Top),
                    ((960, 1079), Edge::Bottom),
                ] {
                    let result = place(
                        monitor,
                        (origin_x + point.0, origin_y + point.1),
                        (420, card_height),
                    )
                    .unwrap();
                    let left = if result.horizontal == Edge::Left {
                        result.horizontal_margin
                    } else {
                        1920 - 420 - result.horizontal_margin
                    };
                    let top = if result.vertical == Edge::Top {
                        result.vertical_margin
                    } else {
                        1080 - card_height - result.vertical_margin
                    };
                    assert!(
                        left >= 0 && top >= 0 && left + 420 <= 1920 && top + card_height <= 1080
                    );
                    match edge {
                        Edge::Left => assert_eq!(left, 0),
                        Edge::Right => assert_eq!(left, 1500),
                        Edge::Top => assert_eq!(top, 0),
                        Edge::Bottom => assert_eq!(top, 1080 - card_height),
                    }
                }
            }
        }
    }
    #[test]
    fn given_signed_monitor_origins_and_panel_dimensions_when_placed_then_every_margin_stays_within_logical_monitor(
    ) {
        // Arrange: each activation coordinate belongs to the selected
        // monitor in logical pixels. Reconstruct the absolute card rectangle
        // from its returned edge and margin rather than trusting a label.
        for (width, height) in [(50, 30), (360, 250), (1920, 1080)] {
            for (origin_x, origin_y) in [(0, 0), (-1300, 400), (i32::MAX - 2000, i32::MIN + 1000)] {
                let monitor = Rect {
                    x: origin_x,
                    y: origin_y,
                    width,
                    height,
                };
                for size in [(1, 1), (20, 15), (width / 2, height / 2), (width, height)] {
                    for x in [0, 1, width / 2, width - 2, width - 1] {
                        for y in [0, 1, height / 2, height - 2, height - 1] {
                            let result =
                                place(monitor, (origin_x + x, origin_y + y), size).unwrap();
                            let left = if result.horizontal == Edge::Left {
                                result.horizontal_margin
                            } else {
                                width - size.0 - result.horizontal_margin
                            };
                            let top = if result.vertical == Edge::Top {
                                result.vertical_margin
                            } else {
                                height - size.1 - result.vertical_margin
                            };
                            assert!(
                                left >= 0
                                    && top >= 0
                                    && left + size.0 <= width
                                    && top + size.1 <= height,
                                "monitor={monitor:?}, point=({x},{y}), card={size:?}, placement={result:?}"
                            );
                            let distances = [
                                (x, Edge::Left),
                                (width - 1 - x, Edge::Right),
                                (y, Edge::Top),
                                (height - 1 - y, Edge::Bottom),
                            ];
                            let nearest = distances
                                .into_iter()
                                .min_by_key(|(distance, _)| *distance)
                                .unwrap()
                                .1;
                            match nearest {
                                Edge::Left => assert_eq!(left, 0),
                                Edge::Right => assert_eq!(left + size.0, width),
                                Edge::Top => assert_eq!(top, 0),
                                Edge::Bottom => assert_eq!(top + size.1, height),
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn given_equal_edge_distances_and_signed_coordinate_extremes_when_placed_then_ties_are_stable_and_offscreen_is_rejected(
    ) {
        let monitor = Rect {
            x: -200,
            y: 300,
            width: 101,
            height: 101,
        };
        // Arrange/Act/Assert: when distances tie, the documented
        // [left, right, top, bottom] comparison order is deterministic.
        for (x, y, pinned) in [
            (0, 0, Edge::Left),
            (100, 0, Edge::Right),
            (0, 100, Edge::Left),
            (100, 100, Edge::Right),
            (50, 0, Edge::Top),
            (50, 100, Edge::Bottom),
            (1, 1, Edge::Left),
            (99, 99, Edge::Right),
        ] {
            let result = place(monitor, (monitor.x + x, monitor.y + y), (20, 30)).unwrap();
            let left = if result.horizontal == Edge::Left {
                result.horizontal_margin
            } else {
                101 - 20 - result.horizontal_margin
            };
            let top = if result.vertical == Edge::Top {
                result.vertical_margin
            } else {
                101 - 30 - result.vertical_margin
            };
            match pinned {
                Edge::Left => assert_eq!(left, 0, "{x},{y}"),
                Edge::Right => assert_eq!(left, 81, "{x},{y}"),
                Edge::Top => assert_eq!(top, 0, "{x},{y}"),
                Edge::Bottom => assert_eq!(top, 71, "{x},{y}"),
            }
        }

        let extreme = Rect {
            x: i32::MAX - 10,
            y: i32::MIN,
            width: 11,
            height: 11,
        };
        let legal = place(extreme, (i32::MAX, i32::MIN + 10), (11, 11)).unwrap();
        assert_eq!((legal.horizontal_margin, legal.vertical_margin), (0, 0));
        for point in [
            (i32::MIN, i32::MIN),
            (i32::MAX - 11, i32::MIN),
            (i32::MAX, i32::MAX),
        ] {
            assert_eq!(place(extreme, point, (1, 1)), None);
        }
        for size in [(0, 1), (1, 0), (-1, 10), (12, 11), (11, 12)] {
            assert_eq!(place(extreme, (i32::MAX, i32::MIN), size), None);
        }
    }

    #[test]
    fn given_invalid_sizes_points_or_extreme_coordinates_when_placed_then_no_overflow_or_guessed_output_is_used(
    ) {
        // Arrange / Act / Assert
        for (width, height, size) in [
            (0, 1080, (420, 497)),
            (1920, 0, (420, 497)),
            (-1, 1080, (420, 497)),
            (419, 1080, (420, 497)),
            (1920, 496, (420, 497)),
            (1920, 1080, (0, 497)),
            (1920, 1080, (420, -1)),
        ] {
            assert_eq!(
                place(
                    Rect {
                        x: 0,
                        y: 0,
                        width,
                        height
                    },
                    (0, 0),
                    size
                ),
                None
            );
        }
        let rect = Rect {
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
        };
        for point in [(-1, 0), (0, -1), (1920, 0), (0, 1080), (i32::MIN, i32::MAX)] {
            assert_eq!(place(rect, point, (420, 497)), None);
        }
        assert!(place(
            Rect {
                x: i32::MIN,
                y: i32::MIN,
                width: i32::MAX,
                height: i32::MAX
            },
            (-1, -1),
            (420, 497)
        )
        .is_none());
        let exact = place(
            Rect {
                x: 0,
                y: 0,
                width: 420,
                height: 497,
            },
            (419, 496),
            (420, 497),
        )
        .unwrap();
        assert_eq!((exact.horizontal_margin, exact.vertical_margin), (0, 0));
    }
}
