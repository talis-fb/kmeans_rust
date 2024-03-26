use serde::Deserialize;
use std::{error::Error, fs::File};

use crate::{entities::Point, kmeans::KmeansSerialBuilder};

mod entities;
mod input;
mod kmeans;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = std::env::args_os().nth(1).expect("no input file given");
    let file = File::open(input_file)?;

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .from_reader(file);

    let mut ele = Vec::new();

    for result in rdr.deserialize() {
        let record: (String, f64, f64, f64) = result?;
        ele.push(record);
        // println!("{:?}", record);
    }

    println!("1 Ok");

    let values = ele
        .into_iter()
        .map(|(label, x, y, z)| Point::from([x, y, z]).with_label(&label))
        .collect::<Vec<Point>>();

    println!("2 ok  => {}", values.len());

    let size_in_bytes = values.len() * std::mem::size_of::<Vec<Point>>();
    let size_in_mb = size_in_bytes as f64 / (1024.0 * 1024.0);
    
    println!("Size of the Vec<Point>: {} bytes", size_in_bytes);
    println!("Size of the Vec<Point>: {:.2} MB", size_in_mb);

    let kmeans = KmeansSerialBuilder::default().with_data(values).with_k(10);

    let clusters = kmeans.execute();

    println!("3 ok {}", clusters.len());
    println!("---------------");
    println!("Clusters...");
    println!("  {:?}", clusters.len());
    println!("---------------");

    // Overwrite all data with point of their clusters
    let points_of_clusters: Vec<Point> = clusters
        .iter()
        .flat_map(|el| {
            el.points
                .iter()
                .map(|p| el.center.clone().with_label(p.get_label().unwrap_or("--")))
        })
        .collect();

    // write clusters
    let mut out = csv::WriterBuilder::new()
        .delimiter(b' ')
        .from_writer(std::io::stdout());

    for point in points_of_clusters {
        // let center =
        let label = point.get_label().unwrap_or("--");
        // let [x, y, z] = point.get_data();
        let [x, y, z] = point.get_data().map(|n| n.max(0.0).min(255.0) as u8);

        let record = vec![
            label.to_string(),
            x.to_string(),
            y.to_string(),
            z.to_string(),
        ];
        out.write_record(record).unwrap();
    }

    // write values

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::kmeans::KmeansSerialBuilder;

    use super::*;
    use entities::{Cluster, Point};

    #[test]
    fn test_kmeans_two_points() {
        let data = [Point::from([1, 2]), Point::from([5, 8])];
        let k = 2;

        let clusters_output = KmeansSerialBuilder::default()
            .with_data(data.clone())
            .with_k(k)
            .with_initial_centers(data.into_iter().take(k as usize))
            .execute();

        let clusters_output_set: HashSet<Cluster> = HashSet::from_iter(clusters_output.clone());

        assert_eq!(clusters_output.len(), k as usize);

        let expected_set = HashSet::from_iter([
            Cluster {
                center: Point::from([1, 2]),
                points: vec![Point::from([1, 2])],
            },
            Cluster {
                center: Point::from([5, 8]),
                points: vec![Point::from([5, 8])],
            },
        ]);

        assert_eq!(expected_set, clusters_output_set);
    }

    #[test]
    fn test_kmeans_few_points() {
        let data = [[1, 2], [2, 3], [8, 10], [9, 11], [10, 12]].map(Point::from);
        let k = 2;

        let clusters_output = KmeansSerialBuilder::default()
            .with_data(data.clone())
            .with_k(k)
            .with_initial_centers(data.into_iter().take(k as usize))
            .execute();
        let clusters_output_set: HashSet<Cluster> = HashSet::from_iter(clusters_output.clone());

        let expected_set = HashSet::from_iter([
            Cluster {
                center: Point::from([1.5, 2.5]),
                points: vec![Point::from([1, 2]), Point::from([2, 3])],
            },
            Cluster {
                center: Point::from([9, 11]),
                points: vec![
                    Point::from([8, 10]),
                    Point::from([9, 11]),
                    Point::from([10, 12]),
                ],
            },
        ]);

        assert_eq!(expected_set, clusters_output_set);
    }

    #[test]
    fn test_kmeans_three_clusters() {
        let data = [[1, 1], [2, 2], [8, 8], [9, 9], [20, 20], [21, 21]].map(Point::from);
        let k = 3;

        let clusters_output = KmeansSerialBuilder::default()
            .with_data(data.clone())
            .with_k(k)
            .with_initial_centers(data.into_iter().take(k as usize))
            .execute();
        let clusters_output_set: HashSet<Cluster> = HashSet::from_iter(clusters_output.clone());

        let expected_set = HashSet::from_iter([
            Cluster {
                center: Point::from([1.5, 1.5]),
                points: vec![Point::from([1, 1]), Point::from([2, 2])],
            },
            Cluster {
                center: Point::from([8.5, 8.5]),
                points: vec![Point::from([8, 8]), Point::from([9, 9])],
            },
            Cluster {
                center: Point::from([20.5, 20.5]),
                points: vec![Point::from([20, 20]), Point::from([21, 21])],
            },
        ]);

        assert_eq!(expected_set, clusters_output_set);
    }
}
