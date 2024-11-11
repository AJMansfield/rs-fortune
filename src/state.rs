use core::panic;
use std::{cmp::{max, min}, hash::Hash};

/// Packed single-byte representation of a FF card, or of a few other states needed for the algorithm.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct C(u8);

// Constants defining the numeric ranges allocated to each category of card.
const EVERY_BASE: u8 = 0;

    const CARDS_BASE: u8 = EVERY_BASE;
        const MAGIC_BASE: u8 = CARDS_BASE;
        const MAGIC_COUNT: u8 = 22;
        const MAGIC_HIGH: u8 = MAGIC_BASE + MAGIC_COUNT - 1;
        const WANDS_BASE: u8 = MAGIC_HIGH + 1;
        const WANDS_COUNT: u8 = 12;
        const WANDS_HIGH: u8 = WANDS_BASE + WANDS_COUNT - 1;
        const STARS_BASE: u8 = WANDS_HIGH + 1;
        const STARS_COUNT: u8 = 12;
        const STARS_HIGH: u8 = STARS_BASE + STARS_COUNT - 1;
        const SWRDS_BASE: u8 = STARS_HIGH + 1;
        const SWRDS_COUNT: u8 = 12;
        const SWRDS_HIGH: u8 = SWRDS_BASE + SWRDS_COUNT - 1;
        const CUUPS_BASE: u8 = SWRDS_HIGH + 1;
        const CUUPS_COUNT: u8 = 12;
        const CUUPS_HIGH: u8 = CUUPS_BASE + CUUPS_COUNT - 1;
    const CARDS_HIGH: u8 = CUUPS_HIGH;
    const CARDS_COUNT: u8 = CARDS_HIGH - CARDS_BASE + 1;

    const OTHER_BASE: u8 = CARDS_HIGH + 1;
        const TABLE_BYTE: u8 = OTHER_BASE;
        const FREEC_BYTE: u8 = TABLE_BYTE + 1;
        const MAJLO_BYTE: u8 = FREEC_BYTE + 1; // TODO condense all foundation values together?
        const MAJHI_BYTE: u8 = MAJLO_BYTE + 1;
        const MINOR_BYTE: u8 = MAJHI_BYTE + 1;
        const NOCRD_BYTE: u8 = MINOR_BYTE + 1;
    const OTHER_HIGH: u8 = NOCRD_BYTE;
    const OTHER_COUNT: u8 = OTHER_HIGH - OTHER_BASE + 1;

const EVERY_HIGH: u8 = OTHER_HIGH;
const EVERY_COUNT: u8 = EVERY_HIGH - EVERY_BASE + 1;

impl Default for C {
    fn default() -> Self {
        Self(NOCRD_BYTE)
    }
}


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Suit {
    Magic,
    Wands,
    Stars,
    Swrds,
    Cuups,
}
impl Suit {
    fn is_minor(&self) -> bool {
        match self {
            Magic => false,
            Wands => true,
            Stars => true,
            Swrds => true,
            Cuups => true,
        }
    }
    fn minor_idx(&self) -> usize {
        match self {
            Magic => panic!("not a minor suit"),
            Wands => 0,
            Stars => 1,
            Swrds => 2,
            Cuups => 3,
        }
    }
}

use Suit::*;
/// Unpacked information for a card value or card state.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum CardInfo {
    Card(Suit, u8),
    Tableau,
    Freecell,
    MajorLo,
    MajorHi,
    Minor,
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
            Card(Magic, rank) => { assert!(/*0 <= rank &&*/ rank <= 21); MAGIC_BASE + rank },
            Card(Wands, rank) => { assert!(2 <= rank && rank <= 13); WANDS_BASE + (rank - 2) },
            Card(Stars, rank) => { assert!(2 <= rank && rank <= 13); STARS_BASE + (rank - 2) },
            Card(Swrds, rank) => { assert!(2 <= rank && rank <= 13); SWRDS_BASE + (rank - 2) },
            Card(Cuups, rank) => { assert!(2 <= rank && rank <= 13); CUUPS_BASE + (rank - 2) },
            Tableau => TABLE_BYTE,
            Freecell => FREEC_BYTE,
            MajorLo => MAJLO_BYTE,
            MajorHi => MAJHI_BYTE,
            Minor => MINOR_BYTE,
            NoCard => NOCRD_BYTE,
        })
    }
}
impl C {
    pub fn info(self) -> CardInfo {
        match self.0 {
            MAGIC_BASE..=MAGIC_HIGH => Card(Magic, self.0 - MAGIC_BASE),
            WANDS_BASE..=WANDS_HIGH => Card(Wands, self.0 + 2 - WANDS_BASE),
            STARS_BASE..=STARS_HIGH => Card(Stars, self.0 + 2 - STARS_BASE),
            SWRDS_BASE..=SWRDS_HIGH => Card(Swrds, self.0 + 2 - SWRDS_BASE),
            CUUPS_BASE..=CUUPS_HIGH => Card(Cuups, self.0 + 2 - CUUPS_BASE),
            TABLE_BYTE => Tableau,
            FREEC_BYTE => Freecell,
            MAJLO_BYTE => MajorLo,
            MAJHI_BYTE => MajorHi,
            MINOR_BYTE => Minor,
            NOCRD_BYTE => NoCard,
            _ => panic!("unknown card byte self {}", self.0),
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
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
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
            vec.append(&mut vec![Tableau, Freecell, MajorLo, MajorHi, Minor, NoCard]);
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
struct Board {
    state: BoardState,
    info: BoardInfo,
}

/// Core state representing the game's underlying symmetry and equivalence classes.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct BoardState {
    /// Pointer to where each card is located: atop another card, in the freecell, or in the foundation.
    cards: [C; CARDS_COUNT as usize],
}

/// Auxiliary state useful for manipulating a board quickly.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct BoardInfo {
    /// Pointer to the card at the top of each tableau stack.
    tableau: [C; 11],

    /// Pointer to the card in the freecell.
    freecell: C,

    /// Pointer to cards at the top of the left and right major foundations.
    major: (C, C),

    /// Pointer to cards at the top of each minor foundation.
    minor: [C; 4],
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

impl Default for BoardInfo {
    fn default() -> Self {
        Self { tableau: Default::default(), freecell: Default::default(), major: Default::default(), minor: Default::default() }
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
                MajorLo => new.major.0 = max(new.major.0, cp),
                MajorHi => new.major.1 = min(new.major.1, cp),
                Minor => new.minor[cs.minor_idx()] = max(new.minor[cs.minor_idx()], cp), 
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

