fn main()
{
    enum Coin {
        penny,
        nickel,
        Dime,
        Quarter(UsState), 

    }
    
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    fn value_in_cents(coin : Coin) -> u8 {
        match coin {
            Coin::penny => {
                println!("Lucky Penny");
                1
            },
            Coin::nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("state quarter from {:?}",state);
                25
                }
        }
    }

    println!("value in cents {}", value_in_cents(Coin::Quarter(UsState::Alaska)));
}
