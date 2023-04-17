use anyhow::{Result};

#[derive(Clone, Copy, Debug)]
struct Pt<const N: usize> {
    x: [f32; N],
    y: f32
}

#[derive(Clone, Copy, Debug)]
struct WeightsBias<const N: usize> {
    weights: [f32; N],
    bias: f32
}

impl<const N: usize> Default for WeightsBias<N> {
    fn default() -> Self {
        Self {
            weights: [0.0; N],
            bias: Default::default()
        }
    }
}

fn compute_result<const N: usize>(pt: Pt<N>, weights_bias: WeightsBias<N>) -> f32 {
    let mut sum = weights_bias.bias;
    for q in 0..N {
        sum += pt.x[q] * weights_bias.weights[q];
    }

    sum
}

fn correct_weights_bias<const N: usize>(pt: Pt<N>, weights_bias: WeightsBias<N>) -> WeightsBias<N> {
    let mut new_weights_bias = weights_bias.clone();

    for q in 0..N {
        new_weights_bias.weights[q] += pt.x[q] * pt.y;
    }

    new_weights_bias.bias += pt.y;

    println!("Test data failed. Point: {:?}. Setting new weights and bias: {:?}", pt, new_weights_bias);

    new_weights_bias
}

fn perceptron_learn<const N: usize>(data: &Vec<Pt<N>>, starting_weights_bias: WeightsBias<N>) -> WeightsBias<N> {
    let count = data.len();
    let mut success_count = 0;
    let mut current_idx = 0;

    let mut weights_bias = starting_weights_bias;

    println!("Starting with weights {:?} and bias {}", weights_bias.weights, weights_bias.bias);

    while success_count < count {
        let result = compute_result(data[current_idx], weights_bias);
        if result * data[current_idx].y <= 0.0 {
            weights_bias = correct_weights_bias(data[current_idx], weights_bias);
            success_count = 0;
        }
        else {
            success_count += 1;
        }

        current_idx = (current_idx + 1) % count;
    }

    weights_bias
}

pub fn test_perceptron() -> Result<()> {
    println!("Hello, world!");

    let data = vec![
        Pt { x: [ -1.0, 1.0 ], y: 1.0 },
        Pt { x: [ 0.0, -1.0 ], y: -1.0 },
        Pt { x: [ 10.0, 1.0 ], y: 1.0 }
    ];

    let result = perceptron_learn(&data, Default::default());
    println!("Final weights: {:?}; and bias: {:?}", result.weights, result.bias);

    Ok(())
}
