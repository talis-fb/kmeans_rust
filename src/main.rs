mod entities;
mod kmeans;

fn main() {
    println!("nothing here.");
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::kmeans::KmeansSerialBuilder;

    use super::*;
    use entities::{Cluster, Point};

    #[test]
    fn test_kmeans_two_points() {
        let data = [Point::from([ 1, 2 ]), Point::from([ 5, 8 ])];
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
                center: Point::from([ 1, 2 ]),
                points: vec![Point::from([ 1, 2 ])],
            },
            Cluster {
                center: Point::from([ 5, 8 ]),
                points: vec![Point::from([ 5, 8 ])],
            },
        ]);

        assert_eq!(expected_set, clusters_output_set);
    }

    #[test]
    fn test_kmeans_few_points() {
        let data = [[ 1, 2 ], [ 2, 3 ], [ 8, 10 ], [ 9, 11 ], [ 10, 12 ]].map(Point::from);
        let k = 2;

        let clusters_output = KmeansSerialBuilder::default()
            .with_data(data.clone())
            .with_k(k)
            .with_initial_centers(data.into_iter().take(k as usize))
            .execute();
        let clusters_output_set: HashSet<Cluster> = HashSet::from_iter(clusters_output.clone());

        let expected_set = HashSet::from_iter([
            Cluster {
                center: Point::from([ 1.5, 2.5 ]),
                points: vec![Point::from([ 1, 2 ]), Point::from([ 2, 3 ])],
            },
            Cluster {
                center: Point::from([ 9, 11 ]),
                points: vec![
                    Point::from([ 8, 10 ]),
                    Point::from([ 9, 11 ]),
                    Point::from([ 10, 12 ]),
                ],
            },
        ]);

        assert_eq!(expected_set, clusters_output_set);
    }

    #[test]
    fn test_kmeans_three_clusters() {
        let data = [[ 1, 1 ], [ 2, 2 ], [ 8, 8 ], [ 9, 9 ], [ 20, 20 ], [ 21, 21 ]].map(Point::from);
        let k = 3;

        let clusters_output = KmeansSerialBuilder::default()
            .with_data(data.clone())
            .with_k(k)
            .with_initial_centers(data.into_iter().take(k as usize))
            .execute();
        let clusters_output_set: HashSet<Cluster> = HashSet::from_iter(clusters_output.clone());

        let expected_set = HashSet::from_iter([
            Cluster {
                center: Point::from([ 1.5, 1.5 ]),
                points: vec![Point::from([ 1, 1 ]), Point::from([ 2, 2 ])],
            },
            Cluster {
                center: Point::from([ 8.5, 8.5 ]),
                points: vec![Point::from([ 8, 8 ]), Point::from([ 9, 9 ])],
            },
            Cluster {
                center: Point::from([ 20.5, 20.5 ]),
                points: vec![Point::from([ 20, 20 ]), Point::from([ 21, 21 ])],
            },
        ]);

        assert_eq!(expected_set, clusters_output_set);
    }
}
