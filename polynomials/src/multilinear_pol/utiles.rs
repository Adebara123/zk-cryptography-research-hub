

pub fn generate_pairs(total_evaluations: usize, index: usize) -> Vec<(usize, usize)> {
    // Ensure total_evaluations is even
    assert!(total_evaluations % 2 == 0, "total_evaluations must be even");

    // Ensure index is valid
    assert!(index < total_evaluations / 2, "index must be less than total_evaluations / 2");

    let mut pairs_list = Vec::new();
    let step = usize::pow(2, index as u32); 
    let pairs_per_round = total_evaluations / step;

    for i in 0..pairs_per_round / 2 {
        for j in 0..step {
            let first_index = i + j * pairs_per_round / 2;
            let second_index = first_index + pairs_per_round / 2;
            pairs_list.push((first_index, second_index));
        }
    }

    pairs_list
}


