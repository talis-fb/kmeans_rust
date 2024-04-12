#![allow(dead_code)]

use std::sync::Arc;

use tokio::sync::mpsc;

use crate::entities::{Cluster, Point};

use super::{common, Kmeans};

#[derive(Default)]
pub struct KmeansTokioBuilder {
    pub initial_centers: Option<Vec<Point>>,
}

impl Kmeans for KmeansTokioBuilder {
    fn execute<'a>(&'a self, data: &'static Vec<Point>, k: u8) -> Vec<Cluster<'a>> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            let initial_centers: Vec<Point> = self
                .initial_centers
                .clone()
                .unwrap_or_else(|| common::get_n_random_points(&data, k as usize));

            let mut clusters = initial_centers
                .into_iter()
                .map(|center| Cluster::from_center(center))
                .collect::<Vec<Cluster>>();

            eprintln!("initial centers: {:?}", clusters);

            // A map based in index to sender points to add in clusters (tasks)
            loop {
                let mut clusters_inputs: Vec<mpsc::Sender<&Point>> = Vec::with_capacity(k.into());

                let (tx_final_clusters, mut rx_final_clusters) =
                    tokio::sync::mpsc::channel::<Cluster>(k.into());

                for (_i, cluster) in clusters.iter().enumerate() {
                    let center = cluster.center.clone();

                    let (tx_points, mut rx_points) = mpsc::channel::<&Point>(50);

                    clusters_inputs.push(tx_points);

                    let res = tx_final_clusters.clone();

                    tokio::task::spawn(async move {
                        let mut points = Vec::new();
                        while let Some(point) = rx_points.recv().await {
                            points.push(point);
                        }
                        res.send(Cluster { center, points }).await.unwrap();
                    });
                }

                drop(tx_final_clusters);

                let clusters_arc = Arc::new(clusters.clone());

                tokio::task::spawn(async move {
                    let clusters_inputs_arc = Arc::new(clusters_inputs);

                    for point in data.iter() {
                        let input = clusters_inputs_arc.clone();
                        let cl = clusters_arc.clone();
                        tokio::task::spawn(async move {
                            let i = common::get_closest_cluster_index(point, cl.as_ref());
                            input.get(i).unwrap().send(point).await.unwrap();
                        });
                    }
                });

                clusters = Vec::with_capacity(k as usize);
                while let Some(cluster) = rx_final_clusters.recv().await {
                    eprintln!("cluster {:?}", cluster.center);
                    clusters.push(cluster);
                }

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

            // TODO:
            // The tests are right. The problemn is only the order
        })
    }
}

impl KmeansTokioBuilder {
    pub fn with_initial_centers(
        mut self,
        initial_centers: impl IntoIterator<Item = Point>,
    ) -> Self {
        self.initial_centers = Some(initial_centers.into_iter().collect());
        self
    }
}
