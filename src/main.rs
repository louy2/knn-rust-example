/// From https://huonw.github.io/blog/2014/06/comparing-knn-in-rust/#hows-it-compare

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
};

struct LabelPixel {
    label: i64,
    pixels: Vec<i64>,
}

fn slurp_file(file: &Path) -> Vec<LabelPixel> {
    BufReader::new(File::open(file).unwrap())
        .lines()
        .skip(1)
        .map(|line| {
            let line = line.unwrap();
            let mut iter = line.trim().split(',').map(|x| i64::from_str(x).unwrap());

            LabelPixel {
                label: iter.next().unwrap(),
                pixels: iter.collect(),
            }
        })
        .collect()
}

fn distance_sqr(x: &[i64], y: &[i64]) -> i64 {
    x.iter()
        .zip(y.iter())
        .fold(0, |s, (&a, &b)| s + (a - b) * (a - b))
}

fn classify(training: &[LabelPixel], pixels: &[i64]) -> i64 {
    training
        .iter()
        .min_by_key(|p| distance_sqr(p.pixels.as_ref(), pixels))
        .unwrap()
        .label
}

fn main() {
    let training_set = slurp_file(Path::new("./trainingsample.csv"));
    let validation_sample = slurp_file(Path::new("./validationsample.csv"));

    let num_correct = validation_sample
        .iter()
        .filter(|x| classify(training_set.as_ref(), x.pixels.as_ref()) == x.label)
        .count();
    println!("Percentage correct: {}%", num_correct as f64 / validation_sample.len() as f64 * 100.0);
}
