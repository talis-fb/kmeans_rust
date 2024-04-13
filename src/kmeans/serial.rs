#![allow(dead_code)]

use crate::entities::{Cluster, Point};

use super::common;

use super::Kmeans;

#[derive(Default)]
pub struct KmeansSerialBuilder;

impl Kmeans for KmeansSerialBuilder {
    fn execute<'a>(
        &self,
        data: &'static Vec<Point>,
        _k: u8,
        initial_centers: Vec<Point>,
    ) -> Vec<Cluster<'a>> {
        let mut clusters = initial_centers
            .into_iter()
            .map(|center| Cluster::from_center(center))
            .collect::<Vec<Cluster>>();

        let mut i: u64 = 1;
        loop {
            clusters = common::assign_points(data, clusters);

            i = i.saturating_add(1);

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
