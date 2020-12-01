# Learning Rust using The Ray Tracer Challenge book

Implementing the book [The Ray Tracer Challenge by Jamis Buck](https://www.barnesandnoble.com/w/the-ray-tracer-challenge-jamis-buck/1127035142) in the [Rust](https://www.rust-lang.org/) programming language.

## Folder Structure

There are individual projects for each chapter in thier respective folder. The goal is to keep each chapter separate from one another simplifying the structure by not having a single workspace with all chapters included. There would also be package naming conflicts.

Each chapter builds on the previous chapter. Each chapter is separated because the code done in one chapter will change in a future chapter (e.g. 9 & 10). By having a complete project for a given chapter will represent what the code looks like at the end of that chapter including all previous chapters. If you are looking for the final code that will be in Chapter 16.

## Deviations from the book

The implementation in this project was done keeping the end results in mind and working backwards. There are are no tuples, at the end of Chapter 1 there is `Point` and `Vector`. Instead of tuples this implementation use `struct`s that represent a `Point` and `Vector`. Chapter 2 introduces `Color` and is implemented as a tuple, in this project there is a `struct` to represent `Color` and no tuples.  Chapter 3 works through a 2x2 and a 3x3 matrix to get to a 4x4 matrix. There is only one `struct` for storing a matrix, `Matrix`. The `Matrix` is an array of arrays creating a 4x4 matrix. There are 2x2 matrices that use the 4x4 `Matrix` filling in only the first two rows and columns. There are no `struct`s for 3x3 or 2x2 matrices or matrices of any other shape. This was done for simplicity and performance. This implementation also uses Rust array instead of vector and this was done for performance. Working with a fixed size array should be quicker than working with a Rust vector that has extra overhead to manage the ability to change it's size.

## Profiling

Learning Rust using this book was very helpful and the next step after was profiling the Rust application. Ray Tracing is time consuming and finding ways to produce a scene in seconds rather than minutes was valuable. I used [Valgrind](https://www.valgrind.org/) profiling tools and [kcachegrind](http://kcachegrind.sourceforge.net/html/Home.html) to analyze Callgrind output. This was valuable finding expensive code that was called multiple times and in particular inverting a matrix and building a matrix for transformations. Another tool I used to determine if code written one way would compile differently than code written a different way was [Compiler Explorer](https://rust.godbolt.org/) (vector vs. array). Set it up with two sources and compare the compiled code.

## Tooling

Used the following tools:
* [VSCode](https://code.visualstudio.com/)
* [Rust Anayzer](https://rust-analyzer.github.io/) | [VSCode extention](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
* [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) | Debugger
* Windows | Linux (Profiling) | WSL2
* [Valgrind](https://www.valgrind.org/)
* [kcachegrind](http://kcachegrind.sourceforge.net/html/Home.html)
* [Compiler Explorer](https://rust.godbolt.org/)

## Feedback

I'm open to feedback. Submit an issue or PR. I'm learning how to write idiomatic Rust. I'm still learning where to use references `&` when to use `Copy` or `Clone` or any of the other borrow checking and life times. Learning how to write code without fixing broken code with `.clone()` everywhere. Learning when I should use `for` loop or when to use an `Iterator`. What are all these `Box`es let alone a `Box`ed `dyn`. Code will work but does that mean it is written the Rust way. My biggest challenge with Rust is not knowing when to use what and where.
