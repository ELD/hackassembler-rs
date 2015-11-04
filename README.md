# Hack Assembler
## Written in (mostly?) idiomatic Rust

This is the Hack Assembler from the Elements of Computing Systems book and the nand2tetris suite. This solution works
with all the provided test programs and can successfully assemble the Pong game.

Since the book (and website) requests we don't post solutions online, this is a read-only source and you are not
authorized to use this for your own purposes. This was a challenge for me to see if I could reimplement my C++ solution
in reasonably idiomatic Rust.

The most interesting observation is that the debug version is several times slower than the release version with
optimizations. It also appears to be slightly faster than my C++ solution. I'm still going to look into optimizations
(mostly &str vs String) to see if I can speed up execution even further.

This is a very interesting project to implement because it covers some basic parsing and lexing. Projects 10 and 11 are
far more interesting in those aspects as you work to produce a parser that works on a higher level, Java-like language.
