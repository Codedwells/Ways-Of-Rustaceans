struct Temprature {
    defrees_f: f64,
}

// Lower case self  show there is already a temprature declared somewhere
// Uppercase Self
impl Temprature {
    fn freezing() -> Self {
        return Self { defrees_f: 32.5 };
    }

    fn show_temp(&self) {
        println!("The overall temp is :{}", self.defrees_f);
    }
}

fn main() {
    let hot = Temprature { defrees_f: 99.9 };

    // We can access the function using dot notion since we implimented the struct
    hot.show_temp();

    let cold = Temprature::freezing();
    cold.show_temp();
}
