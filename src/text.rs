//! Trait Implementations for Parsing or Outputting State Objects
//! 
//! 
use std::{collections::HashSet, fmt, iter::zip, str::FromStr};
use crate::state::*;

#[cfg(test)]
mod suit_tests {
    use std::collections::HashSet;

    use super::*;

    const SUITS: [Suit; 5] = [
        Suit::Wands,
        Suit::Stars,
        Suit::Swrds,
        Suit::Cuups,
        Suit::Magic,
    ];

    macro_rules! check_format {
        ($fstr: literal) => {
            let mut ss: HashSet<String> = Default::default();
            for suit in SUITS {
                let s = format!($fstr, suit);
                assert!(ss.insert(s.clone()), "non-unique representation: {:?} -> {:?}", suit, &s);
                let parsed: Suit = s.parse().expect("unable to parse");
                assert_eq!(parsed, suit, "misparsed representation: {:?} -> {:?} -> {:?}", suit, &s, parsed);
            }
        };
        ($fstr: literal, $check: expr, $message: expr) => {
            let mut ss: HashSet<String> = Default::default();
            for suit in SUITS {
                let s = format!($fstr, suit);
                assert!($check(suit, &s), "{}: {:?} -> {:?}", $message, suit, &s);
                assert!(ss.insert(s.clone()), "non-unique representation: {:?} -> {:?}", suit, &s);
                let parsed: Suit = s.parse().expect("unable to parse");
                assert_eq!(parsed, suit, "misparsed representation: {:?} -> {:?} -> {:?}", suit, &s, parsed);
            }
        };
    }

    #[test]
    fn upperhex() {
        check_format!("{:X}", |_suit: Suit, s: &String| !s.is_empty(), "empty representation");
    }

    #[test]
    fn lowerhex() {
        check_format!("{:x}");
    }

    #[test]
    fn display() {
        check_format!("{}");
    }

    #[test]
    fn upperexp() {
        check_format!("{:E}", |_suit: Suit, s: &String| !s.is_empty(), "empty representation");
    }

    #[test]
    fn lowerexp() {
        check_format!("{:e}");
    }
}

impl fmt::LowerHex for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Wands => "|",
            Self::Stars => "*",
            Self::Swrds => "!",
            Self::Cuups => "$",
            Self::Magic => "",
        })
    }
}
impl fmt::UpperHex for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Wands => "|",
            Self::Stars => "*",
            Self::Swrds => "!",
            Self::Cuups => "$",
            Self::Magic => "@",
        })
    }
}
impl fmt::LowerExp for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Wands => "W",
            Self::Stars => "P",
            Self::Swrds => "X",
            Self::Cuups => "C",
            Self::Magic => "",
        })
    }
}
impl fmt::UpperExp for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Wands => "W",
            Self::Stars => "P",
            Self::Swrds => "X",
            Self::Cuups => "C",
            Self::Magic => "M",
        })
    }
}
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}
impl FromStr for Suit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_uppercase();
        let is_wands = s.contains("|") || s.contains("W");
        let is_stars = s.contains("*") || s.contains("P");
        let is_swrds = s.contains("!") || s.contains("X");
        let is_cuups = s.contains("$") || s.contains("C");
        let is_magic = s.contains("@") || s.contains("M");

        let num_matches: u32 = is_wands as u32 + is_stars as u32 + is_swrds as u32 + is_cuups as u32 + is_magic as u32;
        match num_matches {
            0 => Ok(Self::Magic),
            1 => {
                if is_wands { Ok(Self::Wands) } else
                if is_stars { Ok(Self::Stars) } else
                if is_swrds { Ok(Self::Swrds) } else
                if is_cuups { Ok(Self::Cuups) } else
                if is_magic { Ok(Self::Magic) } else 
                { panic!("should be impossible") }
            },
            _ => Err(()),
        }
    }
}






#[cfg(test)]
mod card_tests {
    use std::collections::HashSet;

    use super::*;

    macro_rules! check_format {
        ($fstr: literal) => {
            let all_c: Vec<C> = (EVERY_BASE..=EVERY_HIGH).map(|i| C(i)).collect();
            let mut ss: HashSet<String> = Default::default();
            for card in all_c {
                let s = format!($fstr, card);
                assert!(ss.insert(s.clone()), "non-unique representation: {:?}, {:?}", card, &s);
                let parsed: C = s.parse().expect("unable to parse");
                assert_eq!(parsed, card, "misparsed representation: {:?} -> {:?} -> {:?}", card, &s, parsed);
            }
        };
        ($fstr: literal, $check: expr, $message: expr) => {
            let all_c: Vec<C> = (EVERY_BASE..=EVERY_HIGH).map(|i| C(i)).collect();
            let mut ss: HashSet<String> = Default::default();
            for card in all_c {
                let s = format!($fstr, card);
                assert!($check(card, &s), "{}: {:?} -> {:?}", $message, card, &s);
                assert!(ss.insert(s.clone()), "non-unique representation: {:?}, {:?}", card, &s);
                let parsed: C = s.parse().expect("unable to parse");
                assert_eq!(parsed, card, "misparsed representation: {:?} -> {:?} -> {:?}", card, &s, parsed);
            }
        };
    }

    #[test]
    fn upperhex() {
        check_format!("{:X}");
    }

    #[test]
    fn lowerhex() {
        check_format!("{:x}", |_card: C, s: &String| s.len()==2, "not length 2");
    }

    #[test]
    fn display() {
        check_format!("{}");
    }

    #[test]
    fn upperexp() {
        check_format!("{:E}");
    }

    #[test]
    fn lowerexp() {
        check_format!("{:e}", |_card: C, s: &String| s.len()==2, "not length 2");
    }
}


impl fmt::LowerHex for C {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let info = self.info();
        if info.is_card() {
            if info.card_suit().is_minor() {
                write!(f, "{:x}", info.card_suit())?;
                fmt_shortrank(f, info.card_rank())
            } else {
                write!(f, "{:<02}", info.card_rank())
            }
        } else {
            fmt_noncard(f, *self)
        }

    }
}
impl fmt::UpperHex for C {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let info = self.info();
        if info.is_card() {
            write!(f, "{:X}{}", info.card_suit(), info.card_rank())
        } else {
            fmt_noncard(f, *self)
        }
    }
}
impl fmt::LowerExp for C {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let info = self.info();
        if info.is_card() {
            if info.card_suit().is_minor() {
                write!(f, "{:e}", info.card_suit())?;
                fmt_shortrank(f, info.card_rank())
            } else {
                write!(f, "{:<02}", info.card_rank())
            }
        } else {
            fmt_noncard(f, *self)
        }
    }
}
impl fmt::UpperExp for C {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let info = self.info();
        if info.is_card() {
            write!(f, "{:E}{}", info.card_suit(), info.card_rank())
        } else {
            fmt_noncard(f, *self)
        }
    }
}
impl fmt::Display for C {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}
fn fmt_shortrank(f: &mut fmt::Formatter<'_>, rank: u8) -> fmt::Result {
    match rank {
        0..=9 => write!(f, "{}", rank),
        10 => write!(f, "T"),
        11 => write!(f, "J"),
        12 => write!(f, "Q"),
        13 => write!(f, "K"),
        _ => panic!("no shortrank repr for {}", rank)
    }
}
fn from_shortrank_special(s: &str) -> Result<u8,()> {
    let s = s.to_uppercase();
    let is_ace   = s.contains("A");
    let is_ten   = s.contains("T");
    let is_jack  = s.contains("J");
    let is_queen = s.contains("Q");
    let is_king  = s.contains("K");

    let num_matches = is_ace as u32 + is_ten as u32 + is_jack as u32 + is_queen as u32 + is_king as u32;
    match num_matches {
        1 => {
            if is_ace   { Ok( 1) } else
            if is_ten   { Ok(10) } else
            if is_jack  { Ok(11) } else
            if is_queen { Ok(12) } else
            if is_king  { Ok(13) } else 
            { panic!("should be impossible") }
        },
        _ => Err(()),
    }
}
fn from_rank(s: &str) -> Result<u8,()> {
    let i0 = s.find(|c|"0123456789".contains(c)).ok_or(())?;
    let i1 = s.rfind(|c|"0123456789".contains(c)).ok_or(())?;
    s[i0..=i1].parse().or(Err(()))
}
fn fmt_noncard(f: &mut fmt::Formatter<'_>, noncard: C) -> fmt::Result {
    match noncard {
        C::TABLEAU => write!(f, "--"),
        C::FREECELL => write!(f, "FF"),
        C::FOUNDATION => write!(f, "vv"),
        C::DOWNFOUNDN => write!(f, "^^"),
        C::NO_CARD => write!(f, ".."),
        _ => panic!("not a noncard"),
    }
}
fn from_noncard(s: &str) -> Result<C,()> {
    let s = s.to_uppercase();
    let is_tab   = s.contains("-");
    let is_free  = s.contains("F");
    let is_found = s.contains("V");
    let is_down  = s.contains("^");
    let is_no    = s.contains(".");

    let num_matches = is_tab as u32 + is_free as u32 + is_found as u32 + is_down as u32 + is_no as u32;
    match num_matches {
        1 => {
            if is_tab   { Ok(C::TABLEAU) } else
            if is_free  { Ok(C::FREECELL) } else
            if is_found { Ok(C::FOUNDATION) } else
            if is_down  { Ok(C::DOWNFOUNDN) } else
            if is_no    { Ok(C::NO_CARD) } else 
            { panic!("should be impossible") }
        },
        _ => Err(()),
    }
}

impl FromStr for C {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        from_noncard(s).or_else(|_|{
            let suit: Suit = s.parse()?;
            let rank = from_shortrank_special(s).or_else(|_|from_rank(s))?;
            Ok(CardInfo::Card(suit, rank).pack())
        })
    }
}


#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct BoardLayout {
    tableau: [Vec<C>; 11],
    info: BoardInfo,
}

impl Default for BoardLayout {
    fn default() -> Self {
        Self { tableau: Default::default(), info: Default::default() }
    }
}

impl From<BoardState> for BoardLayout {
    fn from(value: BoardState) -> Self {
        let mut done = false;
        let mut result = BoardLayout { tableau: Default::default(), info: value.into() };
        while !done {
            done = true;

            for (i, card_state) in value.cards.iter().enumerate() {
                let card = C(i as u8);
                for tab in result.tableau.iter_mut() {
                    let top = *tab.last().unwrap_or(&C::TABLEAU);
                    if *card_state == top {
                        tab.push(card);
                        done = false;
                    }
                }
            }
        }

        for (tab, tab_top) in zip(result.tableau.iter(), result.info.tableau.iter()) {
            let tab_end = tab.last().unwrap_or(&C::TABLEAU);
            assert_eq!(tab_end, tab_top);
        }

        result
    }
}

impl Into<Board> for BoardLayout {
    fn into(self) -> Board {
        let mut result = Board { state: Default::default(), info: self.info };

        // mark each card in the tableau with the card below it, or tableau for bottom-cards
        for tab in self.tableau.iter() {
            result.state.cards[tab[0].0 as usize] = C::TABLEAU;
            for w in tab.windows(2) {
                result.state.cards[w[1].0 as usize] = w[0];
            }
        }

        // mark the card in the freecell
        result.state.cards[self.info.freecell.0 as usize] = C::FREECELL;

        // mark all minor arcana cards from 2 to top
        for suit in [Suit::Wands, Suit::Stars, Suit::Swrds, Suit::Cuups] {
            let top = self.info.foundation[suit as usize];
            let ti = top.info();
            if ti.is_card() {
                assert_eq!(ti.card_suit(), suit);
                for rank in 2..=ti.card_rank() {
                    let ci = CardInfo::Card(ti.card_suit(), rank);
                    result.state.cards[ci.pack().0 as usize] = C::FOUNDATION;
                }
            }
        }
        
        // mark all major arcana cards from Fool to top
        {
            let suit = Suit::Magic;
            let top = self.info.foundation[suit as usize];
            let ti = top.info();
            if ti.is_card() {
                assert_eq!(ti.card_suit(), suit);
                for rank in 0..=ti.card_rank() {
                    let ci = CardInfo::Card(ti.card_suit(), rank);
                    result.state.cards[ci.pack().0 as usize] = C::FOUNDATION;
                }
            }
        }

        // mark all major arcana cards from top to World
        {
            let suit = Suit::Magic;
            let top: C = self.info.down_foundn;
            let ti = top.info();
            if ti.is_card() {
                assert_eq!(ti.card_suit(), suit);
                for rank in ti.card_rank()..=21 {
                    let ci = CardInfo::Card(ti.card_suit(), rank);
                    result.state.cards[ci.pack().0 as usize] = C::DOWNFOUNDN;
                }
            }
        }

        result
    }
}

// theoretically, the ENTIRE board state could be constructed from _just_ the tableau layout
// any card that's not present in the tableau must be either stacked in the foundation or sitting in the freecell
// and auto-stack means if a card in the freecell _could_ be stacked it would
// Minor-foundation locking doesn't preclude this:
// - a minor arcana card can't become scorable while in the freecell because the next-lower card can't be scored
// - it can't be scorable when being moved to the freecell because if it's free to move it would be scored immediately instead

impl From<[Vec<C>; 11]> for BoardLayout {
    fn from(value: [Vec<C>; 11]) -> Self {
        let all_cards: HashSet<C> = {
            let mut result: HashSet<C> = Default::default();
            for i in CARDS_BASE..=CARDS_HIGH {
                result.insert(C(i));
            }
            result 
        };

        let tab_cards: HashSet<C> = {
            let mut result: HashSet<C> = Default::default();
            for tab in &value {
                for card in tab {
                    assert!(result.insert(*card));
                }
            }
            result
        };

        let mut fdn_cards: HashSet<C> = all_cards.difference(&tab_cards).map(|c|*c).collect();


        let mut result = BoardLayout {
            tableau: value,
            info: BoardInfo { 
                tableau: [C::TABLEAU; 11],
                freecell: C::FREECELL,
                foundation: [C::NO_CARD; 5],
                down_foundn: C::NO_CARD,
            }
        };

        for (tab, tab_top) in zip(result.tableau.iter(), result.info.tableau.iter_mut()) {
            *tab_top = *tab.last().unwrap_or(&C::TABLEAU);
        }

        for suit in [Suit::Wands, Suit::Stars, Suit::Swrds, Suit::Cuups] {
            'rankloop: for rank in 2..=13 {
                let info = CardInfo::Card(suit, rank);
                let card = info.pack();
                if fdn_cards.remove(&card) {
                    result.info.foundation[suit as usize] = card;
                } else {
                    break 'rankloop;
                }
            }
        }

        {
            let suit = Suit::Magic;
            'rankloop: for rank in 0..=21 {
                let info = CardInfo::Card(suit, rank);
                let card = info.pack();
                if fdn_cards.remove(&card) {
                    result.info.foundation[suit as usize] = card;
                } else {
                    break 'rankloop;
                }
            }
        }

        {
            let suit = Suit::Magic;
            'rankloop: for rank in (0..=21).rev() {
                let info = CardInfo::Card(suit, rank);
                let card = info.pack();
                if fdn_cards.remove(&card) {
                    result.info.down_foundn = card;
                } else {
                    break 'rankloop;
                }
            }
        }

        assert!(fdn_cards.len() <= 1, "{} cards left: {:?}", fdn_cards.len(), fdn_cards);

        result.info.freecell = fdn_cards.drain().next().unwrap_or(C::FREECELL);

        result
    }
}

use std::convert::TryInto;

fn vec_to_arr<T, const N: usize>(v: Vec<T>) -> Result<[T; N],()> {
    v.try_into().or(Err(()))
}

impl FromStr for BoardLayout {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut is_err = false;

        let tab_vec = s.lines().map(|line|{
            line.split_whitespace().map(|cardstr|{
                cardstr.parse().unwrap_or_else(|_| {is_err = true; C::NO_CARD})
            }).collect::<Vec<C>>()
        }).collect::<Vec<Vec<C>>>();

        if is_err { return Err(()) }

        let tab: [Vec<C>; 11]  = vec_to_arr::<Vec<C>, 11>(tab_vec)?;

        return Ok(tab.into())
    }
}


#[cfg(test)]
mod board_tests {

    use super::*;

    #[test]
    fn parse_fresh() {
        let s = "\
05 07 2$ 8| T! 3* 8$
18 20 9* 03 K| 2! 01
16 09 9| 4* 3! 15 11
K$ 6* 5$ T* 5* 4! 5|
Q* 6$ J! 14 5! 00 8!

6| Q| 7* 4$ 2| 7! 12
06 02 K* 3| 04 3$ 7|
2* 19 13 T| T$ 4| 10
J| 21 7$ Q$ J* 6! 9!
9$ 17 K! J$ Q! 8* 08";

        let fdn = [C::NO_CARD; 5];
        let dfdn =  C::NO_CARD;
        let frec =  C::FREECELL;

        let bl: BoardLayout = s.parse().expect("parse error");
        assert_eq!(bl.info.foundation, fdn);
        assert_eq!(bl.info.down_foundn, dfdn);
        assert_eq!(bl.info.freecell, frec);
    }

    #[test]
    fn parse_handful_scored() {
        let s = "\
05 07 8| T! 3* 8$
18 9* 03 K|
16 09 9| 4* 3! 15 11
K$ 6* 5$ T* 5* 4! 5|
Q* 6$ J! 14 5! 8!

6| Q| 7* 4$ 2| 7! 12
06 02 K* 3| 04 7|
13 T| T$ 4| 10
J| 7$ Q$ J* 6! 9!
9$ 17 K! J$ Q! 8* 08";

        let fdn = [
            C::NO_CARD,
            "2*".parse().unwrap(),
            "2!".parse().unwrap(),
            "3$".parse().unwrap(),
            "01".parse().unwrap(),
        ];
        let dfdn =  "19".parse().unwrap();
        let frec =  C::FREECELL;

        let bl: BoardLayout = s.parse().expect("parse error");
        assert_eq!(bl.info.foundation, fdn);
        assert_eq!(bl.info.down_foundn, dfdn);
        assert_eq!(bl.info.freecell, frec);
    }

    #[test]
    fn parse_freecell_filled() {
        let s = "\
05 07 2$ 8| T! 3* 8$
18 20 9* 03 K| 2! 01
16 09 4* 3! 15 11
K$ 6* 5$ T* 5* 4! 5|
Q* 6$ J! 14 5! 00 8!

6| Q| 7* 4$ 2| 7! 12
06 02 K* 3| 04 3$ 7|
2* 19 13 T| T$ 4| 10
J| 21 7$ Q$ J* 6! 9!
9$ 17 K! J$ Q! 8* 08";

        let fdn = [C::NO_CARD;5];
        let dfdn =  C::NO_CARD;
        let frec = "9|".parse().unwrap();

        let bl: BoardLayout = s.parse().expect("parse error");
        assert_eq!(bl.info.foundation, fdn);
        assert_eq!(bl.info.down_foundn, dfdn);
        assert_eq!(bl.info.freecell, frec);
    }
    #[test]
    fn parse_empty() {
        let s = "\










";

        let fdn = [
            "K|".parse().unwrap(),
            "K*".parse().unwrap(),
            "K!".parse().unwrap(),
            "K$".parse().unwrap(),
            "21".parse().unwrap(),
        ];
        let dfdn = C::NO_CARD;
        let frec = C::NO_CARD;

        let bl: BoardLayout = s.parse().expect("parse error");
        assert_eq!(bl.info.foundation, fdn);
        assert_eq!(bl.info.down_foundn, dfdn);
        assert_eq!(bl.info.freecell, frec);
    }

}