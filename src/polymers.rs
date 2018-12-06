use std::collections::VecDeque;

pub fn select_partition(left: usize, right: usize) -> usize {
    (left + right) / 2
}

pub fn do_units_react(a: u8, b: u8) -> bool {
    let a_char = a as char;
    let b_char = b as char;

    a_char != b_char && a_char.to_ascii_lowercase() == b_char.to_ascii_lowercase()
}

#[inline]
fn is_ignored_polymer(p: u8, ignore: Option<u8>) -> bool {
    match ignore {
        Some(c) => {
            let p_char = p as char;
            let c_char = c as char;
            p_char.to_ascii_lowercase() == c_char
        }
        None => false,
    }
}

// `ignore` is only relevsant for part 2
pub fn solve_inner(s: &[u8], left: usize, right: usize, ignore: Option<u8>) -> VecDeque<u8> {
    if left == right {
        let mut l = VecDeque::new();
        if !is_ignored_polymer(s[left], ignore) {
            l.push_back(s[left]);
        }

        return l;
    }

    if right - left == 1 {
        return if do_units_react(s[left], s[right]) {
            VecDeque::new()
        } else {
            let mut l = VecDeque::new();
            if !is_ignored_polymer(s[left], ignore) {
                l.push_back(s[left]);
            }
            if !is_ignored_polymer(s[right], ignore) {
                l.push_back(s[right]);
            }

            l
        };
    }

    let partition = select_partition(left, right);

    let mut left = solve_inner(s, left, partition, ignore);
    let mut right = solve_inner(s, partition + 1, right, ignore);

    loop {
        let left_back = left.back();
        let right_front = right.front();
        if left_back.is_none() || right_front.is_none() {
            break;
        }

        if do_units_react(*left_back.unwrap(), *right_front.unwrap()) {
            left.pop_back();
            right.pop_front();
        } else {
            break;
        }
    }

    left.append(&mut right);

    left
}
