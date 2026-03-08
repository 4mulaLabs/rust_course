use rand::seq::SliceRandom;

const MAX_CARDS: usize = 52;
const MAX_PLAYER_CARDS: usize = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    fn value(&self) -> u8 {
        match self {
            Suit::Clubs => 0,
            Suit::Diamonds => 1,
            Suit::Hearts => 2,
            Suit::Spades => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rank {
    Ace,
    Num(u8), // 2..=10
    Jack,
    Queen,
    King,
}

impl Rank {
    fn value(&self) -> u8 {
        match self {
            Rank::Ace => 1,
            Rank::Num(n) => *n,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }
}

fn find_card_position(deck: &[Card], card: &Card) -> Option<usize> {
    deck.iter().position(|c| c == card)
}


fn main() {

    let mut deck: Vec<Card> = Vec::with_capacity(MAX_CARDS);
    let suits  = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
    for suit in suits { //can not use suits after loop 
        deck.push(Card::new(Rank::Ace, suit));
    }
    for n in 2..=10 {
        for suit in suits {
            deck.push(Card::new(Rank::Num(n), suit));
        }
    }
    for suit in suits {
        deck.push(Card::new(Rank::Jack, suit));
    }
    for suit in suits {
        deck.push(Card::new(Rank::Queen, suit));
    }
    for suit in suits {
        deck.push(Card::new(Rank::King, suit));
    }


    let mut deck_shuffled = deck.clone();
    let mut rng = rand::rng();
    deck_shuffled.shuffle(&mut rng);


    let card = Card::new(Rank::Ace, Suit::Hearts);
    if let Some(pos) = find_card_position(&deck, &card) {
        println!("Card at index {}", pos);
    }

    let mut cards_playr_1: Vec<Card> = Vec::with_capacity(MAX_PLAYER_CARDS);
    for i in (0..MAX_PLAYER_CARDS){
        cards_playr_1.push(deck_shuffled.pop().unwrap());
    }

    let mut cards_playr_2: Vec<Card> = Vec::with_capacity(MAX_PLAYER_CARDS);
    for i in (0..MAX_PLAYER_CARDS){
        cards_playr_2.push(deck_shuffled.pop().unwrap());
    }

    let mut player1_score:u32 = 0;
    let mut player2_score:u32 = 0;
    for i in (0..MAX_PLAYER_CARDS){
        let player1_card : Card = cards_playr_1.pop().unwrap();
        let player2_card : Card = cards_playr_2.pop().unwrap();
        let pos1 = find_card_position(&deck, &player1_card).unwrap();
        let pos2 = find_card_position(&deck, &player2_card).unwrap();
       
        print!("Card 1 {:?} value is : {} |", player1_card,pos1);
        print!("Card 2 {:?} value is : {} |", player2_card,pos2);
        if (pos1>pos2){
            player1_score +=1;
            println!("Player 1 wins!");
        }else if (pos1 <pos2){
            player2_score +=1;
            println!("Player 2 wins!");
        }

        println!("Player 1 score {}  Player 2 score {}",player1_score,player2_score);
    }

    
}
         
