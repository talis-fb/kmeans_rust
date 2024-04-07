use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
    label: Option<Arc<str>>,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x
            .to_bits()
            .cmp(&other.x.to_bits())
            .then_with(|| self.y.to_bits().cmp(&other.y.to_bits()))
            .then_with(|| self.z.to_bits().cmp(&other.z.to_bits()))
    }
}

impl Eq for Point {}

impl std::hash::Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
        self.z.to_bits().hash(state);
    }
}

impl Point {
    pub fn from<T, const N: usize>(arr: [T; N]) -> Self
    where
        T: Into<f64>,
    {
        let arr: [f64; N] = arr.map(|n| n.into());
        Self {
            x: *arr.get(0).unwrap_or(&f64::default()),
            y: *arr.get(1).unwrap_or(&f64::default()),
            z: *arr.get(2).unwrap_or(&f64::default()),
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

    pub fn get_data(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    pub fn euclidean_distance(&self, other: &Point) -> f64 {
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;
        let z_diff = self.z - other.z;
        x_diff.powi(2) + y_diff.powi(2) + z_diff.powi(2)
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
        let x_sum: f64 = self.points.iter().map(|point| point.x).sum();
        let y_sum: f64 = self.points.iter().map(|point| point.y).sum();
        let z_sum: f64 = self.points.iter().map(|point| point.z).sum();
        Point {
            x: x_sum / self.points.len() as f64,
            y: y_sum / self.points.len() as f64,
            z: z_sum / self.points.len() as f64,
            label: None,
        }
    }
}
