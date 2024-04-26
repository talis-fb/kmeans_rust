use rand::seq::SliceRandom;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::entities::{Cluster, Point};

pub fn get_n_random_points(points: &[Point], n: usize) -> Vec<Point> {
    let mut points = points.to_vec();
    points.shuffle(&mut rand::thread_rng());
    points.iter().take(n).cloned().collect()
}

pub fn get_closest_cluster_index<'a>(
    point: &Point,
    clusters: impl IntoIterator<Item = &'a Cluster<'a>>,
) -> usize {
    let mut min_distance = u32::MAX;
    let mut index = 0;
    for (i, cluster) in clusters.into_iter().enumerate() {
        let distance = point.euclidean_distance(&cluster.center);
        if distance < min_distance {
            min_distance = distance;
            index = i;
        }
    }
    index
}

pub fn assign_points<'a>(data: &'a Vec<Point>, mut clusters: Vec<Cluster<'a>>) -> Vec<Cluster<'a>> {
    for point in data {
        let index = get_closest_cluster_index(point, &clusters);
        clusters[index].points.push(point);
    }

    clusters
}

pub fn converged<'a>(
    points1: impl IntoIterator<Item = &'a Point>,
    points2: impl IntoIterator<Item = &'a Point>,
) -> bool {
    points1
        .into_iter()
        .zip(points2.into_iter())
        .all(|(p1, p2)| p1 == p2)
}

pub fn calculate_new_centers<'a>(
    clusters: impl IntoIterator<Item = &'a Cluster<'a>>,
) -> Vec<Point> {
    clusters
        .into_iter()
        .map(|cluster| cluster.calculate_center_point())
        .collect()
}

pub fn calculate_new_centers_parallel(cluster: &Vec<Cluster>) -> Vec<Point> {
    cluster
        .par_iter()
        .map(|cluster| cluster.calculate_center_point())
        .collect()
}
