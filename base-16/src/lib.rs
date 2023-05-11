// https://doc.rust-lang.org/std/fmt/trait.LowerHex.html

#[derive(Debug, Clone)]
pub struct Number<T: std::fmt::Debug+ Copy>(T);

impl<T> Number<T> where T: std::fmt::Debug + Copy {
    pub fn new(num: T)->Self{Number(num)} 
}

pub trait Base16 {
    fn to_base16(&self) -> &str;
}

impl<'a, T> Base16 for Number<T>
where
    T: std::fmt::Debug+ Copy,
{
    fn to_base16(&self) -> &str {
        let number = self.clone().0;

        
        // println!("{:?}", number);
        "2a"
    }
}

// #[cfg(test)]
#[test]
fn test_converting_decimal_number_to_base16() {
    let n = Number(16);
    n.to_base16();
}
