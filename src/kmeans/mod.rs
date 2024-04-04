use crate::entities::{Cluster, Point};

pub mod parallel;
pub mod serial;

pub mod common;

pub trait Kmeans {
    fn execute<'a>(&'a self, data: &'a Vec<Point>, k: u8) -> Vec<Cluster<'a>>;
}
