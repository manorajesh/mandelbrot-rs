# mandelbrot-rs
Rustily interact with the mandelbrot set

![Demo Picture](https://github.com/manorajesh/mandelbrot-rs/blob/master/images/demo1.png)

## Installation
```shell
git clone https://github.com/manorajesh/mandelbrot-rs.git && cd mandelbrot-rs
cargo run --release
```

## Usage
`WIP`

## Why
Idea popped into my mind during a creative lull. Seemed easy enough while also pushing me along my graphics journey.

#### Important Code
The `calculate_pixel` and `draw_mandelbrot` functions are what determine each pixel and mutate the frame buffer accordingly.
Those familiar with [this](https://en.wikipedia.org/wiki/Plotting_algorithms_for_the_Mandelbrot_set) Wikipedia page will recognize the method.
