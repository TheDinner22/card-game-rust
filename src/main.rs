use card_game_rust::helpers;

struct Player {
    name: String,
    money: i32,
    hand: Vec<Card>
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

// todo format for ez print
struct Card {
    suit: Suit,
    name: Name,
}

impl Card {
    fn new (suit: Suit, name: Name) -> Self {
        Card { suit, name }
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

fn make_deck() -> Vec<Card> { // TODO this could be much faster with iter and multithreading
    let mut deck = vec![];

    for suit in Suit::suit_iterator() {
        for name in Name::names_iterator() {
            deck.push( Card::new(suit, name) );
        }
    }

    deck    
}

fn get_players() -> Vec<Player> {
    let mut players = vec![];

    loop {
        let input = helpers::user_input(Some("to add a user, enter a username\nto start, type 's'\n~"));

        if input == "s" {break;}

        players.push( Player::new(input));
    }

    players
}

fn main() { // TODO welcome msg
    // start the game (init stuff get players)
    let _deck = make_deck();
    let _players = get_players();


    // enter game loop
    // loop {
        // players turn

        // dealers turn

        // money is distruted as needed
    // }
}
