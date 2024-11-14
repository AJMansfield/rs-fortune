//! Trait Implementations for Parsing or Outputting State Objects
//! 
//! 
use std::{fmt, str::FromStr};
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
fn from_shortrank_special(s: &str) -> Option<u8> {
    let has_digits = s.contains(|c|"0123456789".contains(c));
    if has_digits { return None; }

    let s = s.to_uppercase();
    let is_ace   = s.contains("A");
    let is_ten   = s.contains("T");
    let is_jack  = s.contains("J");
    let is_queen = s.contains("Q");
    let is_king  = s.contains("K");

    let num_matches = is_ace as u32 + is_ten as u32 + is_jack as u32 + is_queen as u32 + is_king as u32;
    if num_matches != 1 { return None; }

    if is_ace   { Some( 1) } else
    if is_ten   { Some(10) } else
    if is_jack  { Some(11) } else
    if is_queen { Some(12) } else
    if is_king  { Some(13) } else 
    { panic!("logically impossible") }
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
fn from_noncard(s: &str) -> Option<C> {
    let s = s.to_uppercase();
    let is_tab   = s.contains("-");
    let is_free  = s.contains("F");
    let is_found = s.contains("V");
    let is_down  = s.contains("^");
    let is_no    = s.contains(".");

    let num_matches = is_tab as u32 + is_free as u32 + is_found as u32 + is_down as u32 + is_no as u32;
    if num_matches != 1 { return None; }

    if is_tab   { Some(C::TABLEAU) } else
    if is_free  { Some(C::FREECELL) } else
    if is_found { Some(C::FOUNDATION) } else
    if is_down  { Some(C::DOWNFOUNDN) } else
    if is_no    { Some(C::NO_CARD) } else 
    { panic!("logically impossible") }
}

impl FromStr for C {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match from_noncard(s) {
            Some(noncard) => return Ok(noncard),
            None => (),
        };

        let suit: Suit = s.parse()?;

        let get_digits_subsequence = ||{
            let i0 = s.find(|c|"0123456789".contains(c))?;
            let i1 = s.rfind(|c|"0123456789".contains(c))?;
            s[i0..=i1].parse::<u8>().ok()
        };
        let rank = from_shortrank_special(s).or_else(get_digits_subsequence).ok_or(())?;

        Ok(CardInfo::Card(suit, rank).pack())
    }
}