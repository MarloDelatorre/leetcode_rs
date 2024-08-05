#[cfg(test)]
mod tests {
    use std::collections::hash_map::HashMap;

    trait Solution {
        fn two_sum(&self, nums: Vec<i32>, target: i32) -> Vec<i32>;
    }

    struct BruteForceSolution {}

    impl Solution for BruteForceSolution {
        fn two_sum(&self, nums: Vec<i32>, target: i32) -> Vec<i32> {
            for i in 0..nums.len() {
                for j in (i + 1)..nums.len() {
                    if nums[i] + nums[j] == target {
                        return vec![i as i32, j as i32];
                    }
                }
            }

            panic!("No Solution");
        }
    }

    struct MapSolution {}

    impl Solution for MapSolution {
        fn two_sum(&self, nums: Vec<i32>, target: i32) -> Vec<i32> {
            let mut diff_to_index: HashMap<i32, i32> = HashMap::new();

            for (index, value) in nums.iter().enumerate() {
                let diff = target - value;
                let matching = diff_to_index.get(&diff);                

                match matching {
                    Some(&other_index) => return vec![
                        other_index,
                        index as i32,
                    ],
                    None => diff_to_index.insert(*value, index as i32),
                };
            }

            panic!("No Solution");
        }
    }

    fn create_solutions_for_test() -> Vec<Box<dyn Solution>> {
       vec![
            Box::new(BruteForceSolution {}),
            Box::new(MapSolution {}),
       ] 
    }

    #[test]
    fn given_case() {
        for sol in create_solutions_for_test().iter() {
            let actual = sol.two_sum(vec![2, 7, 11, 15], 9);

            assert_eq!(actual, vec![0, 1]);
        } 
    }

    #[test]
    fn not_same_index() {
        for sol in create_solutions_for_test().iter() {
            let actual = sol.two_sum(vec![2, 7, 2, 15], 4);

            assert_ne!(actual, vec![0, 0]);
            assert_eq!(actual, vec![0, 2]);
        } 
    }

    #[test]
    fn min_len() {
        for sol in create_solutions_for_test().iter() {
            let actual = sol.two_sum(vec![2, 1], 3);

            assert_eq!(actual, vec![0, 1])
        }
    }
}

