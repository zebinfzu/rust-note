pub fn quick_sort<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() > 1 {
        quick_sort_range(arr, 0, arr.len() - 1);
    }
}

fn quick_sort_range<T: PartialOrd>(arr: &mut [T], left: usize, right: usize) {
    let mut right = right;
    while left < right {
        let pos = partition(arr, left, right);
        quick_sort_range(arr, pos + 1, right);
        if pos == 0 {
            break;
        }
        // 如果 pos == 0, pos - 1会发生溢出错误
        right = pos - 1;
    }
}

fn partition<T: PartialOrd>(arr: &mut [T], left: usize, right: usize) -> usize {
    // 默认选择左边作为划分点
    let pivot = left;
    let (mut left, mut right) = (left, right);
    while left < right {
        // 找到右边第一个不大于等于 arr[pivot] 的元素
        while left < right && arr[right] >= arr[pivot] {
            right -= 1;
        }
        // 找到左边第一个不小于等于 arr[pivot] 的元素
        while left < right && arr[left] <= arr[pivot] {
            left += 1;
        }
        // 交换找到的两个元素
        if left != right {
            arr.swap(left, right);
        }
    }
    arr.swap(pivot, left);
    // 返回正确的分割位置
    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vec() {
        let mut empty_vec: Vec<String> = vec![];
        quick_sort(&mut empty_vec);
        assert_eq!(empty_vec, Vec::<String>::new());
    }

    #[test]
    fn test_number_vec() {
        let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
        quick_sort(&mut vec);
        assert_eq!(vec, vec![7, 9, 23, 30, 44, 49, 58, 72, 73, 78]);
    }

    #[test]
    fn test_string_vec() {
        let mut vec = vec![
            String::from("Bob"),
            String::from("David"),
            String::from("Carol"),
            String::from("Alice"),
        ];
        quick_sort(&mut vec);
        assert_eq!(
            vec,
            vec![
                String::from("Alice"),
                String::from("Bob"),
                String::from("Carol"),
                String::from("David"),
            ]
        );
    }
}
