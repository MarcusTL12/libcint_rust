use libc::{c_double, c_int};

extern "C" {
    pub fn CINTgto_norm(l: c_int, e: c_double) -> c_double;
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_norm() {
        let norm = unsafe { CINTgto_norm(1, 1.0) };

        println!("Test1: {}", norm);
    }
}
