enum BoxColor {
    RED,
    BLUE,
    GREEN,
}
 
impl BoxColor {
    fn print_color(&self) {
        match self {
            BoxColor::RED => println!("The color of this box is : Red"),
            BoxColor::BLUE => println!("The color of this box is : Blue"),
            BoxColor::GREEN => println!("The color of this box is : Green"),
        }
    }
}

struct Dimensions {
    length: f64,
    width: f64,
    depth: f64,
}

impl Dimensions {
    fn print_dimensions(&self) {
        println!("The length is :{}", self.length);
        println!("The width is :{}", self.width);
        println!("The depth is :{}", self.depth);
    }
}

struct Shippingbox {
    dimensions: Dimensions,
    weight: f64,
    color: BoxColor,
}

impl Shippingbox {
    fn new(weight: f64, color: BoxColor, dimensions:Dimensions) -> Self {
        return Self {
            weight,
            dimensions,
            color,
        };
    }

    fn print_box(&self) {
        self.color.print_color();
        self.dimensions.print_dimensions();
        println!("The weight of this box is :{}", self.weight);
    }
}

fn main() {
    let dimensions = Dimensions {
        length: 34.0,
        width: 24.0,
        depth: 14.0,
    };

    let blue_box = Shippingbox::new(12.0, BoxColor::BLUE, dimensions);

    blue_box.print_box();
}
