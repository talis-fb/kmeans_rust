#![allow(dead_code)]

use crate::entities::{Cluster, Point};

use super::{common, Kmeans};

use std::sync::mpsc;
use std::sync::Arc;

#[derive(Default)]
pub struct KmeansParallelStdBuilder {
    pub max_threads: usize,
}

impl Kmeans for KmeansParallelStdBuilder {
    fn execute<'a>(
        &self,
        data: &'static Vec<Point>,
        k: u8,
        initial_centers: Vec<Point>,
    ) -> Vec<Cluster<'a>> {
        let mut clusters = initial_centers
            .into_iter()
            .map(|center| Cluster::from_center(center))
            .collect::<Vec<Cluster>>();


        // A map based in index to sender points to add in clusters (tasks)
        loop {
            let (tx_final_clusters, rx_final_clusters) = mpsc::channel::<Cluster>();

            let clusters_senders: Arc<Vec<mpsc::Sender<&Point>>> = clusters
                .iter()
                .map(|cluster| {
                    let (sender_points, listen_points) = mpsc::channel::<&Point>();
                    std::thread::spawn({
                        let center = cluster.center.clone();
                        let send_finish = tx_final_clusters.clone();
                        move || {
                            let mut points = Vec::with_capacity(data.len());
                            while let Ok(point) = listen_points.recv() {
                                points.push(point);
                            }
                            send_finish.send(Cluster { center, points }).unwrap();
                        }
                    });
                    sender_points
                })
                .collect::<Vec<mpsc::Sender<&Point>>>()
                .into();

            drop(tx_final_clusters);

            let clusters_arc = Arc::new(clusters);

            let max_threads = self.max_threads.min(data.len());
            for mut index in 0..max_threads {
                let clusters_senders = clusters_senders.clone();
                let clusters = clusters_arc.clone();
                std::thread::spawn(move || {
                    while index < data.len() {
                        let point = data.get(index).unwrap();

                        let ind_closest_cluster =
                            common::get_closest_cluster_index(point, clusters.as_ref());
                        clusters_senders
                            .get(ind_closest_cluster)
                            .unwrap()
                            .send(point)
                            .unwrap();
                        index += max_threads;
                    }
                });
            }

            drop(clusters_senders);

            clusters = Vec::with_capacity(k as usize);
            while let Ok(cluster) = rx_final_clusters.recv() {
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
    }
}
