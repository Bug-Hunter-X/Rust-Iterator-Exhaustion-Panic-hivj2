# Rust Iterator Exhaustion

This repository demonstrates a common error in Rust when working with iterators: attempting to access elements beyond the iterator's end.  The example uses a `Vec` and its iterator.  After iterating through all elements, further calls to `next()` will panic.

The solution shows how to gracefully handle the exhaustion of iterators by checking if `next()` returns `None` before attempting to access the returned value.