# Don't forget your `--release` flag when benchmarking

Saw this [very old post](https://github.com/amitkgupta/nearest_neighbour) comparing k-NN algorithm implemented in different languages, and for the love of my life I can't believe Rust runs the thing in over 1 minute.

So I took the code referred to in the post and ran it. Well it ran in over 40 seconds. I puzzled for an hour, even trying to implement the Go style code in Rust.

Then I realized I forgot the `--release` flag. I append it. It runs in not even a second. 

How I got distracted by this when I was on that old blog for Hindley-Milner is another question.