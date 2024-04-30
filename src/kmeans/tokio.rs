#![allow(dead_code)]

use std::sync::Arc;

use tokio::sync::mpsc;

use crate::entities::{Cluster, Point};

use super::{common, Kmeans};

#[derive(Default)]
pub struct KmeansTokioBuilder {
    pub max_threads: usize,
}

impl Kmeans for KmeansTokioBuilder {
    fn execute<'a>(
        &self,
        data: &'static Vec<Point>,
        k: u8,
        initial_centers: Vec<Point>,
    ) -> Vec<Cluster<'a>> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            let mut clusters = initial_centers
                .into_iter()
                .map(|center| Cluster::from_center(center))
                .collect::<Vec<Cluster>>();

            eprintln!("initial centers: {:?}", clusters);

            // A map based in index to sender points to add in clusters (tasks)
            loop {
                let (tx_final_clusters, mut rx_final_clusters) =
                    tokio::sync::mpsc::channel::<Cluster>(k.into());

                let clusters_senders: Arc<Vec<mpsc::Sender<&Point>>> = clusters
                    .iter()
                    .map(|cluster| {
                        let (sender_points, mut listen_points) = mpsc::channel::<&Point>(500);
                        tokio::task::spawn({
                            let center = cluster.center.clone();
                            let send_finish = tx_final_clusters.clone();
                            async move {
                                let mut points = Vec::with_capacity(data.len());
                                while let Some(point) = listen_points.recv().await {
                                    points.push(point);
                                }
                                send_finish.send(Cluster { center, points }).await.unwrap();
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
                    tokio::task::spawn(async move {
                        while index < data.len() {
                            let point = data.get(index).unwrap();

                            let ind_closest_cluster =
                                common::get_closest_cluster_index(point, clusters.as_ref());
                            clusters_senders
                                .get(ind_closest_cluster)
                                .unwrap()
                                .send(point)
                                .await
                                .unwrap();
                            index += max_threads;
                        }
                    });
                }

                drop(clusters_senders);

                clusters = Vec::with_capacity(k as usize);
                while let Some(cluster) = rx_final_clusters.recv().await {
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
        })
    }
}
