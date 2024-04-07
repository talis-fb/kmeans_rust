use crate::entities::{Cluster, Point};

pub mod parallel;
pub mod parallel_2;
pub mod serial;
pub mod tokio;

pub mod common;

pub trait Kmeans {
    fn execute<'a>(&'a self, data: &'static Vec<Point>, k: u8) -> Vec<Cluster<'a>>;
}
