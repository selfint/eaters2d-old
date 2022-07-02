use bevy::prelude::*;

/// Very simple neural network implementation, with a flat parameter vector
/// for easier genetic algorithm implementation.
#[derive(Component)]
pub struct NeuralNetwork {
    pub input_size: usize,
    pub parameters: Vec<f32>,
    pub dimensions: Vec<usize>,
    pub activation_function: fn(f32) -> f32,
}

impl NeuralNetwork {
    fn forward(&self, inputs: &[f32]) -> Vec<f32> {
        let (weights, biases) = self.unpack_parameters();

        let mut prev_layer_outputs = inputs.to_vec();
        for (weights, biases) in weights.iter().zip(biases.iter()) {
            let mut layer_output = vec![0.0; biases.len()];

            for node_index in 0..biases.len() {
                let mut node_output = 0.;

                for (input_index, input) in prev_layer_outputs.iter().enumerate() {
                    node_output += weights[node_index][input_index] * input;
                }

                layer_output[node_index] =
                    (self.activation_function)(node_output) + biases[node_index];
            }

            prev_layer_outputs = layer_output;
        }

        prev_layer_outputs
    }

    fn unpack_parameters(&self) -> (Vec<Vec<Vec<f32>>>, Vec<Vec<f32>>) {
        let mut weights = Vec::new();
        let mut biases = Vec::new();
        let mut index = 0;

        // Unpack weights and biases
        let mut prev_layer_size = self.input_size;

        for &current_layer_size in self.dimensions.iter() {
            let mut layer_weights = Vec::new();
            let mut layer_biases = Vec::new();

            for _ in 0..current_layer_size {
                let mut row = Vec::new();

                for _ in 0..prev_layer_size {
                    row.push(self.parameters[index]);
                    index += 1;
                }

                layer_weights.push(row);
            }

            for _ in 0..current_layer_size {
                layer_biases.push(self.parameters[index]);
                index += 1;
            }

            weights.push(layer_weights);
            biases.push(layer_biases);

            prev_layer_size = current_layer_size;
        }

        (weights, biases)
    }
}

pub fn sigmoid_activation(x: f32) -> f32 {
    1. / (1. + (-x).exp())
}

pub fn relu_activation(x: f32) -> f32 {
    if x > 0. {
        x
    } else {
        0.
    }
}

pub fn linear_activation(x: f32) -> f32 {
    x
}

pub fn tanh_activation(x: f32) -> f32 {
    x.tanh()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_unpack_parameters() {
        let neural_network = NeuralNetwork {
            input_size: 2,
            parameters: vec![1., 2., 3.],
            dimensions: vec![1],
            activation_function: linear_activation,
        };

        let (weights, biases) = neural_network.unpack_parameters();

        assert_eq!(weights, vec![vec![vec![1., 2.]]]);
        assert_eq!(biases, vec![vec![3.]]);
    }

    #[test]
    fn test_neural_network() {
        let neural_network = NeuralNetwork {
            input_size: 2,
            parameters: vec![1.0, 0.0, 0.5],
            dimensions: vec![1],
            activation_function: sigmoid_activation,
        };

        let input = vec![1.0, 1.0];
        let output = neural_network.forward(&input);
        assert_eq!(output, vec![sigmoid_activation(1.) + 0.5]);
    }
}
