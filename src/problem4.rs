/// --- Day 4: Secure Container ---

/// You arrive at the Venus fuel depot only to discover it's protected by a
/// password. The Elves had written the password on a sticky note, but someone
/// threw it out.

/// However, they do remember a few key facts about the password:

/// It is a six-digit number.
/// The value is within the range given in your puzzle input.
/// Two adjacent digits are the same (like 22 in 122345).
/// Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).


/// Other than the range rule, the following are true:

/// 111111 meets these criteria (double 11, never decreases).
/// 223450 does not meet these criteria (decreasing pair of digits 50).
/// 123789 does not meet these criteria (no double).

/// How many different passwords within the range given in your puzzle input meet these criteria?

/// --- Part Two ---

/// An Elf just remembered one more important detail: the two adjacent matching
/// digits are not part of a larger group of matching digits.

/// Given this additional criterion, but still ignoring the range rule, the
/// following are now true:

/// 112233 meets these criteria because the digits never decrease and all repeated digits are exactly two digits long.
/// 123444 no longer meets the criteria (the repeated 44 is part of a larger group of 444).
/// 111122 meets the criteria (even though 1 is repeated more than twice, it still contains a double 22).

/// How many different passwords within the range given in your puzzle input meet all of the criteria?

pub fn run() {
    let mut count1 = 0;
    let mut count2 = 0;
    let mut buf: [u8; 6] = [0; 6];

    for i in 264360..=746325 {
        buf.copy_from_slice(&i.to_string().as_bytes()[..6]);

        if is_valid_password1(&buf) {
            count1 += 1;
        }

        if is_valid_password2(&buf) {
            count2 += 1;
        }
    }

    println!("Num Valid Passwords (Part 1): {}", count1);
    println!("Num Valid Passwords (Part 2): {}", count2);
}

fn is_valid_password1(pw: &[u8; 6]) -> bool {
    return digits_are_monotonic(pw) && contains_repeat(pw)
}

fn is_valid_password2(pw: &[u8; 6]) -> bool {
    return digits_are_monotonic(pw) && contains_exact_repeat(pw);
}

fn digits_are_monotonic(pw: &[u8; 6]) -> bool {
    pw.windows(2).all(|x| x[0] <= x[1])
}

fn contains_repeat(pw: &[u8; 6]) -> bool{
    pw.windows(2).any(|x| x[0] == x[1])
}

fn contains_exact_repeat(pw: &[u8; 6]) -> bool {
    let at_start = (pw[0] == pw[1]) && (pw[1] != pw[2]);
    let at_end = (pw[4] == pw[5]) && (pw[3] != pw[4]);
    let internal = pw.windows(4).any(|x| {
        (x[0] != x[1]) && (x[1] == x[2]) && (x[2] != x[3])
    });

    at_start || at_end || internal
}
