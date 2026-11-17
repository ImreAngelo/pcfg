# Probabilistic Context-Free Grammar (PCFG) Guesser
A rust implementation of password guessing and generation based on PCFG. 

![guesser](../.github/docs/pcfg.gif)

```sh
pcfg train <list_of_passwords.txt> [-r "ruleset name"]  # Train
pcfg run [-r "ruleset name"]             # Generate new guesses
```

## Installation

### Kali Linux / Ubuntu 24.04
```sh
curl -fsSL https://imreangelo.github.io/pcfg/setup.sh | sudo sh

sudo apt update
sudo apt install pcfg
```

### Cargo
The package contains a library crate for training and running trained models, and a binary crate that can be installed and run via cargo:
```sh
cargo install pcfg
cargo pcfg train <file> -r ruleset
cargo pcfg run <ruleset>
```


## Benchmarks


## Future work
In order:

* Optimize PCFG v2
* PCFG `next()` v1
* Mask uppercase/lowercase letters
* Built-in OMEN nodes
* Password strength estimation
* Approximate probability order

## References
- [1] M. Weir, S. Aggarwal, B. d. Medeiros and B. Glodek, "Password Cracking Using Probabilistic Context-Free Grammars," 2009 30th IEEE Symposium on Security and Privacy, Oakland, CA, USA, 2009, pp. 391-405, doi: 10.1109/SP.2009.8
- [2] https://github.com/lakiw/pcfg_cracker