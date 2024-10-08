use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && (n & (n - 1)) == 0
    }

    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let n = nums.len();

        // Mark present numbers
        for i in 0..n {
            let index = (nums[i].abs() - 1) as usize;
            if nums[index] > 0 {
                nums[index] = -nums[index];
            }
        }

        // Collect unmarked indices
        (0..n as i32)
            .filter(|&i| nums[i as usize] > 0)
            .map(|i| i + 1)
            .collect()
    }

    pub fn reverse_vowels(s: String) -> String {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let mut chars: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut right = chars.len() - 1;

        while left < right {
            while left < right && !vowels.contains(&chars[left]) {
                left += 1;
            }
            while left < right && !vowels.contains(&chars[right]) {
                right -= 1;
            }
            if left < right {
                chars.swap(left, right);
                left += 1;
                right -= 1;
            }
        }

        chars.into_iter().collect()
    }

    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.unwrap());

        while !queue.is_empty() {
            let level_size = queue.len();
            let mut level_max = std::i32::MIN;

            for _ in 0..level_size {
                if let Some(node) = queue.pop_front() {
                    let node = node.borrow();
                    level_max = level_max.max(node.val);

                    if let Some(left) = &node.left {
                        queue.push_back(Rc::clone(left));
                    }
                    if let Some(right) = &node.right {
                        queue.push_back(Rc::clone(right));
                    }
                }
            }

            result.push(level_max);
        }

        result
    }

    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        image
            .into_iter()
            .map(|row| row.into_iter().rev().map(|pixel| 1 - pixel).collect())
            .collect()
    }

    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        // Helper function to collect leaf values
        fn collect_leaves(root: Option<Rc<RefCell<TreeNode>>>, leaves: &mut Vec<i32>) {
            if let Some(node) = root {
                let node = node.borrow();
                if node.left.is_none() && node.right.is_none() {
                    leaves.push(node.val);
                } else {
                    collect_leaves(node.left.clone(), leaves);
                    collect_leaves(node.right.clone(), leaves);
                }
            }
        }

        let mut leaves1 = Vec::new();
        let mut leaves2 = Vec::new();

        collect_leaves(root1, &mut leaves1);
        collect_leaves(root2, &mut leaves2);

        leaves1 == leaves2
    }

    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut result = vec![' '; s.len()];
        for (i, &index) in indices.iter().enumerate() {
            result[index as usize] = s.chars().nth(i).unwrap();
        }
        result.into_iter().collect()
    }

    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        points.windows(2).map(|window| {
            let from = &window[0];
            let to = &window[1];
            let dx = (from[0] - to[0]).abs();
            let dy = (from[1] - to[1]).abs();
            dx.max(dy)
        }).sum()
    }

    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        // Create a frequency map for elements in arr1
        let mut freq_map: HashMap<i32, usize> = HashMap::new();
        for &num in &arr1 {
            *freq_map.entry(num).or_insert(0) += 1;
        }

        // Create the result vector
        let mut result = Vec::with_capacity(arr1.len());

        // Add elements from arr2 to the result in order
        for &num in &arr2 {
            if let Some(&count) = freq_map.get(&num) {
                result.extend(std::iter::repeat(num).take(count));
                freq_map.remove(&num);
            }
        }

        // Add remaining elements from arr1 that are not in arr2
        let mut remaining: Vec<_> = freq_map.into_iter().collect();
        remaining.sort_unstable_by_key(|&(num, _)| num);

        for (num, count) in remaining {
            result.extend(std::iter::repeat(num).take(count));
        }

        result
    }

    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let (n, k, target) = (n as usize, k as usize, target as usize);
        
        // Initialize DP table
        let mut dp = vec![vec![0; target + 1]; n + 1];
        dp[0][0] = 1;
        
        // Fill the DP table
        for i in 1..=n {
            for j in 1..=target {
                for face in 1..=k.min(j) {
                    dp[i][j] = (dp[i][j] + dp[i-1][j-face]) % MOD;
                }
            }
        }
        
        dp[n][target]
    }

    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
            match node {
                Some(n) => {
                    let n = n.borrow();
                    if n.left.is_none() && n.right.is_none() && is_left {
                        return n.val;
                    }
                    dfs(&n.left, true) + dfs(&n.right, false)
                }
                None => 0,
            }
        }

        dfs(&root, false)
    }

    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n = cost.len();
        let mut dp = vec![0; n + 1];
        
        for i in 2..=n {
            dp[i] = (dp[i-1] + cost[i-1]).min(dp[i-2] + cost[i-2]);
        }
        
        dp[n]
    }

}

fn test_is_power_of_two() {
    println!("1 -> {}", Solution::is_power_of_two(1));
    println!("16 -> {}", Solution::is_power_of_two(16));
    println!("3 -> {}", Solution::is_power_of_two(3));
}

fn test_find_disappeared_numbers() {
    println!(
        "Disappeared numbers: {:?}",
        Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1])
    );
    println!(
        "Disappeared numbers: {:?}",
        Solution::find_disappeared_numbers(vec![1, 1])
    );
}

fn test_reverse_vowels() {
    println!(
        "IceCreAm -> {}",
        Solution::reverse_vowels("IceCreAm".to_string())
    );
    println!(
        "leetcode -> {}",
        Solution::reverse_vowels("leetcode".to_string())
    );
}

fn test_largest_values() {
    // Helper function to create a tree
    fn create_tree(vals: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(Rc::clone(&root));

        let mut i = 1;
        while !queue.is_empty() && i < vals.len() {
            let node = queue.pop_front().unwrap();
            let mut node = node.borrow_mut();

            if let Some(val) = vals[i] {
                let left = Rc::new(RefCell::new(TreeNode::new(val)));
                node.left = Some(Rc::clone(&left));
                queue.push_back(left);
            }
            i += 1;

            if i < vals.len() {
                if let Some(val) = vals[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.right = Some(Rc::clone(&right));
                    queue.push_back(right);
                }
                i += 1;
            }
        }

        Some(root)
    }

    let tree1 = create_tree(vec![
        Some(1),
        Some(3),
        Some(2),
        Some(5),
        Some(3),
        None,
        Some(9),
    ]);
    println!("Largest values: {:?}", Solution::largest_values(tree1));

    let tree2 = create_tree(vec![Some(1), Some(2), Some(3)]);
    println!("Largest values: {:?}", Solution::largest_values(tree2));
}

fn test_flip_and_invert_image() {
    let image1 = vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]];
    println!(
        "Flipped and inverted image 1: {:?}",
        Solution::flip_and_invert_image(image1)
    );

    let image2 = vec![
        vec![1, 1, 0, 0],
        vec![1, 0, 0, 1],
        vec![0, 1, 1, 1],
        vec![1, 0, 1, 0],
    ];
    println!(
        "Flipped and inverted image 2: {:?}",
        Solution::flip_and_invert_image(image2)
    );
}

fn test_leaf_similar() {
    // Helper function to create a tree (reuse from test_largest_values)
    fn create_tree(vals: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(Rc::clone(&root));

        let mut i = 1;
        while !queue.is_empty() && i < vals.len() {
            let node = queue.pop_front().unwrap();
            let mut node = node.borrow_mut();

            if let Some(val) = vals[i] {
                let left = Rc::new(RefCell::new(TreeNode::new(val)));
                node.left = Some(Rc::clone(&left));
                queue.push_back(left);
            }
            i += 1;

            if i < vals.len() {
                if let Some(val) = vals[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.right = Some(Rc::clone(&right));
                    queue.push_back(right);
                }
                i += 1;
            }
        }

        Some(root)
    }

    // Test case 1
    let tree1 = create_tree(vec![
        Some(3),
        Some(5),
        Some(1),
        Some(6),
        Some(2),
        Some(9),
        Some(8),
        None,
        None,
        Some(7),
        Some(4),
    ]);
    let tree2 = create_tree(vec![
        Some(3),
        Some(5),
        Some(1),
        Some(6),
        Some(7),
        Some(4),
        Some(2),
        None,
        None,
        None,
        None,
        None,
        None,
        Some(9),
        Some(8),
    ]);
    println!(
        "Leaf Similar (Example 1): {}",
        Solution::leaf_similar(tree1, tree2)
    );

    // Test case 2
    let tree3 = create_tree(vec![Some(1), Some(2), Some(3)]);
    let tree4 = create_tree(vec![Some(1), Some(3), Some(2)]);
    println!(
        "Leaf Similar (Example 2): {}",
        Solution::leaf_similar(tree3, tree4)
    );
}

fn test_restore_string() {
    println!(
        "codeleet -> {}",
        Solution::restore_string("codeleet".to_string(), vec![4, 5, 6, 7, 0, 2, 1, 3])
    );
    println!(
        "abc -> {}",
        Solution::restore_string("abc".to_string(), vec![0, 1, 2])
    );
}

fn test_min_time_to_visit_all_points() {
    println!(
        "Minimum time to visit [[1,1],[3,4],[-1,0]]: {}",
        Solution::min_time_to_visit_all_points(vec![vec![1,1], vec![3,4], vec![-1,0]])
    );
    println!(
        "Minimum time to visit [[3,2],[-2,2]]: {}",
        Solution::min_time_to_visit_all_points(vec![vec![3,2], vec![-2,2]])
    );
}

fn test_relative_sort_array() {
    println!(
        "Relative sort array: {:?}",
        Solution::relative_sort_array(
            vec![2,3,1,3,2,4,6,7,9,2,19],
            vec![2,1,4,3,9,6]
        )
    );
    println!(
        "Relative sort array: {:?}",
        Solution::relative_sort_array(
            vec![28,6,22,8,44,17],
            vec![22,28,8,6]
        )
    );
}

fn test_num_rolls_to_target() {
    println!("1 die, 6 faces, target 3: {}", Solution::num_rolls_to_target(1, 6, 3));
    println!("2 dice, 6 faces, target 7: {}", Solution::num_rolls_to_target(2, 6, 7));
    println!("30 dice, 30 faces, target 500: {}", Solution::num_rolls_to_target(30, 30, 500));
}

fn test_sum_of_left_leaves() {
    // Helper function to create a tree (reuse from previous tests)
    fn create_tree(vals: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(Rc::clone(&root));

        let mut i = 1;
        while !queue.is_empty() && i < vals.len() {
            let node = queue.pop_front().unwrap();
            let mut node = node.borrow_mut();

            if let Some(val) = vals[i] {
                let left = Rc::new(RefCell::new(TreeNode::new(val)));
                node.left = Some(Rc::clone(&left));
                queue.push_back(left);
            }
            i += 1;

            if i < vals.len() {
                if let Some(val) = vals[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.right = Some(Rc::clone(&right));
                    queue.push_back(right);
                }
                i += 1;
            }
        }

        Some(root)
    }

    // Test case 1: [3,9,20,null,null,15,7]
    let tree1 = create_tree(vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    println!("Sum of left leaves (Example 1): {}", Solution::sum_of_left_leaves(tree1));

    // Test case 2: [1]
    let tree2 = create_tree(vec![Some(1)]);
    println!("Sum of left leaves (Example 2): {}", Solution::sum_of_left_leaves(tree2));

    // Additional test case: [1,2,3,4,5]
    let tree3 = create_tree(vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
    println!("Sum of left leaves (Additional): {}", Solution::sum_of_left_leaves(tree3));
}

fn test_min_cost_climbing_stairs() {
    println!(
        "Minimum cost for [10,15,20]: {}",
        Solution::min_cost_climbing_stairs(vec![10, 15, 20])
    );
    println!(
        "Minimum cost for [1,100,1,1,1,100,1,1,100,1]: {}",
        Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1])
    );
}

fn main() {
    test_is_power_of_two();
    test_find_disappeared_numbers();
    test_reverse_vowels();
    test_largest_values();
    test_flip_and_invert_image();
    test_leaf_similar();
    test_restore_string();
    test_min_time_to_visit_all_points();
    test_relative_sort_array();
    test_num_rolls_to_target();
    test_sum_of_left_leaves();
    test_min_cost_climbing_stairs();
}
