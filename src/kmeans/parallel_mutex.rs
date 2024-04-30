#![allow(dead_code)]

use crate::entities::{Cluster, Point};

use super::{common, Kmeans};

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

#[derive(Default)]
pub struct KmeansParallelMutex {
    pub max_threads: usize,
}

impl Kmeans for KmeansParallelMutex {
    fn execute<'a>(
        &self,
        data: &'static Vec<Point>,
        k: u8,
        initial_centers: Vec<Point>,
    ) -> Vec<Cluster<'a>> {
        let clusters: Vec<RwLock<Cluster>> = initial_centers
            .into_iter()
            .map(|center| Cluster::from_center(center))
            .map(|cluster| RwLock::new(cluster))
            .collect();

        let clusters_arc: Arc<Vec<_>> = Arc::new(clusters);

        let max_threads = self.max_threads.min(data.len());


        let has_finished = Arc::new(Mutex::new(false));

        let (tx, rx) = mpsc::channel::<()>();
        let mut tx_init_vec: Vec<mpsc::Sender<()>> = Vec::with_capacity(max_threads);


        for index_of_thread in 0..max_threads {
            let tx = tx.clone();
            let has_finished = has_finished.clone();
            let clusters = clusters_arc.clone();

            let (tx_init, rx_init) = mpsc::channel::<()>();
            tx_init_vec.push(tx_init);

            std::thread::spawn(move || {
                let initial_index = index_of_thread;
                loop {
                    // Aguarda o messagem da main para inicio
                    rx_init.recv().unwrap();

                    if *has_finished.lock().unwrap() {
                        break;
                    }

                    let mut index = initial_index;
                    while index < data.len() {
                        let point = data.get(index).unwrap();

                        let ind_closest_cluster = {
                            let clusters_centers = clusters
                                .iter()
                                .map(|lock| lock.read().unwrap().center.clone())
                                .collect::<Vec<_>>();
                            get_closest_cluster_index_based_in_centroids(point, clusters_centers.iter())
                        };

                        {
                            clusters.get(ind_closest_cluster).unwrap().write().unwrap().points.push(point);
                        }

                        index += max_threads;
                    }

                    // Mensagem de encerramento do processamento
                    tx.send(()).unwrap();
                }
            });
        }

        tx_init_vec
            .iter()
            .for_each(|tx_init| tx_init.send(()).unwrap());

        loop {


            let mut threads_finished = 0;
            while threads_finished < max_threads {
                rx.recv().unwrap();
                threads_finished += 1;
            }




            let new_centers = {
                let new_centers: Vec<Point> = {
                    clusters_arc
                        .iter()
                        .map(|lock| lock.read().unwrap().calculate_center_point())
                        .collect()
                };

                let old_centers: Vec<Point> = clusters_arc
                    .iter()
                    .map(|cluster| cluster.read().unwrap().center.clone())
                    .collect();

                if common::converged(new_centers.iter(), old_centers.iter()) {
                    {
                        let mut has_finished = has_finished.lock().unwrap();
                        *has_finished = true;
                    }

                    tx_init_vec
                        .iter()
                        .for_each(|tx_init| tx_init.send(()).unwrap());

                    return clusters_arc
                        .iter()
                        .map(|cluster| cluster.read().unwrap().clone())
                        .collect();
                }

                new_centers
            };

            for (i, center) in new_centers.into_iter().enumerate() {
                let mut target_cluster = clusters_arc.get(i).unwrap().write().unwrap();
                target_cluster.center = center;
                target_cluster.points.clear();
            }

            tx_init_vec
                .iter()
                .for_each(|tx_init| tx_init.send(()).unwrap());
        }
    }
}

pub fn get_closest_cluster_index_based_in_centroids<'a>(
    point: &Point,
    centroids: impl IntoIterator<Item = &'a Point>,
) -> usize {
    let mut min_distance = u32::MAX;
    let mut index = 0;
    for (i, cluster) in centroids.into_iter().enumerate() {
        let distance = point.euclidean_distance(&cluster);
        if distance < min_distance {
            min_distance = distance;
            index = i;
        }
    }
    index
}
