use std::ops::{Add, Mul};

use crate::utils::*;

impl Circuit {
    pub fn new(layers: Vec<CircuitLayer>, num_inputs: usize) -> Self {
        Self { layers, num_inputs }
    }

    pub fn num_vars_at(&self, layer: usize) -> Option<usize> {
        let num_gates = if let Some(layer) = self.layers.get(layer) {
            layer.len()
        } else if layer == self.layers.len() {
            self.num_inputs
        } else {
            return None;
        };

        Some((num_gates as u64).trailing_zeros() as usize)
    }

    /// Evaluate a `Circuit` on a given input.
    pub fn evaluate<F>(&self, input: &[F]) -> CircuitEvaluation<F>
    where
        F: Add<Output = F> + Mul<Output = F> + Copy,
    {
        let mut layers = vec![];
        let mut current_input = input;

        layers.push(input.to_vec());

        for layer in self.layers.iter().rev() {
            let temp_layer: Vec<_> = layer
                .layer
                .iter()
                .map(|e| match e.g_type {
                    GateType::Add => current_input[e.inputs[0]] + current_input[e.inputs[1]],
                    GateType::Mul => current_input[e.inputs[0]] * current_input[e.inputs[1]],
                })
                .collect();

            layers.push(temp_layer);
            current_input = &layers[layers.len() - 1];
        }

        layers.reverse();
        CircuitEvaluation { layers }
    }

    /// The $\text{add}_i(a, b, c)$ polynomial value at layer $i$.
    pub fn add_i(&self, i: usize, a: usize, b: usize, c: usize) -> bool {
        let gate = &self.layers[i].layer[a];

        gate.g_type == GateType::Add && gate.inputs[0] == b && gate.inputs[1] == c
    }

    /// The $\text{mul}_i(a, b, c)$ polynomial value at layer $i$.
    pub fn mul_i(&self, i: usize, a: usize, b: usize, c: usize) -> bool {
        let gate = &self.layers[i].layer[a];

        gate.g_type == GateType::Mul && gate.inputs[0] == b && gate.inputs[1] == c
    }

    pub fn layers(&self) -> &[CircuitLayer] {
        &self.layers
    }

    pub fn num_outputs(&self) -> usize {
        self.layers[0].layer.len()
    }

    pub fn num_inputs(&self) -> usize {
        self.num_inputs
    }


}