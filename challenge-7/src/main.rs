use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
struct Match {
    h: String,
    a: String,
    score: Score,
}

impl Match {
    fn new(h: &str, a: &str, score: &str) -> Self {
        let (l, r) = score.split_once('-').expect("Malformed input");
        let score = Score(
            l.parse().expect("Malformed input"),
            r.parse().expect("Malformed input"),
        );

        Match {
            h: h.into(),
            a: a.into(),
            score,
        }
    }
}

#[derive(Debug)]
struct Score(u8, u8);

fn main() {
    let mut rankings: HashMap<String, f64> = HashMap::new();
    let input = read_to_string("input.txt").expect("No input.txt in directory");
    let mut lines = input.trim().lines();

    lines.next();

    let matches: Vec<Match> = lines
        .map(|line| {
            let mut chunks = line.split(',');
            let h = chunks.next().expect("Malformed input");
            let a = chunks.next().expect("Malformed input");
            let score = chunks.next().expect("Malformed input");

            Match::new(h, a, score)
        })
        .collect();

    matches.iter().for_each(|game| {
        let Match { h, a, score } = game;
        let mut h_ranking = *rankings.get(h).unwrap_or(&1200f64);
        let mut a_ranking = *rankings.get(a).unwrap_or(&1200f64);
        let ranking_delta: f64;

        if score.0 < score.1 {
            ranking_delta = calculate_ranking_delta(h_ranking, a_ranking);

            h_ranking -= ranking_delta;
            a_ranking += ranking_delta;
        } else {
            ranking_delta = calculate_ranking_delta(a_ranking, h_ranking);

            h_ranking += ranking_delta;
            a_ranking -= ranking_delta;
        }

        {
            let h_mut = rankings.entry(h.clone()).or_default();
            *h_mut = h_ranking;
        }
        {
            let a_mut = rankings.entry(a.clone()).or_default();
            *a_mut = a_ranking;
        }
    });

    let min = rankings
        .values()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max = rankings
        .values()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    dbg!(max.floor() - min.floor());
    dbg!(rankings);
}

fn calculate_elo(a: f64, b: f64) -> f64 {
    1f64 / (1f64 + 10f64.powf((a - b) / 400f64))
}

fn calculate_ranking_delta(a: f64, b: f64) -> f64 {
    20f64 * (1f64 - calculate_elo(a, b))
}
