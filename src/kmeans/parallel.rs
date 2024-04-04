#![allow(dead_code)]

use std::sync::mpsc;

use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use crate::entities::{Cluster, Point};

use super::{Kmeans, common};

#[derive(Default)]
pub struct KmeansParallelBuilder {
    pub initial_centers: Option<Vec<Point>>,
}

impl Kmeans for KmeansParallelBuilder {
    fn execute<'a>(&'a self, data: &'a Vec<Point>, k: u8) -> Vec<Cluster<'a>> {
        let initial_centers: Vec<Point> = self
            .initial_centers
            .clone()
            .unwrap_or_else(|| common::get_n_random_points(data, k as usize));

        let mut clusters = initial_centers
            .into_iter()
            .map(|center| Cluster::from_center(center))
            .collect::<Vec<Cluster>>();

        loop {
            std::thread::scope(|scope| {
                let clusters_to_read = clusters.clone();
                let clusters_to_write = &mut clusters;

                let (tx, rx) = mpsc::channel::<(&Point, usize)>();

                scope.spawn(move || {
                    while let Ok((point, index)) = rx.recv() {
                        clusters_to_write[index].points.push(point);
                    }
                });

                data.par_iter().for_each(|point| {
                    let index = common::get_closest_cluster_index(&point, &clusters_to_read);
                    tx.send((point, index)).unwrap();
                });
            });

            let new_centers: Vec<Point> = common::calculate_new_centers_parallel(&clusters);
            let old_centers: Vec<_> = clusters.iter().map(|cluster| &cluster.center).collect();

            if common::converged(new_centers.iter(), old_centers) {
                return clusters;
            }

            clusters = new_centers
                .into_iter()
                .map(|center| Cluster::from_center(center))
                .collect();
        }

        // TODO:
        // The tests are right. The problemn is only the order
    }
}

impl KmeansParallelBuilder {
    pub fn with_initial_centers(
        mut self,
        initial_centers: impl IntoIterator<Item = Point>,
    ) -> Self {
        self.initial_centers = Some(initial_centers.into_iter().collect());
        self
    }
}
