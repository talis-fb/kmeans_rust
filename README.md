# KMeans Image Segmentation CLI

A Rust-based command-line tool implementing various [KMeans clustering algorithms](https://en.wikipedia.org/wiki/K-means_clustering) for image segmentation. 

It covers multiple implementations of the KMeans algorithm, both serial and parallel. For perfomance comparison and study purposes.

This tool is designed to work seamlessly with the [Image to CSV CLI project](https://github.com/talis-fb/img-to-csv). Use the img-to-csv tool to convert an image to CSV, process it with kmeans for segmentation, and convert the result back to an image.

### Java Version Available

For a Java-based implementation of this project, visit the [KMeans Java Repository](https://github.com/talis-fb/kmeans_java).

## Image Segmentations with Different K Values

To get a concrete example of result of this program...

| Base Image      |
| ------------- |
| <img src="https://res.cloudinary.com/dfjn94vg8/image/upload/v1716473684/kmeans/input.jpg" height="300px"> |


| K     | Image      |
| ------------- | ------------- |
| 2 | <img src="https://res.cloudinary.com/dfjn94vg8/image/upload/v1716473684/kmeans/output_2.jpg" height="300px"> |
| 5 | <img src="https://res.cloudinary.com/dfjn94vg8/image/upload/v1716473684/kmeans/output_5.jpg" height="300px"> |
| 10 | <img src="https://res.cloudinary.com/dfjn94vg8/image/upload/v1716473684/kmeans/output_10.jpg" height="300px"> |
| 25 | <img src="https://res.cloudinary.com/dfjn94vg8/image/upload/v1716473684/kmeans/output_25.jpg" height="300px"> |
| 60 | <img src="https://res.cloudinary.com/dfjn94vg8/image/upload/v1716473684/kmeans/output_60.jpg" height="300px"> |


### Example Workflow
As the tool uses STDIN and STDOUT for communication, you can use pipes and redirection to integrate it with `img-to-csv`.

This example processes an input_image.jpg image file and creates another image file called output_final_image.png, applying KMeans image segmentation with K equals 5:
```sh
img-to-csv to-csv input_image.jpg | kmeans -K 5 -m parallel | img-to-csv to-image -o output_final_image.png
```

### Workflow step-by-step
1. Convert image to CSV:
```sh
img-to-csv to-csv input_image.jpg > image.csv
```
2. Apply KMeans clustering:
```sh
kmeans -K 5 -m parallel < image.csv > segmented_image.csv
```
3. Convert CSV back to image:
```sh
img-to-csv to-image -o output_image.jpg < segmented_image.csv
```

## How It Works
The tool processes CSV files where each line represents a pixel's coordinates (X and Y) and RGB values:
```
X:Y R G B
```
* Input: CSV format from STDIN.
* Output: CSV format to STDOUT with modified RGB values representing cluster centers.
