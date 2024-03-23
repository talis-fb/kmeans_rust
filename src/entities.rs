#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Eq for Point {}

impl std::hash::Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
    }
}

impl Point {
    pub fn from<T: Into<f64>>((x, y): (T, T)) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }

    pub fn euclidean_distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
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
        Point {
            x: x_sum / self.points.len() as f64,
            y: y_sum / self.points.len() as f64,
        }
    }

}
