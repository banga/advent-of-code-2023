pub mod lib {

    use std::{
        fmt::{Debug, Display},
        io::{self, Read},
        ops::{Add, AddAssign, Mul, Sub, SubAssign},
        str::FromStr,
    };

    use num::ToPrimitive;

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

    pub fn ascii_to_string(s: &[u8]) -> String {
        String::from_utf8(s.to_vec()).unwrap()
    }

    pub fn print_line(line: &[u8]) {
        println!("{}", ascii_to_string(line));
    }

    pub fn print_lines(lines: &Vec<Vec<u8>>) {
        for line in lines {
            print_line(line);
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Point<T> {
        pub x: T,
        pub y: T,
    }

    impl<T> Point<T> {
        pub fn from(x: T, y: T) -> Self {
            Self { x, y }
        }

        pub fn min(&self, other: &Point<T>) -> Point<T>
        where
            T: Ord + Copy,
        {
            Point {
                x: self.x.min(other.x),
                y: self.y.min(other.y),
            }
        }

        pub fn max(&self, other: &Point<T>) -> Point<T>
        where
            T: Ord + Copy,
        {
            Point {
                x: self.x.max(other.x),
                y: self.y.max(other.y),
            }
        }
    }

    impl<T> Display for Point<T>
    where
        T: Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("({}, {})", self.x, self.y))
        }
    }

    impl<T> Add<Self> for &Point<T>
    where
        T: Add<T, Output = T> + Copy,
    {
        type Output = Point<T>;

        fn add(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl<T> AddAssign<&Point<T>> for Point<T>
    where
        T: AddAssign + Copy,
    {
        fn add_assign(&mut self, rhs: &Point<T>) {
            self.x += rhs.x;
            self.y += rhs.y;
        }
    }

    impl<T> Sub<Self> for &Point<T>
    where
        T: Sub<T, Output = T> + Copy,
    {
        type Output = Point<T>;

        fn sub(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }

    impl<T> SubAssign<&Point<T>> for Point<T>
    where
        T: SubAssign + Copy,
    {
        fn sub_assign(&mut self, rhs: &Point<T>) {
            self.x -= rhs.x;
            self.y -= rhs.y;
        }
    }

    impl<T, U> Mul<U> for &Point<T>
    where
        T: Mul<U> + Copy,
        U: Copy,
    {
        type Output = Point<T::Output>;

        fn mul(self, rhs: U) -> Self::Output {
            Point {
                x: self.x * rhs,
                y: self.y * rhs,
            }
        }
    }

    pub struct Grid<V> {
        pub values: Vec<Vec<V>>,
    }

    impl<V> Grid<V> {
        pub fn height(&self) -> isize {
            self.values.len() as isize
        }
        pub fn width(&self) -> isize {
            self.values[0].len() as isize
        }
    }

    impl<V> Grid<V>
    where
        V: Default + Clone + Copy,
    {
        pub fn new<T>(width: T, height: T, default: V) -> Self
        where
            T: ToPrimitive,
        {
            Self {
                values: vec![vec![default; width.to_usize().unwrap()]; height.to_usize().unwrap()],
            }
        }

        pub fn get<T>(&self, point: &Point<T>) -> &V
        where
            T: ToPrimitive,
        {
            &self.values[point.y.to_usize().unwrap()][point.x.to_usize().unwrap()]
        }

        pub fn set<T>(&mut self, point: &Point<T>, value: V)
        where
            T: ToPrimitive,
        {
            self.values[point.y.to_usize().unwrap()][point.x.to_usize().unwrap()] = value;
        }
    }

    impl Display for Grid<u8> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for line in &self.values {
                f.write_str(&ascii_to_string(&line))?;
                f.write_str("\n")?;
            }
            Ok(())
        }
    }
}
