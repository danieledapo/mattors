//! A simple [K-Means](https://en.wikipedia.org/wiki/K-means_clustering)
//! implementation.

extern crate num;

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::iter;

use geo::Point;

/// Cluster the given set of points in at most k clusters. If k is greater or
/// equal than the set of unique points then all the input points are returned.
/// Note that K-Means doesn't return the optimal solution and in fact it's
/// totally possible that the clusters contain less than k clusters. To avoid
/// that try to increase the number of max_iterations and/or shuffle the points.
pub fn kmeans<T, I>(points: I, k: usize, max_iterations: usize) -> HashMap<Point<T>, Vec<Point<T>>>
where
    T: num::Num + Ord + Copy + Hash + From<u8> + ::std::fmt::Debug,
    I: IntoIterator<Item = Point<T>>,
{
    if k == 0 {
        return HashMap::new();
    }

    // first dedup points in an hashset and then store them in a vec.
    let points = points
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    if points.len() <= k {
        return points.into_iter().map(|p| (p, vec![p])).collect();
    }

    let mut clusters = iter::repeat(vec![]).take(k).collect::<Vec<_>>();

    // don't want to pickup random values, the caller can always shuffle the
    // array to achieve the same effect.
    let mut pivots = (0..k)
        .map(|i| points[i * points.len() / k])
        .collect::<Vec<_>>();

    for _ in 0..max_iterations {
        for cluster in &mut clusters {
            cluster.clear();
        }

        for point in &points {
            let closest_i = pivots
                .iter()
                .enumerate()
                .min_by_key(|(i, p)| (p.squared_dist::<T>(point), clusters[*i].len()))
                .unwrap()
                .0;

            clusters[closest_i].push(*point);
        }

        let pivot_changed = update_pivots(&mut pivots, &clusters, &points, k);
        if !pivot_changed {
            break;
        }
    }

    pivots
        .into_iter()
        .zip(clusters.into_iter())
        .filter(|(_, c)| !c.is_empty())
        .collect()
}

fn update_pivots<T>(
    pivots: &mut [Point<T>],
    clusters: &[Vec<Point<T>>],
    points: &[Point<T>],
    k: usize,
) -> bool
where
    T: num::Num + Copy + From<u8> + Debug,
{
    let mut pivot_changed = false;

    for (i, pivot) in pivots.iter_mut().enumerate() {
        let new_pivot = if clusters[i].is_empty() {
            // if the cluster for this pivot is empty pickup a point that's
            // different from the current pivot and hope for the best.
            let new_pivot_ix = i * points.len() / k;
            let mut p = points[new_pivot_ix];

            if p == *pivot {
                // since the points were deduped, if p is the pivot the next
                // point is definitely not.
                p = points[(new_pivot_ix + 1) % points.len()];
                debug_assert_ne!(p, *pivot);
            }

            p
        } else {
            avg_point(&clusters[i])
        };

        if new_pivot != *pivot {
            pivot_changed = true;
        }

        *pivot = new_pivot;
    }

    pivot_changed
}

fn avg_point<T>(cluster: &[Point<T>]) -> Point<T>
where
    T: num::Num + Copy + From<u8>,
{
    let (sum_x, sum_y, len) = cluster.iter().fold(
        (T::from(0_u8), T::from(0_u8), T::from(0_u8)),
        |(sum_x, sum_y, len), pt| (sum_x + pt.x, sum_y + pt.y, len + T::from(1)),
    );

    Point::new(sum_x / len, sum_y / len)
}

#[cfg(test)]
mod tests {
    use super::kmeans;

    extern crate proptest;

    use proptest::prelude::*;

    use geo::Point;

    type PointK = Point<i32>;

    fn points_and_k() -> impl Strategy<Value = (Vec<PointK>, usize)> {
        prop::collection::vec((-255_i32..255, -255_i32..255), 1..100).prop_flat_map(|points| {
            let points = points
                .into_iter()
                .map(|(x, y)| Point::new(x, y))
                .collect::<Vec<_>>();
            let len = points.len();

            (Just(points), 0..len)
        })
    }

    proptest! {
        #![proptest_config(proptest::test_runner::Config::with_cases(500))]
        #[test]
        fn prop_kmeans_clusters_contains_closest_point(
            (points, k) in points_and_k()
        ) {
            _prop_kmeans_clusters_contains_closest_point(points, k)
        }
    }

    fn _prop_kmeans_clusters_contains_closest_point(points: Vec<PointK>, k: usize) {
        let clusters = kmeans(points.clone(), k, usize::max_value());
        assert!(clusters.len() <= k);

        for (pivot, cluster) in &clusters {
            assert!(!cluster.is_empty());

            for point in cluster {
                let closest_pivot = clusters.keys().min_by_key(|p| p.squared_dist::<i64>(point));
                assert!(closest_pivot.is_some());

                let closest_pivot = closest_pivot.unwrap();
                assert_eq!(
                    point.squared_dist::<i64>(closest_pivot),
                    point.squared_dist::<i64>(pivot)
                );
            }
        }
    }
}
