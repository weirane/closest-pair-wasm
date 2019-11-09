#![feature(test)]
extern crate test;

mod point;

pub use crate::point::{ParsePointError, Point};
use std::cmp::Ordering::Equal;
use std::f64::{EPSILON as eps, INFINITY as inf};

/// Calculates the closest pair of a slice of points, returns the closest
/// distance and the two points.
pub fn closest_pair(points: &[Point]) -> (f64, Point, Point) {
    let mut points = points.to_vec();
    let mut points_ysort = points.clone();
    points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(Equal));
    points_ysort.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap_or(Equal));

    closest_pair_inner(&points, &points_ysort)
}

fn closest_pair_inner(points: &[Point], points_ysort: &[Point]) -> (f64, Point, Point) {
    let mid = match points.len() {
        0 => unreachable!("Empty slice"),
        1 => return (inf, points[0], points[0]),
        2 => return (points[0].distance(&points[1]), points[0], points[1]),
        3 => {
            return *[
                (points[0].distance(&points[1]), points[0], points[1]),
                (points[1].distance(&points[2]), points[1], points[2]),
                (points[0].distance(&points[2]), points[0], points[2]),
            ]
            .iter()
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Equal))
            .unwrap();
        }
        n => n / 2,
    };
    let mid_line = points[mid].x;
    let (xleft, xright) = (&points[..mid], &points[mid..]);
    let (yleft, yright): (Vec<_>, Vec<_>) = points_ysort
        .iter()
        .partition(|&p| p.x < mid_line || ((p.x - mid_line).abs() < eps && xleft.contains(p)));
    let left = closest_pair_inner(&xleft, &yleft);
    let right = closest_pair_inner(&xright, &yright);
    let (mut min_dist, mut min_p0, mut min_p1) = if left.0 < right.0 { left } else { right };

    let points_ysort: Vec<_> = points_ysort
        .iter()
        .filter(|p| (p.x - mid_line).abs() <= min_dist)
        .collect();

    for (i, &p0) in points_ysort.iter().enumerate() {
        for &p in points_ysort[i + 1..].iter().take(7) {
            let dist = p0.distance(&p);
            if dist < min_dist {
                min_dist = dist;
                min_p0 = *p0;
                min_p1 = *p;
            }
        }
    }

    (min_dist, min_p0, min_p1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::str::FromStr;

    #[test]
    #[should_panic]
    fn cp_trivial() {
        closest_pair(&[]);
    }

    #[test]
    fn cp_one_line() {
        let pts: Vec<Point> = [(1, 0), (1, 3), (1, 4), (1, 7)]
            .iter()
            .map(|&t| t.into())
            .collect();
        let rslt = closest_pair(&pts);
        assert_eq!(rslt, (1., (1, 3).into(), (1, 4).into()));

        let pts: Vec<Point> = [(0, 1), (3, 1), (4, 1), (7, 1)]
            .iter()
            .map(|&t| t.into())
            .collect();
        let rslt = closest_pair(&pts);
        assert_eq!(rslt, (1., (3, 1).into(), (4, 1).into()));
    }

    fn cp_bench(
        file: &str,
        b: &mut test::Bencher,
        result: Option<(f64, Point, Point)>,
    ) -> Result<(), Box<dyn Error>> {
        let points: Vec<Point> = BufReader::new(File::open(file)?)
            .lines()
            .map(|li| Ok(Point::from_str(&li?)?))
            .collect::<Result<_, Box<dyn Error>>>()?;

        let mut rslt = Default::default();
        b.iter(|| rslt = closest_pair(&points));

        if let Some(result) = result {
            assert_eq!(rslt, result);
        }

        Ok(())
    }

    #[bench]
    fn cp_size_20(b: &mut test::Bencher) {
        let expected = (
            4.174759894242652,
            Point {
                x: 5.635107150827567,
                y: 84.39338071000523,
            },
            Point {
                x: 8.580038586790051,
                y: 87.35243444592435,
            },
        );
        assert!(cp_bench("./data/in1.txt", b, Some(expected)).is_ok());
    }

    #[bench]
    fn cp_size_50(b: &mut test::Bencher) {
        let expected = (
            0.9998515096750291,
            Point {
                x: 24.858920394659467,
                y: 81.69074274900228,
            },
            Point {
                x: 24.51804919124837,
                y: 82.63069480314361,
            },
        );
        assert!(cp_bench("./data/in2.txt", b, Some(expected)).is_ok());
    }

    #[bench]
    fn cp_size_100(b: &mut test::Bencher) {
        let expected = (
            0.33648585196923414,
            Point {
                x: 40.41374919465394,
                y: 55.542423614213334,
            },
            Point {
                x: 40.71088677155328,
                y: 55.70032228099001,
            },
        );
        assert!(cp_bench("./data/in3.txt", b, Some(expected)).is_ok());
    }

    #[bench]
    fn cp_size_200(b: &mut test::Bencher) {
        let expected = (
            0.6671770195228724,
            Point {
                x: 118.52192038143514,
                y: 92.52394268631166,
            },
            Point {
                x: 118.39024778636384,
                y: 93.17799734999552,
            },
        );
        assert!(cp_bench("./data/in4.txt", b, Some(expected)).is_ok());
    }

    #[bench]
    fn cp_size_1000(b: &mut test::Bencher) {
        let expected = (
            0.011585729954339841,
            Point {
                x: 81.0284365760448,
                y: 30.531025977209403,
            },
            Point {
                x: 81.03640405283963,
                y: 30.53943718711635,
            },
        );
        assert!(cp_bench("./data/in5.txt", b, Some(expected)).is_ok());
    }

    #[bench]
    fn cp_size_10000(b: &mut test::Bencher) {
        let expected = (
            0.008666344541381515,
            Point {
                x: 42.073257309384495,
                y: 40.54991392609082,
            },
            Point {
                x: 42.07209894184973,
                y: 40.55850250643709,
            },
        );
        assert!(cp_bench("./data/in6.txt", b, Some(expected)).is_ok());
    }

    #[bench]
    fn cp_size_50000(b: &mut test::Bencher) {
        let expected = (
            0.014114898678767343,
            Point {
                x: 347.9920466250392,
                y: 511.16886178108024,
            },
            Point {
                x: 347.99550872637883,
                y: 511.15517806013065,
            },
        );
        assert!(cp_bench("./data/in7.txt", b, Some(expected)).is_ok());
    }
}
