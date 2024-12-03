mod input;
use anyhow::Result;

fn check_sortedness_w_removal<F>(levels: &[u32], idx: usize, cmp: F) -> Option<Vec<Vec<u32>>>
where
    F: Fn(&u32, &u32) -> bool,
    F: Copy,
{
    let mut vec1 = levels.to_vec();
    vec1.remove(idx);
    let vec1 = vec1.is_sorted_by(cmp).then_some(vec1);

    let mut vec2 = levels.to_vec();
    vec2.remove(idx + 1);
    let vec2 = vec2.is_sorted_by(cmp).then_some(vec2);

    match (vec1, vec2) {
        (Some(vec1), Some(vec2)) => Some(vec![vec1, vec2]),
        (None, Some(vec2)) => Some(vec![vec2]),
        (Some(vec1), None) => Some(vec![vec1]),
        _ => None,
    }
}

fn perform_check<F>(levels: &[u32], cmp: F) -> Option<Vec<Vec<u32>>>
where
    F: Fn(&u32, &u32) -> bool,
    F: Copy,
{
    match levels.windows(2).position(|s| !cmp(&s[0], &s[1])) {
        Some(idx) => check_sortedness_w_removal(levels, idx, cmp),
        None => Some(vec![]),
    }
}

fn check_sortedness(levels: &[u32]) -> Option<Vec<Vec<u32>>> {
    // Check if the list is sorted in ascending order
    let cmp = |a: &u32, b: &u32| a < b;
    let is_sorted_asc = perform_check(levels, cmp);

    if is_sorted_asc.is_some() {
        return is_sorted_asc;
    }

    // Check if the list is sorted in descending order
    let cmp = |a: &u32, b: &u32| a > b;
    perform_check(levels, cmp)
}

fn has_correct_diffs(levels: &[u32]) -> bool {
    let diff_is_correct = |s: &[u32]| (1..=3).contains(&s[0].abs_diff(s[1]));

    levels.windows(2).all(diff_is_correct)
}

fn has_correct_diffs_w_removal(levels: &[u32]) -> bool {
    let diff_is_correct = |s: &[u32]| (1..=3).contains(&s[0].abs_diff(s[1]));

    levels
        .windows(2)
        .position(|s| !diff_is_correct(s))
        .filter(|&idx| {
            // Check if removing any element will make the list safe
            let mut vec = levels.to_vec();
            vec.remove(idx);
            if vec.windows(2).all(diff_is_correct) {
                return false;
            }

            let mut vec = levels.to_vec();
            vec.remove(idx + 1);
            !vec.windows(2).all(diff_is_correct)
        })
        .is_none()
}

fn is_safe(levels: &[u32]) -> bool {
    let sorted = check_sortedness(levels);

    match sorted {
        Some(v) if v.is_empty() => has_correct_diffs_w_removal(levels),
        Some(v) => v.iter().map(|v| has_correct_diffs(v)).any(|b| b),
        None => false,
    }
}

fn main() -> Result<()> {
    let res = input::INPUT
        .lines()
        .map(|l| {
            let levels: Vec<u32> = l.split(' ').map(|num| num.parse().unwrap()).collect();
            is_safe(&levels)
        })
        .filter(|&b| b)
        .count();

    assert_eq!(res, 536);

    println!("{}", res);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_sortednes_with_removal() {
        assert_eq!(
            check_sortedness_w_removal(&[1, 2, 3], 0, |a, b| a < b),
            Some(vec![vec![2, 3], vec![1, 3]])
        );
        assert_eq!(
            check_sortedness_w_removal(&[3, 2, 1], 0, |a, b| a < b),
            None
        );
        assert_eq!(
            check_sortedness_w_removal(&[2, 3, 1, 4], 1, |a, b| a < b),
            Some(vec![vec![2, 3, 4]])
        );
        assert_eq!(
            check_sortedness_w_removal(&[1, 3, 2], 1, |a, b| a < b),
            Some(vec![vec![1, 2], vec![1, 3]])
        );
    }

    #[test]
    fn test_is_sorted() {
        assert_eq!(check_sortedness(&[1, 2, 3]), Some(vec![]));
        assert_eq!(check_sortedness(&[3, 2, 1]), Some(vec![]));
        assert_eq!(check_sortedness(&[2, 3, 1, 4]), Some(vec![vec![2, 3, 4]]));
        assert_eq!(
            check_sortedness(&[1, 3, 2]),
            Some(vec![vec![1, 2], vec![1, 3]])
        );
    }

    #[test]
    fn test_has_correct_diffs() {
        assert!(has_correct_diffs(&[1, 2, 3]));
        assert!(has_correct_diffs(&[3, 2, 1]));
        assert!(!has_correct_diffs(&[3, 2, 2]));
        assert!(!has_correct_diffs(&[1, 3, 3]));
        assert!(!has_correct_diffs(&[1, 3, 7]));
    }

    #[test]
    fn test_is_safe() {
        assert!(is_safe(&[1, 2, 3]));
        assert!(is_safe(&[1, 3, 5]));
        assert!(is_safe(&[3, 2, 1]));
        assert!(is_safe(&[1, 3, 2]));
        assert!(is_safe(&[7, 6, 4, 2, 1]));
        assert!(is_safe(&[1, 3, 2, 4, 5]));
        assert!(is_safe(&[8, 6, 4, 4, 1]));
        assert!(is_safe(&[8, 6, 4, 5, 1]));
        assert!(is_safe(&[1, 3, 6, 7, 9]));
        assert!(!is_safe(&[1, 2, 7, 8, 9]));
        assert!(!is_safe(&[9, 7, 6, 2, 1]));
        assert!(!is_safe(&[1, 7, 1]));
    }
}
