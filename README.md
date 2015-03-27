# Inefficient picture rotation algorithm

Author: Daniel Jonsson  
License: [MIT License](LICENSE)

Only works with nightly version of Rust.

## Example

A GIF compilation of the original picture, followed by 9 pictures that each is
gradually more rotated using the program's algorithm.

![Algorithm demo](assets/demo.gif)

Created the GIF image with GIMP.

## Instructions

1. Put a 512x512 pixels large 24-bit BMP image in the project's root directory
   named `img1.bmp`.
2. Execute `$ cargo run`.
3. Done. In the project's root directory you can now find a sequence of 10
   images which are gradually rotated using a recursive algorithm.

## Room for improvement

* Support more sizes than 512x512.
* Make main loop more efficient (i.e. don't redo work for each iteration).
* Compatibility with stable Rust.

