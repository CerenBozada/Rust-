
struct Car {
    brand: String,
    model: String,
    year: u32,
}

impl Car {
    fn new(brand: String, model: String, year: u32) -> Car {
        Car { brand, model, year }
    }

    fn print_details(self) {
        println!("Brand: {}", self.brand);
        println!("Model: {}", self.model);
        println!("Year: {}", self.year);
    }
}

fn main() {
    let car = Car::new(String::from("Toyota"), String::from("Corolla"), 2022);
    car.print_details();
}
