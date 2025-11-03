fn merge_sort(input: &[u64]) -> Vec<u64> {
    if input.len() <= 1 {
        return input.to_vec();
    }

    let half = input.len() / 2;
    let left = merge_sort(&input[..half]);
    let right = merge_sort(&input[half..]);

    merge(&left, &right)
}

fn merge(mut left: &[u64], mut right: &[u64]) -> Vec<u64> {
    let iter = std::iter::from_fn(move || match (left.split_first(), right.split_first()) {
        (None, None) => None,
        (None, Some((r, rest_r))) => {
            right = rest_r;
            Some(*r)
        }
        (Some((l, rest_l)), None) => {
            left = rest_l;
            Some(*l)
        }
        (Some((l, rest_l)), Some((r, rest_r))) => {
            if l <= r {
                left = rest_l;
                Some(*l)
            } else {
                right = rest_r;
                Some(*r)
            }
        }
    });

    iter.collect()
}

fn main() {
    let result = merge_sort(&[6, 5, 3, 1, 8, 7, 4, 2]);

    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(merge_sort(&[]), Vec::<u64>::new());
    }

    #[test]
    fn test_one_element() {
        assert_eq!(merge_sort(&[5]), vec![5]);
    }

    #[test]
    fn test_sorted() {
        assert_eq!(merge_sort(&[1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn test_reverse() {
        assert_eq!(merge_sort(&[3, 2, 1]), vec![1, 2, 3]);
    }

    #[test]
    fn test_with_duplicates() {
        assert_eq!(merge_sort(&[5, 3, 5, 1]), vec![1, 3, 5, 5]);
    }
}
