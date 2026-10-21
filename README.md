# Probabilistic Context-Free Grammar (PCFG) Guesser
A rust implementation of password guessing and generation based on PCFG. 

```sh
pcfg train <list_of_passwords.txt> [-o "ruleset name"]  # Train
pcfg guess [-r "ruleset name"]           # Generate new guesses
```

## Future work
In order:

* PCFG `next()` v1 and v2
* Serialize model after training
* Mask uppercase/lowercase letters
* Password strength estimation

## Terminology

- Group: 

## References
[1] M. Weir, S. Aggarwal, B. d. Medeiros and B. Glodek, "Password Cracking Using Probabilistic Context-Free Grammars," 2009 30th IEEE Symposium on Security and Privacy, Oakland, CA, USA, 2009, pp. 391-405, doi: 10.1109/SP.2009.8