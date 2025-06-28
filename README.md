<h1 align="center">BrainSponge v4</h1>
This is the 4th version of the BrainSponge programming language.

❗***Brainsponge is in very early stages, there are not many features***

## What is brainsponge?
BrainSponge is a superset of brainf\*\*k, an esoteric programming language designed to make your brain hurt.
So far, every valid brainf\*\*k program is a valid BrainSponge program but that will change when new instructions are added.

## Try it out
So far, there is only a windows build but I will build and test it on linux as well. It is very easy to build it from source.

To try it on windows, grab the latest version from the release page and run `bs`.

For help run `bs --help`. This will show all the commands and options.

To run a brainsponge file, run `bs --file <filename>`.

## Instructions
BrainSponge has every instruction that brainf**k has, here are the *cutting edge* new features that BrainSponge brings:
- `;` - prints the current cell as a number followed by a space (for example `23 `)
- `{` - runs anything inside `{}` if the current cell is 0

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
