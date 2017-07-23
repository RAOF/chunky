#[cfg(test)]
#[macro_use]
extern crate quickcheck;

fn next_cutpoint(input : &[u8], window : usize) -> usize {
    struct Candidate {
        value : u8,
        position : usize
    }
    let mut max = Candidate {
        value : input[0],
        position : 0
    };
    for (pos, val) in input.iter().enumerate() {
        if *val <= max.value {
            if pos == max.position + window + 1 {
                return pos
            }
        }
        else {
            max.value = *val;
            max.position = pos;
        }
    }
    input.len()
}

pub fn chunk(input: &[u8], window: usize) -> Vec<&[u8]> {
    if input.len() == 0 {
        return vec![input];
    }

    let mut chunked_input = Vec::new();
    let mut current_pos : usize = 0;
    let mut next_pos = next_cutpoint(input, window);

    while next_pos < input.len() {
        println!("current_pos: {:?}, next_pos: {:?}", current_pos, next_pos);
        chunked_input.push(&input[current_pos..next_pos]);
        current_pos = next_pos;
        next_pos = next_cutpoint(&input[current_pos..], window) + current_pos;
    }

    chunked_input.push(&input[current_pos .. input.len()]);

    chunked_input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn no_cutpoint_in_string_returns_end() {
        assert_eq!(next_cutpoint(&[1, 2, 3, 4, 5], 2), 5);
    }

    #[test]
    fn decreasing_string_has_cutpoint_equal_to_window_size() {
        let data : &[u8] = &[10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        for window in 1..(data.len() - 1) {
            assert_eq!(next_cutpoint(data, window), window + 1);
        }
    }

    quickcheck! {
        fn sum_of_chunk_lengths_is_original_length(data: Vec<u8>, window : usize) -> quickcheck::TestResult {
            if window < 1 {
                return quickcheck::TestResult::discard();
            }
            let mut total_length = 0;
            for c in chunk(&data, window) {
                total_length = total_length + c.len();
            }
            quickcheck::TestResult::from_bool(total_length == data.len())
        }

        fn chunked_input_merges_to_input(data: Vec<u8>, window : usize) -> quickcheck::TestResult {
            if window < 1 {
                return quickcheck::TestResult::discard();
            }

            let fused_chunks = chunk(&data, window).into_iter().flat_map(|chunk| chunk.iter().map(|item| *item)).collect::<Vec<u8>>();

            quickcheck::TestResult::from_bool(fused_chunks == data)
        }
   }
}
