use card_game_rust::helpers;

struct Player {
    name: String,
    money: i32,
    hand: Vec<Card>
}

// todo format for ez print
struct Card {
    color: Color,
    name: Name,
    points: u32    
}

enum Name {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

enum Color {
    Red,
    Black
}

impl Player {
    fn new(name: String) -> Self {
        Player { name, money: 100, hand: vec![] }
    }

    fn clear_hand(&mut self) {
        self.hand = vec![];
    }

    fn add_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    fn add_cards(&mut self, mut cards: Vec<Card>) {
        self.hand.append(&mut cards);
    }
}

fn main() {

    let thingy = helpers::user_input(Some("put a thing"));
    println!("{thingy} is what you just typed");

    // start the game (init stuff get players)

    // enter game loop
    // loop {
        // players turn

        // dealers turn

        // money is distruted as needed
    // }
}
