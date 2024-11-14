use core::panic;
use std::{cmp::{max, min}, hash::Hash, iter::once};

/// Packed single-byte representation of a FF card, or of a few other states needed for the algorithm.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct C(pub u8);

// Constants defining the numeric ranges allocated to each category of card.
pub const EVERY_BASE: u8 = 0;

    pub const CARDS_BASE: u8 = EVERY_BASE;
        pub const WANDS_BASE: u8 = CARDS_BASE;
        pub const WANDS_COUNT: u8 = 12;
        pub const WANDS_HIGH: u8 = WANDS_BASE + WANDS_COUNT - 1;
        pub const STARS_BASE: u8 = WANDS_HIGH + 1;
        pub const STARS_COUNT: u8 = 12;
        pub const STARS_HIGH: u8 = STARS_BASE + STARS_COUNT - 1;
        pub const SWRDS_BASE: u8 = STARS_HIGH + 1;
        pub const SWRDS_COUNT: u8 = 12;
        pub const SWRDS_HIGH: u8 = SWRDS_BASE + SWRDS_COUNT - 1;
        pub const CUUPS_BASE: u8 = SWRDS_HIGH + 1;
        pub const CUUPS_COUNT: u8 = 12;
        pub const CUUPS_HIGH: u8 = CUUPS_BASE + CUUPS_COUNT - 1;
        pub const MAGIC_BASE: u8 = CUUPS_HIGH + 1;
        pub const MAGIC_COUNT: u8 = 22;
        pub const MAGIC_HIGH: u8 = MAGIC_BASE + MAGIC_COUNT - 1;
    pub const CARDS_HIGH: u8 = MAGIC_HIGH;
    pub const CARDS_COUNT: u8 = CARDS_HIGH - CARDS_BASE + 1;

    pub const OTHER_BASE: u8 = CARDS_HIGH + 1;
        pub const TABLE_BYTE: u8 = OTHER_BASE;
        pub const FREEC_BYTE: u8 = TABLE_BYTE + 1;
        pub const FOUND_BYTE: u8 = FREEC_BYTE + 1;
        pub const MAJHI_BYTE: u8 = FOUND_BYTE + 1;
        pub const NOCRD_BYTE: u8 = MAJHI_BYTE + 1;
    pub const OTHER_HIGH: u8 = NOCRD_BYTE;
    pub const OTHER_COUNT: u8 = OTHER_HIGH - OTHER_BASE + 1;

pub const EVERY_HIGH: u8 = OTHER_HIGH;
pub const EVERY_COUNT: u8 = EVERY_HIGH - EVERY_BASE + 1;


#[allow(dead_code)]
impl C {
    pub const TABLEAU: Self = Self(TABLE_BYTE);
    pub const FREECELL: Self = Self(FREEC_BYTE);
    pub const DOWNFOUNDN: Self = Self(MAJHI_BYTE);
    pub const FOUNDATION: Self = Self(FOUND_BYTE);
    pub const NO_CARD: Self = Self(NOCRD_BYTE);

    pub const WANDS_KING: Self = Self(WANDS_HIGH);
    pub const STARS_KING: Self = Self(STARS_HIGH);
    pub const SWRDS_KING: Self = Self(SWRDS_HIGH);
    pub const CUUPS_KING: Self = Self(CUUPS_HIGH);
    pub const MAGIC_FOOL: Self = Self(MAGIC_BASE);
    pub const MAGIC_WORLD: Self = Self(MAGIC_HIGH);
}
impl Default for C {
    fn default() -> Self {
        Self::NO_CARD
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Wands = 0,
    Stars = 1,
    Swrds = 2,
    Cuups = 3,
    Magic = 4,
}
impl Suit {
    pub fn is_minor(&self) -> bool {
        match self {
            Wands => true,
            Stars => true,
            Swrds => true,
            Cuups => true,
            Magic => false,
        }
    }
}

use Suit::*;
/// Unpacked information for a card value or card state.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardInfo {
    Card(Suit, u8),
    Tableau,
    Freecell,
    Foundation,
    DownFoundn,
    NoCard,
}
use CardInfo::*;
impl CardInfo {
    pub fn is_card(&self) -> bool {
        match self {
            Card(_, _) => true,
            _ => false,
        }
    }
    pub fn card_suit(&self) -> Suit {
        match self {
            Card(suit, _) => *suit,
            _ => panic!("not a card"),
        }
    }
    pub fn card_rank(&self) -> u8 {
        match self {
            Card(_, rank) => *rank,
            _ => panic!("not a card"),
        }
    }
}

impl CardInfo {
    pub fn pack(self) -> C {
        C(match self {
            Card(Wands, rank @ 2..=13) => WANDS_BASE + (rank - 2),
            Card(Stars, rank @ 2..=13) => STARS_BASE + (rank - 2),
            Card(Swrds, rank @ 2..=13) => SWRDS_BASE + (rank - 2),
            Card(Cuups, rank @ 2..=13) => CUUPS_BASE + (rank - 2),
            Card(Magic, rank @ 0..=21) => MAGIC_BASE + rank,
            Card(_, _) => panic!("no packed representation for {:?}", self),
            Tableau => TABLE_BYTE,
            Freecell => FREEC_BYTE,
            DownFoundn => MAJHI_BYTE,
            Foundation => FOUND_BYTE,
            NoCard => NOCRD_BYTE,
        })
    }
}
impl C {
    pub fn info(self) -> CardInfo {
        match self.0 {
            WANDS_BASE..=WANDS_HIGH => Card(Wands, self.0 + 2 - WANDS_BASE),
            STARS_BASE..=STARS_HIGH => Card(Stars, self.0 + 2 - STARS_BASE),
            SWRDS_BASE..=SWRDS_HIGH => Card(Swrds, self.0 + 2 - SWRDS_BASE),
            CUUPS_BASE..=CUUPS_HIGH => Card(Cuups, self.0 + 2 - CUUPS_BASE),
            MAGIC_BASE..=MAGIC_HIGH => Card(Magic, self.0 - MAGIC_BASE),
            TABLE_BYTE => Tableau,
            FREEC_BYTE => Freecell,
            MAJHI_BYTE => DownFoundn,
            FOUND_BYTE => Foundation,
            NOCRD_BYTE => NoCard,
            _ => panic!("unknown packed value for {:?}", self),
        }
    }
}

impl From<CardInfo> for C {
    fn from(value: CardInfo) -> Self {
        value.pack()
    }
}
impl From<C> for CardInfo {
    fn from(value: C) -> Self {
        value.info()
    }
}


#[cfg(test)]
mod card_tests {
    use super::*;

    #[test]
    fn all_c_have_cardinfo() {
        let all_c: Vec<C> = (EVERY_BASE..=EVERY_HIGH).map(|i| C(i)).collect();
        assert_eq!(all_c.len(), EVERY_COUNT.into());

        let info_conv: Vec<CardInfo> = all_c.clone().into_iter().map(|x| x.into()).collect();
        let c_conv: Vec<C> = info_conv.into_iter().map(|x| x.into()).collect();

        assert_eq!(all_c, c_conv);
    }

    #[test]
    fn all_cardinfo_have_c() {
        let all_info: Vec<CardInfo> = {
            let mut vec = Vec::with_capacity(74 + 4);
            vec.append(&mut (0..=21).map(|i| Card(Magic, i)).collect());
            for suit in [Wands, Stars, Swrds, Cuups] {
                vec.append(&mut (2..=13).map(|i| Card(suit, i)).collect());
            }
            vec.append(&mut vec![Tableau, Freecell, Foundation, DownFoundn, NoCard]);
            vec
        };

        let c_conv: Vec<C> = all_info.clone().into_iter().map(|x| x.into()).collect();
        let info_conv: Vec<CardInfo> = c_conv.into_iter().map(|x| x.into()).collect();

        assert_eq!(all_info, info_conv);
    }

    #[test]
    fn card_id_range_sanitycheck() {
        assert_eq!(CARDS_COUNT, MAGIC_COUNT + WANDS_COUNT + STARS_COUNT + SWRDS_COUNT + CUUPS_COUNT, "missing card suit?");
        assert_eq!(EVERY_COUNT, CARDS_COUNT + OTHER_COUNT, "missing category?");
    }

    #[test]
    fn card_properties() {
        assert_eq!(Magic.is_minor(), false);
        assert_eq!(Stars.is_minor(), true);
        assert_eq!(NoCard.is_card(), false);
        assert_eq!(Card(Wands, 10).is_card(), true);
        assert_eq!(Card(Cuups, 8).card_rank(), 8);
        assert_eq!(Card(Swrds, 4).card_suit(), Swrds);
    }
}












#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub state: BoardState,
    pub info: BoardInfo,
}

/// Core state representing the game's underlying symmetry and equivalence classes.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoardState {
    /// Pointer to where each card is located: atop another card, in the freecell, or in the foundation.
    pub cards: [C; CARDS_COUNT as usize],
}

/// Auxiliary state useful for manipulating a board quickly.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoardInfo {
    /// Pointer to the card at the top of each tableau stack.
    pub tableau: [C; 11],

    /// Pointer to the card in the freecell.
    pub freecell: C,

    /// Pointer to cards at the top of each ascending foundation.
    pub foundation: [C; 5],
    /// Pointer to card at top of descending major arcana foundation.
    pub down_foundn: C,
}


impl Hash for Board {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.state.hash(state);
    }
}
impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}
impl Eq for Board {}
impl PartialOrd for Board {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.state.partial_cmp(&other.state)
    }
}
impl Ord for Board {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.state.cmp(&other.state)
    }
}

impl Default for BoardState {
    /// Default state is the lexically smallest win.
    fn default() -> Self {
        Self { cards: [C::FOUNDATION; CARDS_COUNT as usize] }
    }
}
impl Default for BoardInfo {
    /// Default state is the lexically smallest win.
    fn default() -> Self {
        Self {
            tableau: [C::TABLEAU; 11],
            freecell: C::FREECELL,
            foundation: [C::WANDS_KING, C::STARS_KING, C::SWRDS_KING, C::CUUPS_KING, C::MAGIC_WORLD],
            down_foundn: C::NO_CARD,
        }
    }
}
impl Default for Board {
    /// Default state is the lexically smallest win.
    fn default() -> Self {
        Self { state: Default::default(), info: Default::default() }
    }
}

impl From<BoardState> for BoardInfo {
    fn from(value: BoardState) -> Self {
        let mut new = Self::default();

        let mut tab_count = 0;

        // single-pass to find all of the bottom tableau cards and top foundation cards
        for (i, card_state) in value.cards.iter().enumerate() {
            let cp = C(i as u8);
            let cs = cp.info().card_suit();
            
            match card_state.info() {
                Card(_,_) => (),
                Tableau => {new.tableau[tab_count] = cp; tab_count += 1},
                Freecell => {assert_eq!(new.freecell.info(), NoCard); new.freecell = cp},
                DownFoundn => new.down_foundn = min(new.down_foundn, cp),
                Foundation => new.foundation[cs as usize] = max(new.foundation[cs as usize], cp), 
                NoCard => panic!("missing card"),
            };
        }

        assert!(&new.tableau.windows(2).all(|w| w[0] <= w[1]), "not in sorted order");
        // would sort them here if necessary, but the cards were already traversed in that order as they were added

        // build each stack up by finding which cards are atop each previous
        for t in new.tableau.iter_mut() {
            loop {
                let mut next_cards = value.cards.iter().filter(|card_state| **card_state == *t);
                assert!(next_cards.clone().count() <= 1, "bifurcating stack");
                match next_cards.next() {
                    Some(cp) => *t = *cp,
                    None => break,
                }
            }
        }

        new
    }
}

impl From<BoardState> for Board {
    fn from(value: BoardState) -> Self {
        Board { state: value, info: value.into() }
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;

    #[test]
    fn default_boardinfo_correct() {
        let board = Board::default();
        let dbi: BoardInfo = board.info;
        let dsi: BoardInfo = board.state.into();

        assert_eq!(dbi, dsi);
    }
}



impl Board {

    pub fn apply_forced(&mut self) {
        #![allow(unused_labels)]

        let mut done = false;
        'doneloop: while ! done {
            done = true;

            let suits: &[Suit] = if self.info.freecell == C::FREECELL {
                &[Magic]
            } else {
                &[Wands, Stars, Swrds, Cuups, Magic]
            };

            let srcs: Box<dyn Iterator<Item = &mut C>> = if self.info.freecell == C::FREECELL {
                Box::new(self.info.tableau.iter_mut().chain(once(&mut self.info.freecell)))
            } else {
                Box::new(self.info.tableau.iter_mut())
            };

            'srcloop: for src in srcs {

                // Up Foundations
                'dstloop: for suit in suits {
                    let dst = &mut self.info.foundation[*suit as usize];
                    'optloop: loop { // optimistically retry the same move until it fails first before rescanning

                        // hopefully these get hoisted 
                        let srci = (*src).info();
                        let dsti = (*dst).info();
                        if ! srci.is_card() { continue 'srcloop };

                        let do_move = srci.card_suit() == *suit && srci.card_rank() == {
                            if dsti.is_card() {
                                dsti.card_rank() + 1
                            } else if *suit == Magic {
                                0
                            } else {
                                2
                            }
                        };

                        if do_move {
                            let card = *src;
                            *src = self.state.cards[card.0 as usize];
                            *dst = card;
                            done = false;
                            continue 'optloop;
                        } else {
                            break 'optloop;
                        }
                    }
                }

                // Down Foundation
                'dstloop: {
                    let suit = &Magic;
                    let dst = &mut self.info.down_foundn;
                    'optloop: loop { // optimistically retry the same move until it fails first before rescanning

                        // hopefully these get hoisted 
                        let srci = (*src).info();
                        let dsti = (*dst).info();
                        if ! srci.is_card() { continue 'srcloop };

                        let do_move = srci.card_suit() == *suit && srci.card_rank() == {
                            if dsti.is_card() {
                                dsti.card_rank() - 1
                            } else if *suit == Magic {
                                21
                            } else {
                                panic!("no upstacked minor foundation!"); 
                                // would be 13 tho
                            }
                        };

                        if do_move {
                            let card = *src;
                            *src = self.state.cards[card.0 as usize];
                            *dst = card;
                            done = false;
                            continue 'optloop;
                        } else {
                            break 'optloop;
                        }
                    }
                }
            }
        }
    }
}





#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MoveLoc {
    Tableau(u8),
    Freecell,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Move {
    pub src: MoveLoc,
    pub dst: MoveLoc,
}

