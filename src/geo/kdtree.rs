//! A simple(and probably inefficient) implementation of a [K-d
//! Tree](https://en.wikipedia.org/wiki/K-d_tree). Only 2D as of now.

extern crate num;

use std::cmp::{Ord, Ordering};
use std::collections::{BinaryHeap, VecDeque};

use geo::{Point, Rect};
use utils::{ksmallest_by_key, split_element_at, OrdWrapper};

/// The axis used to split the space at a given point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Axis {
    /// X axis.
    X,

    /// Y axis.
    Y,
}

/// Trait that allows to extract the axis value for a given axis from an entity
/// contained in the KdTree.
pub trait AxisValue {
    /// The value that will be returned by axis_value.
    type Value;

    /// Return the value for the given axis.
    fn axis_value(&self, axis: Axis) -> &Self::Value;
}

/// A [K-d Tree](https://en.wikipedia.org/wiki/K-d_tree).
#[derive(Debug, PartialEq)]
pub struct KdTree<T, V> {
    root: Option<Node<T, V>>,
    length: usize,
}

#[derive(Debug, PartialEq)]
struct Node<T, V> {
    axis: Axis,
    median: Point<T>,
    value: V,

    left: Option<Box<Node<T, V>>>,
    right: Option<Box<Node<T, V>>>,
}

/// Simple trait that allow to support range queries for multiple types of
/// shapes.
pub trait Range<T> {
    /// Return whether the given value is contained in the range.
    fn contains(&self, v: &T) -> bool;

    /// The type of the axis value.
    type AxisValue;

    /// Return the range that the axis values are in the given axis.
    fn axis_value_range(&self, axis: Axis) -> (Self::AxisValue, Self::AxisValue);
}

impl<T, V> Default for KdTree<T, V> {
    fn default() -> Self {
        KdTree {
            root: None,
            length: 0,
        }
    }
}

impl<T, V> KdTree<T, V>
where
    T: Copy + Ord,
    Point<T>: AxisValue<Value = T>,
{
    /// Create a new empty KdTree.
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if this kdtree is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return the length of this kdtree.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Build a KdTree from a vector of points. This method should be preferred
    /// over to add when the set of points doesn't change because it creates a
    /// tree that is often more balanced. The construction is a bit slower
    /// though.
    pub fn from_vector(points: Vec<(Point<T>, V)>) -> Self {
        let mut kdtree = KdTree::default();

        let mut ranges = VecDeque::new();
        ranges.push_back((points, Axis::X));

        while let Some((mut points, axis)) = ranges.pop_front() {
            if points.is_empty() {
                continue;
            }

            let mid = points.len() / 2;

            // this is actually partitioning data at the median
            ksmallest_by_key(&mut points, mid, |(pt, _val)| {
                (*pt.axis_value(axis), *pt.axis_value(axis.next()))
            }).unwrap();

            let (left, elem, right) = split_element_at(points, mid);

            let (new_point, new_val) = elem.unwrap();
            kdtree.add(new_point, new_val);

            ranges.push_back((left, axis.next()));
            ranges.push_back((right, axis.next()));
        }

        kdtree
    }

    /// Add a point to this KdTree. Note that this could unbalance the tree,
    /// prefer from_vector if the set of points is not dynamic.
    pub fn add(&mut self, point: Point<T>, value: V) -> Option<V> {
        if self.root.is_none() {
            self.root = Some(Node::new(point, value, Axis::X));
            self.length = 1;

            return None;
        }

        let root_node = self.root.as_mut().unwrap();

        let old_value = root_node.add(point, value);
        if old_value.is_none() {
            self.length += 1;
        }

        old_value
    }

    /// Return an iterator over all the elements of the tree.
    pub fn iter<'a>(self: &'a Self) -> KdTreeIter<'a, T, V> {
        KdTreeIter {
            nodes: self.root.as_ref().map(|r| vec![r]).unwrap_or_else(Vec::new),
        }
    }

    /// Return all the points that are in the given range.
    pub fn in_range_iter<'s, 'r, R>(self: &'s Self, range: &'r R) -> InRangeIter<'s, 'r, T, V, R>
    where
        R: Range<Point<T>, AxisValue = T>,
    {
        InRangeIter {
            nodes: self.root.as_ref().map(|r| vec![r]).unwrap_or_else(Vec::new),
            range,
        }
    }

    /// Return the nearest neighbor to the given point.
    pub fn nearest_neighbor(&self, point: Point<T>) -> Option<(&Point<T>, &V)>
    where
        T: num::Num + From<u8> + ::std::fmt::Debug,
        V: ::std::fmt::Debug,
        i64: From<T>,
    {
        self.nearest_neighbors(point, 1).into_iter().next()
    }

    /// Return, at most, the k nearest neighbors to the given point.
    pub fn nearest_neighbors(&self, point: Point<T>, k: usize) -> Vec<(&Point<T>, &V)>
    where
        T: num::Num + From<u8> + ::std::fmt::Debug,
        V: ::std::fmt::Debug,
        i64: From<T>,
    {
        if self.root.is_none() || k == 0 {
            return vec![];
        }

        let root_node = self.root.as_ref().unwrap();
        let mut nodes = vec![root_node];

        let mut neighbors = BinaryHeap::new();
        let mut min_dist = i64::max_value();

        while let Some(node) = nodes.pop() {
            let node_dist = node.median.squared_dist(&point);

            min_dist = min_dist.min(node_dist);
            neighbors.push(OrdWrapper::new(node, node_dist));

            if neighbors.len() > k {
                neighbors.pop();
            }

            // since nodes is a stack, push first the data that must be computed
            // last. In this case we want to perform the wrong path after we
            // checked the good one.

            let (next, candidate) = match node.cmp_to_point_value(point) {
                Ordering::Less | Ordering::Equal => (&node.left, &node.right),
                Ordering::Greater => (&node.right, &node.left),
            };

            if let Some(candidate_node) = candidate {
                // check if there could be intersection on the wrong side of the
                // plane. This is done by checking whether the candidate point's
                // axis is still reachable within the current minimum distance.
                let split_plane = i64::from(*node.median.axis_value(node.axis));
                let plane_dist = i64::from(*point.axis_value(node.axis)) - split_plane;
                let plane_dist2 = plane_dist * plane_dist;

                if plane_dist2 <= min_dist {
                    nodes.push(candidate_node);
                }
            }

            if let Some(next_node) = next {
                nodes.push(next_node);
            }
        }

        neighbors
            .into_sorted_vec()
            .into_iter()
            .map(|ow| {
                let (node, _) = ow.into();
                (&node.median, &node.value)
            })
            .collect()
    }
}

impl<T, V> Node<T, V>
where
    T: Copy + Ord,
    Point<T>: AxisValue<Value = T>,
{
    fn new(pt: Point<T>, value: V, axis: Axis) -> Self {
        Node {
            median: pt,
            axis,
            value,
            left: None,
            right: None,
        }
    }

    fn add(&mut self, point: Point<T>, value: V) -> Option<V> {
        if point == self.median {
            let old_value = ::std::mem::replace(&mut self.value, value);
            return Some(old_value);
        }

        let child = match self.cmp_to_point_value(point) {
            Ordering::Less | Ordering::Equal => &mut self.left,
            Ordering::Greater => &mut self.right,
        };

        if child.is_none() {
            *child = Some(Box::new(Node::new(point, value, self.axis.next())));
            return None;
        }

        child.as_mut().unwrap().add(point, value)
    }

    /// Return whether the given point lies before, in the same place or after
    /// this point.
    fn cmp_to_point_value(&self, point: Point<T>) -> Ordering {
        let cur_axis_value = self.median.axis_value(self.axis);
        let point_axis_value = point.axis_value(self.axis);

        point_axis_value.cmp(&cur_axis_value)
    }
}

impl Axis {
    /// Return the next axis, going back to the beginning if necessary.
    pub fn next(self) -> Self {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::X,
        }
    }
}

impl<T> AxisValue for Point<T> {
    type Value = T;

    fn axis_value(&self, axis: Axis) -> &Self::Value {
        match axis {
            Axis::X => &self.x,
            Axis::Y => &self.y,
        }
    }
}

impl<T> Range<Point<T>> for Rect<T>
where
    T: num::Num + From<u8> + Copy + PartialOrd,
{
    type AxisValue = T;

    fn contains(&self, point: &Point<T>) -> bool {
        Rect::contains(self, point)
    }

    fn axis_value_range(&self, axis: Axis) -> (Self::AxisValue, Self::AxisValue) {
        match axis {
            Axis::X => (self.origin.x, self.origin.x + self.width),
            Axis::Y => (self.origin.y, self.origin.y + self.height),
        }
    }
}

/// Iterator over all the elements of a KdTree.
pub struct KdTreeIter<'a, T: 'a, V: 'a> {
    nodes: Vec<&'a Node<T, V>>,
}

impl<'a, T, V> Iterator for KdTreeIter<'a, T, V> {
    type Item = (&'a Point<T>, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.nodes.pop().map(|node| {
            if let Some(ref n) = node.right {
                self.nodes.push(n);
            }

            if let Some(ref n) = node.left {
                self.nodes.push(n);
            }

            (&node.median, &node.value)
        })
    }
}

/// Iterator over the points contained in the given range in the kdtree.
pub struct InRangeIter<'a, 'r, T: 'a, V: 'a, R: 'r> {
    nodes: Vec<&'a Node<T, V>>,
    range: &'r R,
}

impl<'a, 'r, T, V, R> Iterator for InRangeIter<'a, 'r, T, V, R>
where
    T: Copy + Ord,
    R: Range<Point<T>, AxisValue = T>,
{
    type Item = (&'a Point<T>, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.nodes.pop() {
            let (range_low, range_high) = self.range.axis_value_range(node.axis);
            let median_axis_value = node.median.axis_value(node.axis);

            let mut push_node = |node: &'a Option<Box<Node<T, V>>>| {
                if let Some(ref n) = node {
                    self.nodes.push(n);
                }
            };

            // if there are no intersections between the range and the median
            // axis value then search only on the side that contains the range.
            // If there is an intersection then we must check both sides since
            // the range could contain both of them.
            if *median_axis_value < range_low {
                push_node(&node.right);
            } else if *median_axis_value > range_high {
                push_node(&node.left);
            } else {
                push_node(&node.right);
                push_node(&node.left);
            }

            if self.range.contains(&node.median) {
                return Some((&node.median, &node.value));
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::{Axis, KdTree, Node};

    extern crate num;
    extern crate proptest;

    use std::collections::HashSet;

    use geo::{PointU32, Rect};

    // arbitrary sorting just to ensure the order is the same in both arrays
    fn sort_points(pts: &mut [(&PointU32, &())]) {
        pts.sort_by_key(|(pt, _)| (pt.x, pt.y));
    }

    #[test]
    fn test_from_vector() {
        let points = vec![
            (PointU32::new(1, 2), "p(1,2)"),
            (PointU32::new(0, 0), "to be replaced"),
            (PointU32::new(4, 5), "root"),
            (PointU32::new(7, 8), "p(7,8)"),
            (PointU32::new(5, 2), "p(5,2)"),
        ];

        let mut kdtree = KdTree::from_vector(points);

        assert_eq!(
            kdtree.add(PointU32::new(0, 0), "p(0,0)"),
            Some("to be replaced")
        );
        assert!(kdtree.add(PointU32::new(2, 9), "p(2,9)").is_none());
        assert!(kdtree.add(PointU32::new(2, 8), "p(2,8)").is_none());

        assert_eq!(
            kdtree,
            KdTree {
                length: 7,
                root: Some(Node {
                    median: PointU32::new(4, 5),
                    axis: Axis::X,
                    value: "root",

                    left: Some(Box::new(Node {
                        median: PointU32::new(1, 2),
                        axis: Axis::Y,
                        value: "p(1,2)",
                        left: Some(Box::new(Node::new(PointU32::new(0, 0), "p(0,0)", Axis::X))),
                        right: Some(Box::new(Node {
                            median: PointU32::new(2, 9),
                            axis: Axis::X,
                            value: "p(2,9)",

                            left: Some(Box::new(Node::new(PointU32::new(2, 8), "p(2,8)", Axis::Y))),
                            right: None,
                        })),
                    })),

                    right: Some(Box::new(Node {
                        median: PointU32::new(7, 8),
                        axis: Axis::Y,
                        value: "p(7,8)",
                        left: Some(Box::new(Node::new(PointU32::new(5, 2), "p(5,2)", Axis::X))),
                        right: None,
                    })),
                })
            }
        );
    }

    proptest! {
        #[test]
        fn prop_kdtree_iter_same_points_as_input(
            points in proptest::collection::hash_set((0_u32..255, 0_u32..255), 0..100)
        ) {
            let points = points
                .into_iter()
                .map(|(x, y)| (PointU32::new(x, y), ()))
                .collect::<Vec<_>>();
            let tree = KdTree::from_vector(points.clone());

            let mut points = points.iter().map(|(pt, v)| (pt, v)).collect::<Vec<_>>();
            let mut tree_points = tree.iter().collect::<Vec<_>>();

            sort_points(&mut points);
            sort_points(&mut tree_points);

            assert_eq!(points, tree_points);
        }
    }

    #[test]
    fn test_basic_range_query() {
        let kdtree = KdTree::from_vector(vec![
            (PointU32::new(0, 0), "origin"),
            (PointU32::new(42, 73), "beautiful"),
            (PointU32::new(7, 14), "random1"),
            (PointU32::new(9, 1), "random2"),
            (PointU32::new(6, 6), "number of the beast"),
        ]);

        assert_eq!(
            kdtree
                .in_range_iter(&Rect::new(PointU32::new(0, 0), 2, 5))
                .collect::<Vec<_>>(),
            vec![(&PointU32::new(0, 0), &"origin")]
        );

        assert_eq!(
            kdtree
                .in_range_iter(&Rect::new(PointU32::new(42, 73), 0, 0))
                .collect::<Vec<_>>(),
            vec![(&PointU32::new(42, 73), &"beautiful")]
        );
    }

    proptest! {
        #![proptest_config(proptest::test_runner::Config::with_cases(500))]
        #[test]
        fn prop_kdtree_in_range_same_as_loop(
            points in proptest::collection::hash_set((0_u32..255, 0_u32..255), 0..100),
            rect in (0_u32..255, 0_u32..255, 0_u32..255, 0_u32..255)
        ) {
            same_as_range_brute_force_loop(points, rect);
        }
    }

    fn same_as_range_brute_force_loop(points: HashSet<(u32, u32)>, rect: (u32, u32, u32, u32)) {
        let points = points
            .into_iter()
            .map(|(x, y)| (PointU32::new(x, y), ()))
            .collect::<Vec<_>>();

        let tree = KdTree::from_vector(points.clone());
        let range = Rect::new(PointU32::new(rect.0, rect.1), rect.2, rect.3);

        let mut contained_points = tree.in_range_iter(&range).collect::<Vec<_>>();
        let mut brute_force_contained_points = points
            .iter()
            .filter_map(|(pt, val)| {
                if range.contains(pt) {
                    Some((pt, val))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        sort_points(&mut contained_points);
        sort_points(&mut brute_force_contained_points);

        assert_eq!(contained_points, brute_force_contained_points);
    }

    #[test]
    fn test_basic_nearest_neighbor() {
        let mut kdtree = KdTree::new();
        kdtree.add(PointU32::new(3, 0), "foo");
        kdtree.add(PointU32::new(4, 6), "bar");
        kdtree.add(PointU32::new(4, 5), "baz");
        kdtree.add(PointU32::new(100, 100), "quux");

        assert_eq!(
            kdtree.nearest_neighbor(PointU32::new(3, 0)),
            Some((&PointU32::new(3, 0), &"foo"))
        );

        assert_eq!(
            kdtree.nearest_neighbor(PointU32::new(3, 1)),
            Some((&PointU32::new(3, 0), &"foo"))
        );

        assert_eq!(
            kdtree.nearest_neighbor(PointU32::new(2, 5)),
            Some((&PointU32::new(4, 5), &"baz"))
        );

        assert_eq!(
            kdtree.nearest_neighbor(PointU32::new(0, 0)),
            Some((&PointU32::new(3, 0), &"foo"))
        );
    }

    #[test]
    fn test_nearest_neighbor_comes_after_candidate() {
        let mut kdtree = KdTree::new();
        kdtree.add(PointU32::new(0, 1), ());
        kdtree.add(PointU32::new(0, 0), ());
        kdtree.add(PointU32::new(0, 2), ());

        assert_eq!(
            kdtree.nearest_neighbor(PointU32::new(1, 2)),
            Some((&PointU32::new(0, 2), &()))
        );
    }

    proptest! {
        #![proptest_config(proptest::test_runner::Config::with_cases(500))]
        #[test]
        fn prop_kdtree_nearest_neight_same_as_loop(
            points in proptest::collection::hash_set((0_u32..255, 0_u32..255), 1..100),
            to_search in (0_u32..255, 0_u32..255)
        ) {
            same_as_nn_brute_force_loop(points, to_search);
        }
    }

    fn same_as_nn_brute_force_loop(points: HashSet<(u32, u32)>, to_search: (u32, u32)) {
        let points = points
            .into_iter()
            .map(|(x, y)| (PointU32::new(x, y), ()))
            .collect::<Vec<_>>();

        let tree = KdTree::from_vector(points.clone());
        let to_search = PointU32::new(to_search.0, to_search.1);

        let tree_closest_point = tree.nearest_neighbor(to_search);

        let brute_force_closest_point = points
            .iter()
            .min_by_key(|(pt, _)| pt.squared_dist::<i64>(&to_search));

        assert!(tree_closest_point.is_some());
        assert!(brute_force_closest_point.is_some());

        let brute_force_closest_point = brute_force_closest_point.unwrap().0;
        let tree_closest_point = tree_closest_point.unwrap().0;

        assert_eq!(
            brute_force_closest_point.squared_dist::<i64>(&to_search),
            tree_closest_point.squared_dist::<i64>(&to_search),
            "brute_force: {:?}, kd-tree: {:?} tree: {:?}",
            brute_force_closest_point,
            tree_closest_point,
            tree
        )
    }
}
