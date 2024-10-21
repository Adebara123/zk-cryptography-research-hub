#[derive(Clone, Copy, PartialEq)]
pub enum GateType {
    Add,
    Mul,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Gate {
    pub g_type: GateType,
    pub inputs: [usize; 2],
}

impl Gate {
    pub fn new(g_type: GateType, inputs: [usize; 2]) -> Self {
        Gate { g_type, inputs }
    }
}

// A layer of gates in the circuit.
#[derive(Clone)]
pub struct CircuitLayer {
   pub layer: Vec<Gate>,
}

impl CircuitLayer {
    /// Create a new `CircuitLayer`.
    pub fn new(layer: Vec<Gate>) -> Self {
        Self { layer }
    }

    /// The length of the layer.
    pub fn len(&self) -> usize {
        self.layer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.layer.is_empty()
    }
}


pub struct CircuitEvaluation<F> {
    /// Evaluations on per-layer basis.
    pub layers: Vec<Vec<F>>,
}

impl<F: Copy> CircuitEvaluation<F> {
    /// Takes a gate label and outputs the corresponding gate's value at layer `layer`.
    pub fn w(&self, layer: usize, label: usize) -> F {
        self.layers[layer][label]
    }
}

/// The circuit in layered form.
#[derive(Clone)]
pub struct Circuit {
    /// First layer being the output layer, last layer being
    /// the input layer.
    pub layers: Vec<CircuitLayer>,

    /// Number of inputs
    pub num_inputs: usize,
}
