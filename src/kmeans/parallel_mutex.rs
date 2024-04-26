#![allow(dead_code)]

use crate::entities::{Cluster, Point};

use super::{common, Kmeans};

use std::sync::Barrier;
use std::sync::Condvar;
use std::sync::Mutex;
use std::sync::mpsc;
use std::sync::Arc;
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

        let mut clusters_arc: Arc<Vec<_>> = Arc::new(clusters);

        eprintln!("initial centers: {:?}", clusters_arc);

        let max_threads = self.max_threads.min(data.len());

        let (tx, rx) = mpsc::channel::<()>();
        let mut tx_init_vec: Vec<mpsc::Sender<()>> = Vec::with_capacity(max_threads);

        // let (tx_init, rx_init) = mpsc::channel::<()>();
        // let (tx_init, rx_init) = mpsc::channel::<()>();

        let has_finished = Arc::new(Mutex::new(false));

        for index_of_thread in 0..max_threads {
            let tx = tx.clone();
            let has_finished = has_finished.clone();
            let clusters = clusters_arc.clone();

            let (tx_init, rx_init) = mpsc::channel::<()>();
            tx_init_vec.push(tx_init);

            std::thread::spawn(move || {
                let initial_index = index_of_thread;
                eprintln!("task with ind: {index_of_thread}");
                loop {
                    rx_init.recv().unwrap();

                    if *has_finished.lock().unwrap() {
                        break;
                    }

                    let mut index = initial_index;
                    while index < data.len() {
                        let point = data.get(index).unwrap();

                        let ind_closest_cluster = {
                            let bind = clusters
                                .iter()
                                .map(|lock| lock.read().unwrap().center.clone())
                                .collect::<Vec<_>>();
                            get_closest_cluster_index_based_in_centroids(point, bind.iter())
                        };

                        let mut target_cluster =
                            clusters.get(ind_closest_cluster).unwrap().write().unwrap();

                        (*target_cluster).points.push(point);

                        index += max_threads;
                    }

                    tx.send(()).unwrap();
                }
            });
        }

        tx_init_vec.iter().for_each(|tx_init| tx_init.send(()).unwrap());

        // A map based in index to sender points to add in clusters (tasks)
        loop {
            eprintln!("INICIOU O LOOP");

            let mut threads_finished = 0;
            while threads_finished < max_threads {
                eprintln!("a");
                rx.recv().unwrap();
                threads_finished += 1;
                eprintln!("b");
            }

            let new_centers = {
                let new_centers: Vec<Point> = {
                    clusters_arc
                        .iter()
                        .map(|lock| {
                            let ele = lock.read().unwrap();
                            eprintln!("new_centers -> {:?}", ele.center);
                            eprintln!("new_centers -> {:?}", ele.points.len());
                            ele.calculate_center_point()
                        })
                        .collect()
                };

                eprintln!("new_centers {:?}", new_centers);

                let old_centers: Vec<Point> = clusters_arc
                    .iter()
                    .map(|cluster| cluster.read().unwrap().center.clone())
                    .collect();

                eprintln!("old_centers {:?}", old_centers);

                if common::converged(new_centers.iter(), old_centers.iter()) {
                    eprintln!("FECHOUUUUUUUUUUUUUUU aq msm");
                    eprintln!("{:?}", clusters_arc.len());
                    eprintln!("{:?}", clusters_arc);

                    {
                        let mut has_finished = has_finished.lock().unwrap();
                        *has_finished = true;
                    }

                    tx_init_vec.iter().for_each(|tx_init| tx_init.send(()).unwrap());

                    return clusters_arc
                        .iter()
                        .map(|cluster| cluster.read().expect("aq mesmoo").clone())
                        .collect();
                } 

                new_centers
            };

            // clusters_arc.iter().for_each(|lock| {
            //     let mut lock = lock.write().unwrap();
            //     *lock = RwLock::new(Cluster::from_center(new_centers.get(0).unwrap().clone()));
            // });

            for (i, center) in new_centers.into_iter().enumerate() {
                let mut target_cluster = clusters_arc.get(i).unwrap().write().unwrap();
                target_cluster.center = center;
                target_cluster.points.clear();
            }

            // clusters_arc = new_centers
            //     .into_iter()
            //     .map(|center| Cluster::from_center(center))
            //     .map(|bixim| {
            //         eprintln!("{:?}", bixim);
            //         bixim
            //     })
            //     .map(|cluster| RwLock::new(cluster))
            //     .collect::<Vec<_>>()
            //     .into();

            eprintln!("FIM DO LOOP");
            tx_init_vec.iter().for_each(|tx_init| tx_init.send(()).unwrap());
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
