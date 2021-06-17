![achim144](demos/achim144.gif)

# Game of Life

A simple implementation of ["Conway's Game of Life"] in Rust using the
[Bevy game engine]. This application on has minimal support for seeing how an
initial state evolves.

A set of initial states is given in [initstates](initstates), and number of
rows and columns in the cells grid are determined by the input state.

["Conway's Game of Life"]: https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
[Bevy game engine]: https://bevyengine.org

# Usage

Clone the project and build it with Cargo (see [the Rustbook's section on Cargo]).
The CLI help message contains more information on how to use the program.

```plaintext
game_of_life 0.1.0
It's a game of life!

USAGE:
    game_of_life [OPTIONS]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --interval <Cycle interval>    Duration (in milliseconds) to wait before moving to the next generation
    -c, --cols <Grid columns>          Number of columns in the cell grid [default: 30]
    -r, --rows <Grid rows>             Number of rows in the cell grid [default: 30]
    -s, --state <Initial state>        Path to the file contains the initial state
    -h, --height <Window's height>     Height of the game's window [default: 600.0]
    -w, --width <Window's width>       Width of the game's window [default: 600.0]
```

[the Rustbook's section on Cargo]: https://doc.rust-lang.org/cargo/

