pub fn generate_pairs(total_evaluations: usize, index: usize) -> Vec<(usize, usize)> {
    // Ensure total_evaluations is even
    assert!(total_evaluations % 2 == 0, "total_evaluations must be even");

    // Ensure index is valid
    assert!(index < total_evaluations / 2, "index must be less than total_evaluations / 2");

    let mut pairs_list = Vec::new();
    let step = usize::pow(2, index as u32);
    let block_size = total_evaluations / step;

    for block in 0..step {
        for pair_index in 0..block_size / 2 {
            let first_index = block * block_size + pair_index;
            let second_index = first_index + block_size / 2;
            pairs_list.push((first_index, second_index));
        }
    }

    pairs_list
}


#[cfg(test)]
mod tests {
    use crate::multilinear_pol::utiles::generate_pairs;



    #[test] 
    fn test_generate_pairs_index_1_evaluation_length_4() {
        let eval = 4;
        let ind = 1;

        assert_eq!(generate_pairs(eval, ind), [(0, 1), (2, 3)]);
    }

    #[test] 
    fn test_generate_pairs_index_0_evaluation_length_4() {
        let eval = 4;
        let ind = 0;

        assert_eq!(generate_pairs(eval, ind), [(0, 2), (1, 3)]);
    }

    #[test] 
    fn test_generate_pairs_index_2_evalaution_length_8() {
        let eval = 8;
        let ind = 2;

        assert_eq!(generate_pairs(eval, ind), [(0,1), (2,3), (4,5), (6,7)]);
    }

    #[test] 
    fn test_generate_pairs_1_evaluation_length_8() {
        let eval = 8;
        let ind = 1;

        assert_eq!(generate_pairs(eval, ind), [(0,2), (1,3), (4,6), (5,7)]);
    }




}