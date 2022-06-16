# tetris-rust

This is a Tetris Clone, written in Rust using [the ggez library](https://github.com/ggez/ggez).  

## Table of Contents

- [Controls](#controls)
- [Screenshots](#screenshots)
- [Downloads](#downloads)
- [Settings](#settings)
- [Mechanics](#mechanics)
  - [Score](#score)
  - [Falling speed](#falling-speed)
  - [Piece rotations](#piece-rotations)
- [License](#license)

## Controls

- <kbd>←</kbd> / <kbd>→</kbd> to move pieces.
- <kbd>↓</kbd> / <kbd>↑</kbd> to "soft" / "hard" drop pieces.
- <kbd>Space</kbd> / <kbd>Alt</kbd> to rotate pieces clockwise / counter-clockwise.
- <kbd>Numpad 0</kbd> to hold pieces.
- <kbd>Escape</kbd> to pause/unpause the game.
- <kbd>Enter</kbd> to restart the game after game over.

Note that these controls are not traditional, usually `Space` and `Up` are switched around, also not everyone has a Numpad on their Keyboard. These are just the controls that I like to use.  

## Screenshots

![](https://i.imgur.com/kwl5utH.png) | ![](https://i.imgur.com/Xo2u0d8.png) | ![](https://i.imgur.com/tj4Kk73.png)
|:---:|:---:|:----:|

## Downloads

You can download the pre-compiled Windows/Linux Versions [here, at the releases tab](https://github.com/atomflunder/tetris-rust/releases).  
Make sure that the `./resources` folder and the `config.json` file are present in the same directory as the executeable.  

Or, clone this repository and run:

```
cargo build --release   # to build
cargo run --release     # to run
```

([Rust needs to be installed.](https://www.rust-lang.org/learn/get-started))

## Settings

You can modify some settings in the [`config.json`](./config.json) file:

| Setting | Type | Default Value | Explanation | 
|---|---|---|---|
| colored_board | bool | true | If set to true, the already dropped pieces will be colored with their usual color. If set to false they will just appear white. Note that this does not apply to the current falling piece or the pieces on the sidebar.
| modern_piece_rng | bool | true | [Modern piece RNG](https://tetris.wiki/Random_Generator) means that you will be guaranteed to see each piece in equal probabilities. The game generates "bags" containing each of the 7 pieces and shuffles it randomly. If set to false, the game generates pieces completely randomly, like in the classic games.
| bag_amount | int(u8) | 5 | How many "bags" to generate for each cycle. The game generates a bag with X times each of the 7 pieces and shuffles it randomly. This means that you cannot go more than X * 12 pieces in a row without seeing a specific piece and you cannot see a piece more than X * 2 times in a row.
| first_piece_no_overhang | bool | true | If set to true, this will prevent spawning pieces that can generate an ["overhang"](https://tetris.wiki/Glossary#O) as the very first piece. These pieces being the S, Z and O pieces. Only has an effect if modern piece RNG is enabled.
| holding_enabled | bool | true | If you want to enable the [ability to hold pieces](https://tetris.wiki/Hold_piece).

An example of how the default `config.json` file looks:

```json
{
    "colored_board": true,
    "modern_piece_rng": true,
    "bag_amount": 5,
    "first_piece_no_overhang": true,
    "holding_enabled": true
}
```

## Mechanics

### Score

We use a version of the original [Nintendo Scoring System](https://tetris.wiki/Scoring#Original_Nintendo_scoring_system), but we start at Level 1. This means that:

Level | Singles | Doubles | Triples | Tetrises
|---|---|---|---|---|
1 | 40 | 100 | 300 | 1200
2 | 80 | 200 | 600 | 2400
... 
10 | 400 | 1000 | 3000 | 12000
... 
20 | 800 | 2000 | 6000 | 24000
...

There are no points gained for T-Spins, Back-to-Backs and the like.

### Falling speed

The falling piece speed is increased every 5 Levels, starting at 1 Block per Second, increasing by 1 each time. This means that:

Level | Blocks / Second
|---|---|
1 | 1
2 | 1
... 
5 | 1
6 | 2
7 | 2
...
10 | 2
11 | 3
...
20 | 4
21 | 5
...

### Piece rotations

This game uses its own piece rotation system. The pieces rotate clockwise when read from left-to-right.

| ![](https://i.imgur.com/eTjpg2J.png) |
|:---:|
| I-Piece |
| ![](https://i.imgur.com/CIppum0.png) |
| J-Piece |
| ![](https://i.imgur.com/TBRI7VH.png) |
| L-Piece |
| ![](https://i.imgur.com/FIvGC2F.png) |
| O-Piece |
| ![](https://i.imgur.com/NyRYCuk.png) |
| S-Piece |
| ![](https://i.imgur.com/erG0vX7.png) |
| Z-Piece |
| ![](https://i.imgur.com/5kHYW1g.png) |
| T-Piece |

## License

This project is licensed under the [MIT License](./LICENSE).  
We are using the [Press Start 2P Font](https://fonts.google.com/specimen/Press+Start+2P) from Google [licensed under the Open Font License](https://fonts.google.com/specimen/Press+Start+2P#license).  
