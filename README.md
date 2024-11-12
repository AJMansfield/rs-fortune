
# Goals

Efficiently solve the Zachtronics Fortune's Foundation solitaire game.
Given a state of the game, produce a sequence of moves that solves that game, or prove that the game is not solvable.


# Architecture

## Board State Representation

There are 74 cards in a FF deck:
- Major Arcana with numbers 00 to 21
- Four minor suits with numbers 1 to 10, plus J, Q, K.

Note that the aces are never in play, so they can be excluded from the representation.

Each card can either be on top of one of the 69 other cards, in an empty tableau cell, in the freecell, or in the foundation.

Canonical state consists of a `state: [CardState; 70]` with indices corresponding to the 70 'real' cards in the deck.

CardState is:
- a value corresponding to the card this card is on top of
- a value indicating the freecell
- a value indicating it's the root card of a tableau stack
- a value indicating it's been scored to the foundation
- a null value?


This abstracts away permutations that are irrelevant to the solver:
- which stack is in which tableau slot
- which foundation slot corresponds to which suit

These are also not necessary for giving instructions to the user: instructions that tell a user to drag a named card are easier to follow than instructions that just name the card's slot.

Additionally, there's auxiliary state used to speed up the process of generating valid moves and maintained along with it.
- tableau top card IDs
- freecell card ID
- foundation top card IDs



To gen forced moves: (12 x 6 = 72)
- loop on tableau top cards and the freecell card
- loop on foundation cells
- generate a move if the card stacks following the foundation cell's rule

To gen player moves: (12 x 12 = 144)
- loop on tableau top cards and the freecell card
- loop on tableau and freecell
- generate a move if card stacks on cell (or cell is empty?)

To score stacking-ness: (70*)
- precompute `const DOWN: [C;70]` of which card is the down-rank from the next (Q -> K -> Tableau)
- precompute `const UP: [C;70]` of which card is the up-rank from the next (3 -> 2 -> Foundation?)
- count matches between state and DOWN (70*)
- count matches between state and UP (70*)

To score foundation: (6* = 5* + 1)
- add up foundation cell card ranks, inverting upper major foundation (6)
- layout consideration: make minor foundation and lower major foundation contiguous

To score empty-ness: (12*)
- count tab_top and freecell matches to empty value (12*)
- layout consideration: make tab_top and freecell contiguous


Possible move types:

Move from freecell to foundation (forced):
- set freecell's state to card's state ('freecell')
- set card's state to 'foundation'
- set foundation's state to card's ID

Move from tableau to foundation (forced):
- set tab_top's state to card's state ('tableau' or card)
- set card's state to 'foundation'
- set foundation's state to card's ID

Move from freecell to tableau (player):
- set freecell's state to card's state ('freecell')
- set card's state to tab_top's state
- set tab_top's state to card's ID
  
Move from tableau to freecell (player):
- set tab_top's state to card's state ('tableau' or card)
- set card's state to 'freecell'
- set freecell's state to card's ID

Move from tableau to tableau (player):
- set src tab_top's state to card's state ('tableau' or card)
- set card's state to dst tab_top's state
- set dst tab_top's state to card's ID



Stretch goal concept: Train a machine learning model to rate moves and/or game states?
How do you score the badness of an A* eval function?

