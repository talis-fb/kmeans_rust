#![allow(dead_code)]

use std::sync::mpsc;

use rayon::prelude::*;

use crate::entities::{Cluster, Point};

use super::{common, Kmeans};

#[derive(Default)]
pub struct KmeansParallelBuilder {
    pub initial_centers: Option<Vec<Point>>,
}

impl Kmeans for KmeansParallelBuilder {
    fn execute<'a>(&'a self, data: &'static Vec<Point>, k: u8) -> Vec<Cluster<'a>> {
        let initial_centers: Vec<Point> = self
            .initial_centers
            .clone()
            .unwrap_or_else(|| common::get_n_random_points(data, k as usize));

        let mut clusters = initial_centers
            .into_iter()
            .map(|center| Cluster::from_center(center))
            .collect::<Vec<Cluster>>();

        let mut i: u64 = 1;
        loop {
            eprintln!("iteration {}", i);
            let clusters_to_read = clusters.clone();
            let clusters_to_write = &mut clusters;

            let (tx, rx) = mpsc::channel::<(&Point, usize)>();

            // eprintln!("b");
            rayon::scope(move |scope| {
                scope.spawn(move |_| {
                    while let Ok((point, index)) = rx.recv() {
                        eprintln!("> {:?}", point.get_label());
                        clusters_to_write[index].points.push(point);
                    }
                });

                // eprintln!("middle");
                scope.spawn(move |_| {
                    data.par_iter().for_each(|point| {
                        eprintln!("b {:?}", point.get_label());
                        let index = common::get_closest_cluster_index(&point, &clusters_to_read);
                        tx.send((point, index)).unwrap();
                        eprintln!("e {:?}", point.get_label());
                    });
                });
                // eprintln!("end pre");
            });
            // eprintln!("end pos");

            i = i.saturating_add(1);

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
