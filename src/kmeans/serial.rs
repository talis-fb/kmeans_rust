#![allow(dead_code)]

use crate::entities::{Cluster, Point};

use super::common;

use super::Kmeans;

#[derive(Default)]
pub struct KmeansSerialBuilder {
    pub initial_centers: Option<Vec<Point>>,
}

impl Kmeans for KmeansSerialBuilder {
    fn execute<'a>(&'a self, data: &'a Vec<Point>, k: u8) -> Vec<Cluster<'a>> {
        let initial_centers = self
            .initial_centers
            .clone()
            .unwrap_or_else(|| common::get_n_random_points(data, k as usize));

        let mut clusters = initial_centers
            .into_iter()
            .map(|center| Cluster::from_center(center))
            .collect::<Vec<Cluster>>();

        loop {
            clusters = utils::assign_points(data, clusters);

            let new_centers: Vec<Point> = common::calculate_new_centers(&clusters);
            let old_centers: Vec<_> = clusters.iter().map(|cluster| &cluster.center).collect();

            if common::converged(new_centers.iter(), old_centers) {
                return clusters;
            }

            clusters = new_centers
                .into_iter()
                .map(|center| Cluster::from_center(center))
                .collect();
        }
    }
}

impl KmeansSerialBuilder {
    pub fn with_initial_centers(
        mut self,
        initial_centers: impl IntoIterator<Item = Point>,
    ) -> Self {
        self.initial_centers = Some(initial_centers.into_iter().collect());
        self
    }
}

mod utils {
    use super::*;

    pub fn assign_points<'a>(
        data: &'a Vec<Point>,
        mut clusters: Vec<Cluster<'a>>,
    ) -> Vec<Cluster<'a>> {
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
            clusters[index].points.push(point);
        }

        clusters
    }
}
