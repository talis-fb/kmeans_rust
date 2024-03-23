fn main() {
    let data: Vec<Point> = Vec::from([
        (1.0, 2.0),
        (5.0, 8.0),
        (1.0, 3.0),
        (6.0, 9.0),
        (2.0, 3.0),
        (5.0, 1.00),
        (1.0, 1.0),
        (7.0, 8.0),
    ])
    .into_iter()
    .map(Point::from)
    .collect();

    const K: u8 = 2;

    let mut clusters = Cluster::init(data.clone(), K);

    loop {
        clusters = assignPoints(data.clone(), clusters);

        let new_centers: Vec<Point> = calculate_new_centers(&clusters);
        let old_centers: Vec<Point> = clusters.iter().map(|cluster| cluster.center.clone()).collect();

        if converged(&new_centers, &old_centers) {
            break;
        }

        for (i, cluster) in clusters.iter_mut().enumerate() {
            cluster.center = new_centers.get(i).unwrap().clone();
        }
    }

    for cluster in clusters {
        println!("{:?}", cluster);
    }
}

fn assignPoints(data: Vec<Point>, clusters: Vec<Cluster>) -> Vec<Cluster> {
    let mut clusters: Vec<Cluster> = clusters
        .into_iter()
        .map(|cluster| Cluster {
            center: cluster.center,
            points: vec![],
        })
        .collect();

    for point in data {
        let mut min_distance = f64::MAX;
        let mut index = 0;
        for (i, cluster) in clusters.iter().enumerate() {
            let distance = euclideanDistance(&point, &cluster.center);
            if distance < min_distance {
                min_distance = distance;
                index = i;
            }
        }
        clusters[index].points.push(point);
    }

    clusters
}

fn converged(points1: &Vec<Point>, points2: &Vec<Point>) -> bool {
    points1
        .iter()
        .zip(points2.iter())
        .all(|(p1, p2)| p1.is_equal(&p2))
}

fn calculate_new_centers(cluster: &Vec<Cluster>) -> Vec<Point> {
    cluster
        .into_iter()
        .map(|cluster| calculate_center(cluster))
        .collect()
}

fn calculate_center(cluster: &Cluster) -> Point {
    let x_sum: f64 = cluster.points.iter().map(|point| point.x).sum();
    let y_sum: f64 = cluster.points.iter().map(|point| point.x).sum();
    Point {
        x: x_sum / cluster.points.len() as f64,
        y: y_sum / cluster.points.len() as f64,
    }
}

fn euclideanDistance(a: &Point, b: &Point) -> f64 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

#[derive(Debug, Clone)]
struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn is_equal(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl From<(f64, f64)> for Point {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Cluster {
    pub center: Point,
    pub points: Vec<Point>,
}
impl Cluster {
    pub fn init(points: Vec<Point>, k: u8) -> Vec<Self> {
        (0..k)
            .into_iter()
            .map(|i| points.get(i as usize).unwrap())
            .cloned()
            .map(|center| Self {
                center,
                points: Vec::new(),
            })
            .collect()
    }
}
