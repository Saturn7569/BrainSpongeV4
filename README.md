<h1 align="center">BrainSponge v4</h1>
This is the 4th version of the BrainSponge programming language.

## What is brainsponge?
BrainSponge is a superset of brainf\*\*k, an esoteric programming language designed to make your brain hurt.
It has the same features as brainf\*\*k + some more QOL.

## Try it out
So far, there is only a windows build but I will build and test it on linux as well. It is very easy to build it from source.

To try it on windows, grab the latest version from the release page and run `bs`.

For help run `bs --help`. This will show all the commands and options.

To run a brainsponge file, run `bs --file <filename>`.

## Instructions
BrainSponge has every instruction that brainf**k has, here are all the *cutting edge* features that BrainSponge brings:
- `+` - Increments the current cell by one
- `-` - Decrements the current cell by one
- `<` - Moves the pointer left, wraps around
- `>` - Moves the pointer right and also wraps around
- `.` - Prints the current cell as character
- `,` - Reads one byte from stdin and puts it in the current cell
- `;` - Prints the current cell as a number followed by a space (for example `23 `)
- `{` - Runs anything inside `{}` if the current cell is 0
- `[` - Starts a loop that loops anything in `[]` until the cell the loop ends on is 0

You can also create comments with `#`
```bs
+++; # This is a comment
```

## Building from source
If you are on linux or want to build from source to have the latest version, first make sure you have all the dependencies installed. You only need rust (cargo) to build it so on ubuntu you would do:
```sh
sudo apt install cargo
```

Now clone the git repo:
```sh
git clone https://github.com/Saturn7569/BrainSpongeV4.git
```

Finally cd into the directory and run:
```sh
cargo build --release
```

This will create a file called `bs` in `target/release`

If you want to remove the dependencies after you build it run:
```sh
sudo apt remove cargo -y && sudo apt autoremove -y
```

This will remove rust from your computer so it doesn't eat up space if you don't use it.
