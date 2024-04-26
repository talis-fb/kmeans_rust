use std::error::Error;

use itertools::Itertools;

use std::sync::OnceLock;

use clap::Parser;
use kmeans::{parallel_2::KmeansParallelBuilder2, tokio::KmeansTokioBuilder, Kmeans};

use crate::{
    entities::Point,
    kmeans::{
        parallel::KmeansParallelBuilder, parallel_3::KmeansParallelStdBuilder,
        parallel_mutex::KmeansParallelMutex, serial::KmeansSerialBuilder,
    },
};

mod entities;
mod input;
mod kmeans;

fn input_data(data: Vec<Point>) -> &'static Vec<Point> {
    static COMPUTATION: OnceLock<Vec<Point>> = OnceLock::new();
    COMPUTATION.get_or_init(|| data)
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = input::Args::parse();

    let mut csv_builder = csv::ReaderBuilder::new();
    let csv_builder = csv_builder.has_headers(false).delimiter(b' ');

    let input_values: Vec<(String, u32, u32, u32)> = match matches.input_file {
        Some(path) => {
            let mut reader = csv_builder.from_path(path)?;
            reader
                .deserialize()
                .into_iter()
                .filter_map(Result::unwrap)
                .collect()
        }
        None => {
            let mut reader = csv_builder.from_reader(std::io::stdin());
            reader
                .deserialize()
                .into_iter()
                .filter_map(Result::unwrap)
                .collect()
        }
    };

    // Kmeans
    let k = matches.k;
    let values = input_values
        .into_iter()
        .map(|(label, x, y, z)| Point::from([x, y, z]).with_label(&label))
        .collect::<Vec<Point>>();
    let values = input_data(values);

    let initial_centers = if matches.random_initial {
        kmeans::common::get_n_random_points(values, k as usize)
    } else {
        values
            .iter()
            .unique_by(|p| p.get_values())
            .take(k as usize)
            .cloned()
            .collect()
    };

    let kmeans_runner: Box<dyn Kmeans> = match matches.mode {
        input::Mode::S => Box::new(KmeansSerialBuilder::default()),
        input::Mode::Par => Box::new(KmeansParallelStdBuilder { max_threads: 8 }),
        input::Mode::Mutex => Box::new(KmeansParallelMutex { max_threads: 8 }),
        input::Mode::Tokio => Box::new(KmeansTokioBuilder { max_threads: 8 }),
        input::Mode::Ray => Box::new(KmeansParallelBuilder2::default()),
        input::Mode::Ray2 => Box::new(KmeansParallelBuilder::default()),
    };

    let clusters = kmeans_runner.execute(&values, k as u8, initial_centers);

    let output_values = if matches.replace_entry {
        clusters
            .iter()
            .flat_map(|el| {
                el.points
                    .iter()
                    .map(|p| el.center.clone().with_label(p.get_label().unwrap_or("--")))
            })
            .map(|point| {
                let label = point.get_label().unwrap_or("--");
                let [x, y, z] = point.get_data().map(|n| n.max(0).min(255) as u8);

                vec![
                    label.to_string(),
                    x.to_string(),
                    y.to_string(),
                    z.to_string(),
                ]
            })
    } else {
        todo!()
    };

    // Write to output file
    let mut csv_writer = csv::WriterBuilder::new();
    let csv_write = csv_writer.has_headers(false).delimiter(b' ');

    match matches.output_file {
        Some(path) => {
            let mut writer = csv_write.from_path(path)?;
            for row in output_values {
                writer.write_record(row)?;
            }
        }
        None => {
            let mut writer = csv_write.from_writer(std::io::stdout());
            for row in output_values {
                writer.write_record(row)?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::kmeans::{serial::KmeansSerialBuilder, Kmeans};

    use super::*;
    use entities::{Cluster, Point};

    #[test]
    fn test_kmeans_two_points() {
        let data = vec![Point::from([1, 2]), Point::from([5, 8])];

        let k = 2;
        let initial_centers = data.iter().take(k as usize).cloned().collect();

        let clusters_output = KmeansSerialBuilder::default().execute(&data, k, initial_centers);

        let clusters_output_set: HashSet<Cluster> = HashSet::from_iter(clusters_output.clone());

        assert_eq!(clusters_output.len(), k as usize);

        // Expected Cluster 1
        let center = Point::from([1, 2]);
        let points = vec![Point::from([1, 2])];
        let cluster1 = Cluster {
            center,
            points: points.iter().collect(),
        };

        // Expected Cluster 2
        let center = Point::from([5, 8]);
        let points = vec![Point::from([5, 8])];
        let cluster2 = Cluster {
            center,
            points: points.iter().collect(),
        };

        let expected_set = HashSet::from_iter([cluster1, cluster2]);

        assert_eq!(expected_set, clusters_output_set);
    }

    #[test]
    fn test_kmeans_few_points() {
        let data = [[1, 2], [2, 3], [8, 10], [9, 11], [10, 12]]
            .map(Point::from)
            .to_vec();
        let k = 2;
        let initial_centers = data.iter().take(k as usize).cloned().collect();

        let clusters_output = KmeansSerialBuilder::default().execute(&data, k, initial_centers);

        let clusters_output_set: HashSet<Cluster> = HashSet::from_iter(clusters_output.clone());

        // Expected Cluster 1
        let center = Point::from([1.5, 2.5]);
        let points = vec![Point::from([1, 2]), Point::from([2, 3])];
        let cluster1 = Cluster {
            center,
            points: points.iter().collect(),
        };

        // Expected Cluster 2
        let center = Point::from([9, 11]);
        let points = vec![
            Point::from([8, 10]),
            Point::from([9, 11]),
            Point::from([10, 12]),
        ];
        let cluster2 = Cluster {
            center,
            points: points.iter().collect(),
        };

        let expected_set = HashSet::from_iter([cluster1, cluster2]);

        assert_eq!(expected_set, clusters_output_set);
    }

    #[test]
    fn test_kmeans_three_clusters() {
        let data = [[1, 1], [2, 2], [8, 8], [9, 9], [20, 20], [21, 21]]
            .map(Point::from)
            .to_vec();
        let k = 3;
        let initial_centers = data.iter().take(k as usize).cloned().collect();

        let clusters_output = KmeansSerialBuilder::default().execute(&data, k, initial_centers);

        let clusters_output_set: HashSet<Cluster> = HashSet::from_iter(clusters_output.clone());

        // Expected Cluster 1
        let center = Point::from([1.5, 1.5]);
        let points = vec![Point::from([1, 1]), Point::from([2, 2])];
        let cluster1 = Cluster {
            center,
            points: points.iter().collect(),
        };

        // Expected Cluster 2
        let center = Point::from([8.5, 8.5]);
        let points = vec![Point::from([8, 8]), Point::from([9, 9])];
        let cluster2 = Cluster {
            center,
            points: points.iter().collect(),
        };

        // Expected Cluster 2
        let center = Point::from([20.5, 20.5]);
        let points = vec![Point::from([20, 20]), Point::from([21, 21])];
        let cluster3 = Cluster {
            center,
            points: points.iter().collect(),
        };

        let expected_set = HashSet::from_iter([cluster1, cluster2, cluster3]);

        assert_eq!(expected_set, clusters_output_set);
    }
}
