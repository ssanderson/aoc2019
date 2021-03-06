use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;
use std::str::FromStr;

// A result with a boxed error type. We use a box here to allow us to represent
// multiple different possible error types.
pub type BoxedErrorResult<T> = std::result::Result<T, Box<dyn Error>>;

pub type ProblemResult<T> = BoxedErrorResult<T>;

#[derive(Debug)]
pub struct SimpleError(pub String);

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for SimpleError {}

#[allow(dead_code)]
pub fn pp<T: fmt::Debug>(t: &T) {
    println!("{:#?}", t);
}

pub fn bail<T, S: Into<String>>(msg: S) -> ProblemResult<T> {
    Err(Box::new(SimpleError(msg.into())))
}

pub trait ProblemInput
where
    Self: Sized,
{
    fn for_problem(n: u64) -> ProblemResult<Self>;
}

pub fn read_problem_file(n: u64) -> ProblemResult<String> {
    let here = Path::new(file!()).parent().unwrap();
    let input_path = here.join(format!("inputs/problem{}_input.txt", n));
    Ok(fs::read_to_string(input_path)?)
}

impl<T: FromStr> ProblemInput for T
where
    <T as std::str::FromStr>::Err: Into<Box<dyn std::error::Error>>,
{
    fn for_problem(n: u64) -> ProblemResult<T> {
        let s = read_problem_file(n)?;
        match s.parse::<T>() {
            Ok(parsed) => Ok(parsed),
            Err(e) => Err(e.into()),
        }
    }
}

pub mod permute {
    pub struct Permutations<T: Clone> {
        values: Vec<T>,
        ix: usize,
        max_ix: usize,
    }

    impl<T: Clone> Permutations<T> {
        fn new(values: Vec<T>) -> Permutations<T> {
            let max_ix = (1..=values.len()).product();
            Permutations {
                values,
                ix: 0,
                max_ix,
            }
        }
    }

    impl<T: Clone> Iterator for Permutations<T> {
        type Item = Vec<T>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.ix == self.max_ix {
                None
            } else {
                let indices = get_permutation(self.ix, self.values.len());
                let out: Vec<T> = indices.iter().map(|&i| self.values[i].clone()).collect();

                self.ix += 1;
                Some(out)
            }
        }
    }

    pub fn permutations<T: Clone, C: Iterator<Item = T>>(values: C) -> Permutations<T> {
        Permutations::new(values.collect())
    }

    fn get_permutation(mut i: usize, size: usize) -> Vec<usize> {
        let mut modulus: usize = (1..=size).product();
        let mut out: Vec<usize> = (0..size).collect();
        let mut pos = 0;

        while pos < size {
            modulus /= size - pos;

            let choice = (i / modulus) as usize;
            out.swap(pos, pos + choice);

            pos += 1;
            i = i % modulus;
        }

        out
    }

    mod test {
        #[test]
        fn test_distinct_permutations() {
            use std::collections::HashSet;
            let set: HashSet<Vec<usize>> = super::permutations(0..5).collect();
            assert_eq!(set.len(), 120);

            for vec in set {
                assert_eq!(vec.iter().cloned().collect::<HashSet<usize>>().len(), 5);
            }
        }

        #[test]
        fn test_distinct_permutations_6() {
            use std::collections::HashSet;
            let set: HashSet<Vec<usize>> = super::permutations(0..6).collect();
            assert_eq!(set.len(), 720);

            for vec in set {
                assert_eq!(vec.iter().cloned().collect::<HashSet<usize>>().len(), 6);
            }
        }

        #[test]
        fn test_5_to_10() {
            use std::collections::HashSet;
            let set: HashSet<Vec<usize>> = super::permutations(5..10).collect();
            assert_eq!(set.len(), 120);
        }
    }
}

pub fn insert_or_merge<K, V, F>(map: &mut HashMap<K, V>, k: K, v: V, merge: F)
where
    F: FnOnce(V, V) -> V,
    K: std::hash::Hash + Eq,
    V: Clone,
{
    match map.get_mut(&k) {
        Some(value_ref) => {
            *value_ref = merge(value_ref.clone(), v);
        }
        None => {
            map.insert(k, v);
        }
    }
}
