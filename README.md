Just another brick in the wall.

# Context
Small fixed-length strings are very useful for building efficient systems. 16 bytes is a convenient length; power-of-two, fits nicely into CPU registers, etc.
Even if the average length is just 8, having to account for variable length strings introduces a lot of non-obvious overhead.

The strings in question: player IDs for Pokemon Showdown. Specifically, the IDs after running `toID`, which leaves ascii lowercase alpha and digits (`[a-z0-9]`).

# Problem
The range of acceptable lengths is 1 - 18. Research indicates that `18 > 16`.

# Solution
Pack the bits!

The character set, `[a-z0-9]`, only has 36 elements. That needs 5.17 bits, or 6 after rounding up. 16 bytes has 128 bits. 128 / 6 = 21. Things are looking good.

The unused bits will be zeros.

I think a good scheme would be 5 characters in 4 bytes. This wastes 2 / 32 bits, but still fits 20 characters in 16 bytes. It can also do 10 characters in 8 bytes, if a usecase calls for storing small IDs separately for space savings. Plus, it makes the lengths easier to estimate when encoding and decoding. Also, having independent chunks allows for SIMD optimizations. That probably won't be necessary, but it's fun to think about anyway :).

The Rust file contains a reference implementation. When actually putting it into practice, you may want to reassign values to facilitate other algorithms. There's also room for 28 other symbols, which could come in handy.
Also, it's very straightforward to fuse this with `toID` itself.

To recover the length of the original string, you can count leading zeros in each `u32` word subtract 2 and divide by 6, or if you know the max ID length is 18, you could store the length in the last byte. Or you could just count how many null bytes there are after unpacking. Up to you!

## Characteristics
Because this is just designed for IDs, I'm only concerned about comparing for equality. You can still sort the packed values because they're just bytes, but it won't be lexicographical. You'd have to unpack the bytes to compare them as text.

The goal wasn't necessarily to just make the IDs 20% smaller, but to pass the threshold for all IDs fitting in 16 bytes. Doing so removes the need for a second path for long strings and greatly simplifies downstream code.

