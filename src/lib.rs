pub mod lib {

    use std::{
        fmt::Debug,
        io::{self, Read},
        str::FromStr,
    };

    pub fn read_inputs<T: FromStr>() -> Vec<T>
    where
        T::Err: Debug,
    {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        buffer
            .split_ascii_whitespace()
            .map(|s| s.parse::<T>().unwrap())
            .collect()
    }
}
