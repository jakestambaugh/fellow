--2025-03-32 4:37

Moved the scan functions into a separate file

--2025-02-16 20:24

The first iteration is just a single `interpret` function that takes the whole
source code and "tokenizes" it, returning a `FellowValue` that contains the
last token in the file.

I defined a `FellowError` type, which is a standin for a much more robust
type that I have to define later

