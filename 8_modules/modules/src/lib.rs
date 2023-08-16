mod front_of_house {

    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Adding to wait list");
        }
    }
}

mod back_of_house {
    // Struct fields are by default private
    // This is different for enums where fields are by default  public
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

// use is used to bring modules into scope
use self::back_of_house::Breakfast;
// using pub allows external code to access the function
pub use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    let mut meal = Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    //Absolute paths
    hosting::add_to_waitlist();

    //Relative path
    front_of_house::hosting::add_to_waitlist();
}
