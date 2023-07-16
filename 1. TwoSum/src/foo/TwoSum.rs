use std::collections::HashMap;

mod TwoSum {}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums_map: HashMap<i32, usize> = HashMap::new();

    for i in 0..nums.len() {
      match nums_map.get(&nums[i]) {
        Some(value) => return vec![*value as i32, i as i32],
        None => nums_map.insert(target - nums[i], i),
      };
    }

    panic!()
}

pub fn two_sum_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums_map = HashMap::new();

    for (index, value) in nums.iter().enumerate() {
        nums_map.insert(*value, index);
    }

    for (index, value) in nums.iter().enumerate() {
        let remainder = target - value;
        if nums_map.contains_key(&remainder) {
            let first_index = index as i32;
            let second_index = nums_map[&remainder] as i32;

            if first_index != second_index {
                return vec![first_index, second_index];
            }
        }
    }

    return vec![];
}

pub fn two_sum_nlogn(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut sorted_nums: Vec<position> = nums
        .iter()
        .enumerate()
        .map(|(index, num)| position {
            index: index as i32,
            value: *num,
        })
        .collect();

    sorted_nums.sort_by(|a, b| a.value.cmp(&b.value));
    let mut last_point = sorted_nums.len() - 1;
    let mut start_point = 0;

    while start_point < last_point {
        let sum = sorted_nums[start_point].value + sorted_nums[last_point].value;
        if sum == target {
            return vec![
                sorted_nums[start_point].index as i32,
                sorted_nums[last_point].index as i32,
            ];
        }
        if sum > target {
            last_point -= 1;
        } else {
            start_point += 1;
        }
    }

    return vec![];

    struct position {
        value: i32,
        index: i32,
    }
}

pub fn two_sum_n2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (index, number) in nums.iter().enumerate() {
        for (index2, number2) in nums.iter().enumerate() {
            if index == index2 {
                continue;
            }
            if number + number2 == target {
                return vec![index as i32, index2 as i32];
            }
        }
    }
    nums
}
