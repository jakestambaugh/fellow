--2025-07-21 21:11

Get the tokenizer working in the CLI.

--2025-04-06 22:59

The scanner now has some tests around the things tokens that it can recognize.
I'm trying to capture the rest of the tokens now.

--2025-03-31 4:37

Moved the scan functions into a separate file

--2025-02-16 20:24

The first iteration is just a single `interpret` function that takes the whole
source code and "tokenizes" it, returning a `FellowValue` that contains the
last token in the file.

I defined a `FellowError` type, which is a standin for a much more robust
type that I have to define later

