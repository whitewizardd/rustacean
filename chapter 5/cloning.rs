


// when a functiion needs the ownership of a value and also the calling variable , we can implement clone.

// there are different ways to implement clone 

// using the derive 
#[derive(Clone)]
struct Fraction {
    pub numerator: usize,
    pub denominator: usize,
}



struct Fraction {
    pub numerator: usize,
    pub denominator: usize,
}

impl Clone for Fraction {

    fn clone(&self) -> Self {
        Fraction {
            numerator: self.numerator.clone(),
            denominator: self.denominator.clone(),
        }
    }
 }