#![allow(dead_code)]

use crate::entities::{Cluster, Point};

#[derive(Default)]
pub struct KmeansSerialBuilder {
    pub data: Vec<Point>,
    pub k: u8,
    pub initial_centers: Option<Vec<Point>>,
}

impl KmeansSerialBuilder {
    pub fn with_data(mut self, data: impl IntoIterator<Item = Point>) -> Self {
        self.data = data.into_iter().collect();
        self
    }

    pub fn with_k(mut self, k: u8) -> Self {
        self.k = k;
        self
    }

    pub fn with_initial_centers(
        mut self,
        initial_centers: impl IntoIterator<Item = Point>,
    ) -> Self {
        self.initial_centers = Some(initial_centers.into_iter().collect());
        self
    }

    pub fn execute(self) -> Vec<Cluster> {
        let initial_centers = self
            .initial_centers
            .unwrap_or_else(|| utils::get_n_random_points(&self.data, self.k as usize));

        let mut clusters = initial_centers
            .into_iter()
            .map(|center| Cluster::from_center(center))
            .collect::<Vec<Cluster>>();

        loop {
            clusters = utils::assign_points(&self.data, clusters);

            let new_centers: Vec<Point> = utils::calculate_new_centers(&clusters);
            let old_centers: Vec<Point> = clusters
                .iter()
                .map(|cluster| cluster.center.clone())
                .collect();

            if utils::converged(&new_centers, &old_centers) {
                return clusters;
            }

            clusters = new_centers
                .into_iter()
                .map(|center| Cluster::from_center(center))
                .collect();
        }
    }
}

mod utils {
    use super::*;
    use rand::seq::SliceRandom;

    pub fn get_n_random_points(points: &[Point], n: usize) -> Vec<Point> {
        let mut points = points.to_vec();
        points.shuffle(&mut rand::thread_rng());
        points.iter().take(n).cloned().collect()
    }

    pub fn assign_points(data: &Vec<Point>, mut clusters: Vec<Cluster>) -> Vec<Cluster> {
        for point in data {
            let mut min_distance = f64::MAX;
            let mut index = 0;
            for (i, cluster) in clusters.iter().enumerate() {
                let distance = point.euclidean_distance(&cluster.center);
                if distance < min_distance {
                    min_distance = distance;
                    index = i;
                }
            }
            clusters[index].points.push(point.clone());
        }

        clusters
    }

    pub fn converged(points1: &Vec<Point>, points2: &Vec<Point>) -> bool {
        points1.iter().zip(points2.iter()).all(|(p1, p2)| p1 == p2)
    }

    pub fn calculate_new_centers(cluster: &Vec<Cluster>) -> Vec<Point> {
        cluster
            .into_iter()
            .map(|cluster| cluster.calculate_center_point())
            .collect()
    }

}
