#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod front_of_house;


fn serve_order() {}
// module back of the restaurant
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();   //calls from current crate when we use super
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn Summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();


    front_of_house::hosting::add_to_waitlist();


    let mut meal = back_of_house::Breakfast::Summer("Rye");
    meal.toast = String::from("wheat");

    println!("i would like to {} toast please", meal.toast);

}
