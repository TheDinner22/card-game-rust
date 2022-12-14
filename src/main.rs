use card_game_rust::{helpers, my_random};
use card_game_rust::deck_trait::IsDeck;

struct Player {
    name: String,
    money: i32,

    hand: Vec<Card>,
    called: bool,
}

impl Player {
    fn new(name: String) -> Self {
        Player { name, money: 100, hand: vec![], called: false }
    }

    fn reset(&mut self) {
        self.called = false;
        self.clear_hand();
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

    fn points(&self) -> u8 {
        self.hand.iter().map(|c| c.point_value()).sum()
    }
}

// todo format for ez print
struct Card {
    suit: Suit,
    name: Name,
}

impl Card {
    fn new (suit: Suit, name: Name) -> Self {
        Card { suit, name }
    }

    fn point_value(&self) -> u8 {
        match self.name {
            Name::Two => 2,
            Name::Three => 3,
            Name::Four => 4,
            Name::Five => 5,
            Name::Six => 6,
            Name::Seven => 7,
            Name::Eight => 8,
            Name::Nine => 9,
            Name::Ten => 10,
            Name::Jack => 10,
            Name::Queen => 10,
            Name::King => 10,
            Name::Ace(val) => val,
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} of {}", self.name, self.suit)
    }
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
    Ace(u8),
}

impl Name {
    fn names_iterator () -> std::array::IntoIter<Name, 13_usize> {
        [
            Name::Two,
            Name::Three,
            Name::Four,
            Name::Five,
            Name::Six,
            Name::Seven,
            Name::Eight,
            Name::Nine,
            Name::Ten,
            Name::Jack,
            Name::Queen,
            Name::King,
            Name::Ace(1),
        ].into_iter()
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Name::Two => "Two",
            Name::Three => "Three",
            Name::Four => "Four",
            Name::Five => "Five",
            Name::Six => "Six",
            Name::Seven => "Seven",
            Name::Eight => "Eight",
            Name::Nine => "Nine",
            Name::Ten => "Ten",
            Name::Jack => "Jack",
            Name::Queen => "Queen",
            Name::King => "King",
            Name::Ace(_) => "Ace",
        };

        write!(f, "{name}")
    }
}

#[derive(Clone, Copy)]
enum Suit {
    Diamonds,
    Hearts,
    Clubs,
    Spades
}

impl Suit {
    fn suit_iterator () -> std::array::IntoIter<Suit, 4_usize> {
        [
            Suit::Diamonds,
            Suit::Hearts,
            Suit::Clubs,
            Suit::Spades
        ].into_iter()
    }
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suit = match self {
            Suit::Diamonds => "Diamonds",
            Suit::Hearts => "Hearts",
            Suit::Clubs => "Clubs",
            Suit::Spades => "Spades",
        };

        write!(f, "{suit}")
    }
}

fn make_deck() -> Vec<Card> { // TODO this could be much faster with iter and multithreading
    let mut deck = vec![];

    for suit in Suit::suit_iterator() {
        for name in Name::names_iterator() {
            deck.push( Card::new(suit, name) );
        }
    }

    deck    
}

impl IsDeck<Card> for Vec<Card> {
    fn random(&mut self) -> Card {
        let random = my_random::random();

        let length: f64 = (self.len()-1) as f64;

        let index: usize = (random * length).floor() as usize;

        self.swap_remove(index)
    }
}

fn get_players() -> Vec<Player> {
    let mut players = vec![];

    loop {
        let input = helpers::user_input(Some("to add a user, enter a username\nto start, type 's'\n~"));

        if input == "s" {
            if players.len() == 0 { println!("must have at least one player") }
            else { break; }
        }

        players.push( Player::new(input));
    }

    players
}

fn players_turn(deck: &mut Vec<Card>, players: &mut Vec<Player>) {
    // reset all players
    players.iter_mut().for_each(|p| p.reset());

    // loop until either:
    // everyones bust or everyones called, 
    loop {
        let mut any_player_went = false;
        for player in &mut *players {
            // are they bust?
            if player.points() > 21 { continue; }

            // have they called?
            if player.called == true { continue; }

            any_player_went = true;

            println!("\nIt is {}s turn.", player.name);
            let cards_vec: Vec<String> = player.hand.iter().map(|c| c.to_string()).collect();
            let cards_str: String;
            if cards_vec.len() == 0 {
                cards_str = String::from("your hand is empty");
            }
            else {
                cards_str = cards_vec.join(", ");
            }
            println!("you have {} point(s) and these cards in your hand:\n{}", player.points(), cards_str);
            

            // would you like a card or to call?
            let choice = helpers::restricted_user_input(Some("call or hit?:"), vec!["call", "hit"]);
            match choice.as_str() {
                "call" => player.called = true,
                "hit" => {
                    let card = deck.random();
                    let bust = if player.points() + card.point_value() > 21 { " you are bust!" } else {""};
                    println!("You pulled the {card}{bust}");
                    player.add_card(card);
                },
                _ => panic!("Somehow, helpers::restricted_user_input, did not work"),
            }
        }

        if !any_player_went {
            break
        }
    }
}

fn dealers_turn(deck: &mut Vec<Card>) -> Player {
    let mut dealer = Player::new("dealer".to_string());

    loop {
        // are they bust/have they called?
        if dealer.points() > 21 || dealer.called == true {
            println!("\nIt is the dealers turn!");
            let cards_vec: Vec<String> = dealer.hand.iter().map(|c| c.to_string()).collect();
            let cards_str = cards_vec.join("\n");

            println!("The dealer finished their turn with {} points and these cards in their hand:\n{}", dealer.points(), cards_str);

            break dealer;
        }        

        // will the dealer draw a card or to call?
        match dealer.points() {
            0..=16 => {
                // dealer hits
                let card = deck.random();
                let bust = if dealer.points() + card.point_value() > 21 { " the dealer is bust!" } else {""};
                println!("The dealer pulled the {card}{bust}");
                dealer.add_card(card);
            },
            17..=21 => {
                // dealer calls
                dealer.called = true
            },
            _ => panic!("this should never happen")
        };
    }
}

fn decide_winners(players: &Vec<Player>, dealer: &Player) {
    let dealer_points = dealer.points();
    let winners: Vec<&Player> = players.iter().filter(|p| p.points() <=21 && p.points() > dealer_points).collect();

    if winners.is_empty() && dealer_points >= 21 {
        println!("Their are no winners!!");
    }
    
    else if winners.is_empty() {
        println!("The dealer is the only winner here!")
    }

    else {
        let winners_names: Vec<&str> = winners.into_iter().map(|p| p.name.as_str()).collect();
        println!("These players won:\n{}", winners_names.join("\n"));
    }
}

fn main() { // TODO welcome ms
    // NOTE would the deck eventually empty???
    // start the game (init stuff get players)
    let mut players = get_players();
    
    
    // enter game loop
    loop {
        // create a new deck
        let mut deck = make_deck();
        
        // players turn (loop through players over and over unit )
        players_turn(&mut deck, &mut players);

        // dealers turn
        let dealer = dealers_turn(&mut deck);

        // money is distruted as needed/ winner is decided
        decide_winners(&players, &dealer);
    }
}
