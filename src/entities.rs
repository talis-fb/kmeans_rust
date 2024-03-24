#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
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
        }
    }

    pub fn euclidean_distance(&self, other: &Point) -> f64 {
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;
        let z_diff = self.z - other.z;
        x_diff.powi(2) + y_diff.powi(2) + z_diff.powi(2)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Cluster {
    pub center: Point,
    pub points: Vec<Point>,
}

impl Cluster {
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
        }
    }
}
