// solution to https://leetcode.com/problems/two-sum

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let nums_clone = nums.clone();
    let mut snums: Vec<(usize, &i32)> = nums_clone.iter().enumerate().collect();
    snums.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());

    for (i, v) in snums.iter() {
        if let Ok(index) = snums[..].binary_search_by(|(_, probe)| probe.partial_cmp(&&(target - **v)).unwrap()) {
            let orig_index = snums.get(index).unwrap();
            if orig_index.0 != *i {
                return vec![(*i).try_into().unwrap(), orig_index.0.try_into().unwrap()];
            }

        }
    }
    panic!("no solution");
}


fn main() {
    two_sum(vec![2,7,11,15], 22);
    two_sum(vec![3,2,4], 6);
    two_sum(vec![3,3], 6);
}


#[cfg(test)]
mod test {

    use crate::two_sum;
    use test_case::test_case;


    #[test_case(vec![2,7,11,15], 9, vec![0,1])]
    #[test_case(vec![3,2,4], 6, vec![1,2])]
    #[test_case(vec![3,3], 6, vec![0,1])]
    fn test_two_sum(nums: Vec<i32>, target: i32, solution: Vec<i32>) {
        assert_eq!(two_sum(nums, target), solution);
    }
    

}

