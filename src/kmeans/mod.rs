use crate::entities::{Cluster, Point};

pub mod parallel;
pub mod parallel_2;
pub mod parallel_3;
pub mod parallel_mutex;
pub mod serial;
pub mod tokio;

pub mod common;

pub trait Kmeans {
    fn execute<'a>(
        &self,
        data: &'static Vec<Point>,
        k: u8,
        initial_centers: Vec<Point>,
    ) -> Vec<Cluster<'a>>;
}
