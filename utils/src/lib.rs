mod traits;

pub use traits::*;

pub mod djd2 {
    pub fn hash(string: &str) -> i32 {
        let mut hash = 0;

        for index in 0..string.len() {
            hash = string.chars().nth(index).unwrap() as i32 + ((hash << 5) - hash);
        }
        
        hash
    }
}