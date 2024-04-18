use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    x: u32,
    y: u32,
    z: u32,
    label: Option<Arc<str>>,
}

impl Point {
    pub fn from<T, const N: usize>(arr: [T; N]) -> Self
    where
        T: Into<u32>,
    {
        let arr: [u32; N] = arr.map(|n| n.into());
        Self {
            x: *arr.get(0).unwrap_or(&0),
            y: *arr.get(1).unwrap_or(&0),
            z: *arr.get(2).unwrap_or(&0),
            label: None,
        }
    }

    pub fn with_label(mut self, label: &str) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn get_label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub fn get_data(&self) -> [u32; 3] {
        [self.x, self.y, self.z]
    }

    pub fn euclidean_distance(&self, other: &Point) -> u32 {
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;
        let z_diff = self.z - other.z;
        x_diff.pow(2) + y_diff.pow(2) + z_diff.pow(2)
    }

    pub fn get_values(&self) -> [u32; 3] {
        [self.x, self.y, self.z]
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Cluster<'a> {
    pub center: Point,
    pub points: Vec<&'a Point>,
}

impl Cluster<'_> {
    pub fn from_center(center: Point) -> Self {
        Self {
            center,
            points: Vec::new(),
        }
    }

    pub fn calculate_center_point(&self) -> Point {
        let x_sum: u32 = self.points.iter().map(|point| point.x).sum();
        let y_sum: u32 = self.points.iter().map(|point| point.y).sum();
        let z_sum: u32 = self.points.iter().map(|point| point.z).sum();
        let len = self.points.len() as u32;
        Point {
            label: None,
            x: x_sum.checked_div(len).unwrap_or(0),
            y: y_sum.checked_div(len).unwrap_or(0),
            z: z_sum.checked_div(len).unwrap_or(0),
        }
    }
}
