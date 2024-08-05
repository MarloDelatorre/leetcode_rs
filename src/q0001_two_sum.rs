#[cfg(test)]
mod tests {
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

    fn create_solutions_for_test() -> Vec<Box<dyn Solution>> {
       vec![
            Box::new(BruteForceSolution {}),
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

