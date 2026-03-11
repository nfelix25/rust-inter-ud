use std::f32::consts::PI;

fn count_to_5() -> i32 {
    let mut not_foo = 0;
    loop {
        if not_foo > PI as i32 && not_foo > 5 {
            break;
        }
        not_foo += 1;
    }
    5
}

fn main() {
    println!("O can count to {}", count_to_5());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_counting() {
        assert_eq!(count_to_5() == 5, true);
    }
}
