#![allow(dead_code)]

use std::collections::BTreeMap;

use rayon::prelude::*;

use crate::entities::{Cluster, Point};

use super::{common, Kmeans};

#[derive(Default)]
pub struct KmeansParallelBuilder2;

impl Kmeans for KmeansParallelBuilder2 {
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
