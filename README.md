
# Goals

Efficiently solve the Zachtronics Fortune's Foundation solitaire game.
Given a state of the game, produce a sequence of moves that solves that game, or prove that the game is not solvable.


# Architecture

## Board State Representation

There are 74 cards in a FF deck:
- Major Arcana with numbers 00 to 21
- Four minor suits with numbers 1 to 10, plus J, Q, K.

Represent the state of the game with `state: [u8; 74]`?
Each entry represents the corresponding card's current position in the game.

If `state[x] == y` in the range 0 thru 73, that means that card X is stacked on top of card Y.
This makes it easy for the goodness function to evaluate the existence of stacked runs of cards and award goodness for more organized boards.

If `state[x] == <freecell>`, as a single sentinel value, that means it's in the freecell.
If `state[x] == <tableau>`, as a single sentinel value, that means it's positioned on an otherwise-empty tableau seat.
If `state[x] == <foundation>`, as a single sentinel value, that means it's been scored to a foundation.

This deletes some of the symmetries of the problem from the representation -- equality only needs to test these 74 byte values.

Operations that need to be done on this representation:
- determine if a card is at the top of a stack -- use the high bit of each card?
- scan 

`state[<tab i>] == y` means that y is the top card of tableau stack i (11 addl values for the 11 tableau stacks).
`state[<freecell>] == y` means that y is the card in the freecell -- generate candidate moves from the freecell, rather than to it.
`state[]

, for  Additional 11 entries in the array for the 11 tableau stacks?

 that each card is stacked on top of, or some other values.

Cards can be "in foundation"
Card can be "in foundation"


Note that the aces are never in play either, so there's technically only 70 cards.


Represent each card as a u8 
