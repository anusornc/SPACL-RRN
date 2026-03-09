use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
struct SnapshotOperandFeatures {
    union_nodes: usize,
    intersection_nodes: usize,
    complement_nodes: usize,
    some_values_nodes: usize,
    all_values_nodes: usize,
    atom_nodes: usize,
    total_nodes: usize,
    max_depth: usize,
}

#[derive(Debug, Clone, Deserialize)]
struct BranchSnapshotRecord {
    policy_mode: String,
    operand_scores: Vec<f64>,
    operand_features: Vec<SnapshotOperandFeatures>,
}

#[derive(Debug, Clone, Serialize)]
struct TrainedRrnLinearWeights {
    bias: f64,
    union_weight: f64,
    intersection_weight: f64,
    complement_weight: f64,
    some_values_weight: f64,
    all_values_weight: f64,
    atom_weight: f64,
    depth_weight: f64,
    node_weight: f64,
    training_samples: usize,
    source_policy_mode: String,
    epochs: usize,
    learning_rate: f64,
}

fn print_usage() {
    eprintln!(
        "Usage: train_rrn_linear_model <snapshot.jsonl> <output_model.json> [policy_mode]\n\
         Example:\n\
         cargo run --bin train_rrn_linear_model -- \\\n           benchmarks/competitors/results/history/<RUN_ID>/branch_snapshots.jsonl \\\n           benchmarks/models/rrn_linear_model.json heuristic"
    );
}

fn feature_vector(f: &SnapshotOperandFeatures) -> [f64; 9] {
    [
        1.0,
        f.union_nodes as f64,
        f.intersection_nodes as f64,
        f.complement_nodes as f64,
        f.some_values_nodes as f64,
        f.all_values_nodes as f64,
        f.atom_nodes as f64,
        f.max_depth as f64,
        f.total_nodes as f64,
    ]
}

fn dot(weights: &[f64; 9], x: &[f64; 9]) -> f64 {
    weights.iter().zip(x.iter()).map(|(w, v)| w * v).sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print_usage();
        std::process::exit(2);
    }

    let input_path = &args[1];
    let output_path = &args[2];
    let policy_mode = if args.len() >= 4 {
        args[3].trim().to_string()
    } else {
        "heuristic".to_string()
    };

    let epochs: usize = env::var("RRN_TRAIN_EPOCHS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(400);
    let learning_rate: f64 = env::var("RRN_TRAIN_LR")
        .ok()
        .and_then(|v| v.parse::<f64>().ok())
        .filter(|v| *v > 0.0)
        .unwrap_or(0.00005);

    let input_file = fs::File::open(input_path)?;
    let reader = BufReader::new(input_file);

    let mut samples: Vec<([f64; 9], f64)> = Vec::new();
    let mut records_seen = 0usize;
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        records_seen += 1;
        let record: BranchSnapshotRecord = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(_) => continue,
        };
        if record.policy_mode != policy_mode {
            continue;
        }
        let n = record
            .operand_scores
            .len()
            .min(record.operand_features.len());
        for idx in 0..n {
            samples.push((
                feature_vector(&record.operand_features[idx]),
                record.operand_scores[idx],
            ));
        }
    }

    if samples.is_empty() {
        return Err(format!(
            "no training samples found for policy_mode='{}' in {} records from {}",
            policy_mode, records_seen, input_path
        )
        .into());
    }

    let mut weights = [0.0_f64; 9];
    for _ in 0..epochs {
        for (x, y) in &samples {
            let prediction = dot(&weights, x);
            let err = prediction - y;
            for j in 0..weights.len() {
                weights[j] -= learning_rate * err * x[j];
            }
        }
    }

    let mse = samples
        .iter()
        .map(|(x, y)| {
            let err = dot(&weights, x) - y;
            err * err
        })
        .sum::<f64>()
        / samples.len() as f64;

    let output = TrainedRrnLinearWeights {
        bias: weights[0],
        union_weight: weights[1],
        intersection_weight: weights[2],
        complement_weight: weights[3],
        some_values_weight: weights[4],
        all_values_weight: weights[5],
        atom_weight: weights[6],
        depth_weight: weights[7],
        node_weight: weights[8],
        training_samples: samples.len(),
        source_policy_mode: policy_mode,
        epochs,
        learning_rate,
    };

    if let Some(parent) = Path::new(output_path).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    fs::write(output_path, serde_json::to_string_pretty(&output)?)?;

    eprintln!(
        "[rrn-train] wrote model={} samples={} mse={:.6}",
        output_path,
        samples.len(),
        mse
    );

    Ok(())
}
