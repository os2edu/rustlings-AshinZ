// options1.rs
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a hint.


use std::time;

// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
// TODO: Return an Option!
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a value of 0
    // The Option output should gracefully handle cases where time_of_day > 23.
    if time_of_day < 22 {
        Option::Some(5)
    } else if time_of_day >23{
        Option::None
    } else {
        Option::Some(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the Option?
        let icecreams = maybe_icecream(12);
        let mut a:u16 = 0;
        match icecreams{
            Option::None=>{
                a = 0
            },
            Option::Some(num)=>{
                a = num
            },
        }
        assert_eq!(a, 5);
    }
}
