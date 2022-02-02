# Wordle Guess Ranker

This tool generates rankings for first, second, ... nth guess for the web game wordle https://www.powerlanguage.co.uk/wordle/.

The objective is to compute the words that give maximize the expected number of words that can be eliminated from the corpus based on the information gained from the guess.

This is done more or less brute force by: for each possible guess word in the corpus...
 1. Loop over the targets that the word could actually be
   1. Compute the information that is gained by matching the guess against the word
   1. Measure how this reduces the set of possible words after factoring in the information for the guess
 1. The average reduction in corpus size is the word's score.
 
## I'm just here for the results

If you're just here for the results, luckily for you they won't change unless wordle updates their corpus. 

The best guesses in order (by round) are:
 1. "ostia"
 2. "lycee"
 3. "nairu"
 4. "abohm"
 5. "perdy"

Note that the first guess is the optimal first guess, but subsequent guesses can obviously be beaten with human intelligence by factoring in information gained in the first turn. 