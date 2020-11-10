# The Ray Tracer Challenge done in Rust

Implementing the book [The Ray Tracer Challenge by Jamis Buck](https://www.barnesandnoble.com/w/the-ray-tracer-challenge-jamis-buck/1127035142) in the [Rust](https://www.rust-lang.org/) programming language.

There are individual projects for each chapter in the respective folder. The goal is to keep each chapter separate from one another simplifying the structure by not having a single with each chapter.

Each chapter builds on the previous chapter. Each chapter is separated because the code done in one chapter will change in a future chapter. By having a complete project for a given chapter will represent what the code looks like at the end of a chapter including all previous chapters. If you are looking for the final code that will be in Chapter 16.

This implementation was done keeping the end goal in mind and working backwards. There are are no tuples, after working through Chapter 1 what you end up with are `Point` and `Vector`. Instead of tuples there are `struct`s that represent a `Point` and `Vector`. Chapter 3 works through a 2x2 and a 3x3 matrix to get to a 4x4 matrix. There is only one `struct` for storing a matrix, `Matrix`. The `Matrix` is an array of arrays creating a 4x4 matrix. There are 2x2 matrices that use the 4x4 `Matrix` filling in only the first two rows and columns. There are no `struct`s for 3x3 or 2x2 matrices or of any other shape. This was done for simplicity and performance. This implementation also uses Rust array instead of vector and this was done for performance. Working with a fixed size array should be quicker than working with a Rust vector that has extra overhead to manage the ability to change it's size.
