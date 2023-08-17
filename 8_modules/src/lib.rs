mod front_of_house;
mod back_of_house;

// Nested paths
use std::io::{self,Write};


pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    //Absolute paths
    front_of_house::hosting::add_to_waitlist();

    //Relative path
    front_of_house::hosting::add_to_waitlist();
}
