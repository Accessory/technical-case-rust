#[cfg(test)]
mod line_test {
    use technical_case_rust::utils::{line::Line, position::Position};

    #[test]
    fn test_intersections_horizontal1() {
        let left_zero = Position { x: -5, y: 0 };
        let right_zero = Position { x: 5, y: 0 };
        let right_bottom = Position { x: -5, y: -5 };
        let left_bottom = Position { x: 5, y: -5 };

        let l1 = Line {
            start: left_zero,
            end: right_zero,
        };
        let l2 = Line {
            start: right_bottom,
            end: left_bottom,
        };
        let result = l1.intersections(&l2);
        assert_eq!(0, result);
    }

    #[test]
    fn test_intersections_horizontal2() {
        let left_zero = Position { x: -5, y: 0 };
        let right_zero = Position { x: 5, y: 0 };

        let l1 = Line {
            start: left_zero,
            end: right_zero,
        };
        let l2 = Line {
            start: left_zero,
            end: right_zero,
        };
        let result = l1.intersections(&l2);
        assert_eq!(11, result);
    }

    #[test]
    fn test_intersections_horizontal3() {
        let left_zero = Position { x: -5, y: 0 };
        let right_zero = Position { x: 5, y: 0 };
        let right2_zero = Position { x: 10, y: 0 };

        let l1 = Line {
            start: left_zero,
            end: right_zero,
        };
        let l2 = Line {
            start: left_zero,
            end: right2_zero,
        };
        let result = l1.intersections(&l2);
        assert_eq!(11, result);
    }

    #[test]
    fn test_intersections_horizontal4() {
        let left_zero = Position { x: -5, y: 0 };
        let right_zero = Position { x: 5, y: 0 };
        let right_bottom = Position { x: 6, y: 0 };
        let left_bottom = Position { x: 15, y: 0 };

        let l1 = Line {
            start: left_zero,
            end: right_zero,
        };
        let l2 = Line {
            start: right_bottom,
            end: left_bottom,
        };
        let result = l1.intersections(&l2);
        assert_eq!(0, result);
    }

    #[test]
    fn testintersections_horizontal5() {
        let left_zero = Position { x: -5, y: 0 };
        let right_zero = Position { x: 5, y: 0 };
        let right_bottom = Position { x: 4, y: 0 };
        let left_bottom = Position { x: 15, y: 0 };

        let l1 = Line {
            start: left_zero,
            end: right_zero,
        };
        let l2 = Line {
            start: left_bottom,
            end: right_bottom,
        };
        let result = l1.intersections(&l2);
        assert_eq!(2, result);
    }

    #[test]
    fn testintersections_vertical1() {
        let top_zero = Position { x: 0, y: -5 };
        let bottom_zero = Position { x: 0, y: 5 };
        let top_one = Position { x: 1, y: -5 };
        let bottom_one = Position { x: 1, y: 5 };

        let l1 = Line {
            start: top_zero,
            end: bottom_zero,
        };
        let l2 = Line {
            start: top_one,
            end: bottom_one,
        };
        let result = l1.intersections(&l2);
        assert_eq!(0, result);
    }

    #[test]
    fn testintersections_vertical2() {
        let top_zero = Position { x: 0, y: -5 };
        let bottom_zero = Position { x: 0, y: 5 };
        let l1 = Line {
            start: top_zero,
            end: bottom_zero,
        };
        let l2 = Line {
            start: bottom_zero,
            end: top_zero,
        };
        let result = l1.intersections(&l2);
        assert_eq!(11, result);
    }

    #[test]
    fn testintersections_vertical3() {
        let top_zero = Position { x: 0, y: -5 };
        let bottom_zero = Position { x: 0, y: 5 };
        let bottom2_zero = Position { x: 0, y: 10 };

        let l1 = Line {
            start: top_zero,
            end: bottom_zero,
        };
        let l2 = Line {
            start: bottom_zero,
            end: bottom2_zero,
        };

        let result = l1.intersections(&l2);
        assert_eq!(1, result);
    }

    #[test]
    fn testintersections_vertical4() {
        let top_zero = Position { x: 0, y: -5 };
        let bottom_zero = Position { x: 0, y: 5 };

        let l1 = Line {
            start: top_zero,
            end: bottom_zero,
        };
        let l2 = Line {
            start: top_zero,
            end: bottom_zero,
        };

        let result = l1.intersections(&l2);
        assert_eq!(11, result);
    }

    #[test]
    fn testintersections_vertical5() {
        let top_zero = Position { x: 0, y: -5 };
        let bottom_zero = Position { x: 0, y: 5 };
        let top_one = Position { x: 0, y: 6 };
        let bottom_one = Position { x: 0, y: 165 };

        let l1 = Line {
            start: top_zero,
            end: bottom_zero,
        };
        let l2 = Line {
            start: bottom_one,
            end: top_one,
        };
        let result = l1.intersections(&l2);
        assert_eq!(0, result);
    }

    #[test]
    fn line_case() {
        let p1 = Position { x: 10000, y: 10000 };
        let p2 = Position { x: 10000, y: 1 };
        let p3 = Position { x: 10000, y: 0 };
        let p4 = Position {
            x: 10000,
            y: -99999,
        };

        let l1 = Line { start: p1, end: p2 };
        let l2 = Line { start: p3, end: p4 };
        let result = l1.intersections(&l2);
        assert_eq!(0, result);
    }

    #[test]
    fn intersection_case() {
        let p1 = Position { x: -10, y: 0 };
        let p2 = Position { x: 10, y: 0 };
        let p3 = Position { x: 0, y: -10 };
        let p4 = Position { x: 0, y: 10 };

        let l1 = Line { start: p1, end: p2 };
        let l2 = Line { start: p3, end: p4 };
        let result = l1.intersections(&l2);
        assert_eq!(1, result);
    }

    #[test]
    fn intersection_case2() {
        let p1 = Position { x: -4, y: 10 };
        let p2 = Position { x: 0, y: 10 };
        let p3 = Position { x: -5, y: 15 };
        let p4 = Position { x: -5, y: 5 };

        let l1 = Line { start: p1, end: p2 };
        let l2 = Line { start: p3, end: p4 };
        let result = l1.intersections(&l2);
        assert_eq!(0, result);
    }

    #[test]
    fn intersection_case3() {
        let p1 = Position { x: 15, y: 15 };
        let p2 = Position { x: 15, y: 24 };
        let p3 = Position { x: 15, y: 25 };
        let p4 = Position { x: 19, y: 25 };

        let l1 = Line { start: p1, end: p2 };
        let l2 = Line { start: p3, end: p4 };
        let result = l1.intersections(&l2);
        assert_eq!(0, result);
    }

    #[test]
    fn intersection_case4() {
        let p1 = Position { x: 20, y: 25 };
        let p2 = Position { x: 20, y: 21 };
        let p3 = Position { x: 10, y: 22 };
        let p4 = Position { x: 17, y: 22 };

        let l1 = Line { start: p1, end: p2 };
        let l2 = Line { start: p3, end: p4 };
        let result = l1.intersections(&l2);
        assert_eq!(0, result);
    }
}
