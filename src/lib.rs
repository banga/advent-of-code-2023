pub mod lib {

    use std::{
        fmt::Debug,
        io::{self, Read},
        str::FromStr,
    };

    pub fn read_to_string() -> String {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        buffer
    }

    pub fn read_lines_as<T: FromStr>() -> Vec<T>
    where
        T::Err: Debug,
    {
        read_to_string()
            .split_ascii_whitespace()
            .map(|s| s.parse::<T>().unwrap())
            .collect()
    }

    pub fn read_lines() -> Vec<String> {
        read_to_string().lines().map(|s| s.to_string()).collect()
    }

    pub fn read_byte_lines() -> Vec<Vec<u8>> {
        read_lines().iter().map(|s| s.as_bytes().into()).collect()
    }

    pub fn print_line(line: &Vec<u8>) {
        println!("{}", String::from_utf8(line.to_vec()).unwrap());
    }

    pub fn print_lines(lines: &Vec<Vec<u8>>) {
        for line in lines {
            print_line(line);
        }
    }
}
