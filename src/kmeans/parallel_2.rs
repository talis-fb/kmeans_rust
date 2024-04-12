#![allow(dead_code)]

use std::collections::BTreeMap;

use rayon::prelude::*;

use crate::entities::{Cluster, Point};

use super::{common, Kmeans};

#[derive(Default)]
pub struct KmeansParallelBuilder2 {
    pub initial_centers: Option<Vec<Point>>,
}

impl Kmeans for KmeansParallelBuilder2 {
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
            clusters = data
                .par_iter()
                .map(|point| {
                    let index = common::get_closest_cluster_index(point, &clusters);
                    let centroid = &clusters[index].center;
                    (centroid, point)
                })
                .fold_with(
                    BTreeMap::<&Point, Vec<&Point>>::default(),
                    |mut acc, (centroid, point)| {
                        acc.entry(centroid).or_insert_with(Vec::new).push(point);
                        acc
                    },
                )
                .reduce(
                    || BTreeMap::<&Point, Vec<&Point>>::default(),
                    |mut map1, map2| {
                        for (key, values) in map2 {
                            map1.entry(key).or_insert_with(Vec::new).extend(values);
                        }
                        map1
                    },
                )
                .into_iter()
                .map(|(k, v)| Cluster {
                    center: k.clone(),
                    points: v,
                })
                .collect();

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
    }
}

impl KmeansParallelBuilder2 {
    pub fn with_initial_centers(
        mut self,
        initial_centers: impl IntoIterator<Item = Point>,
    ) -> Self {
        self.initial_centers = Some(initial_centers.into_iter().collect());
        self
    }
}
