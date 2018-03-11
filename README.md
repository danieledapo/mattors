# Matto

Have some fun visualizing math.

![redblue dragon](images/redblue-dragon.png)
![mandelbrot](images/mandelbrot.png)

# Examples

```
cargo run -- dragons
cargo run -- julia
cargo run -- julia --iterations 16 mandelbrot
cargo run -- julia --iterations 128 custom -c ' -0.4+0.6i' --start " -3.0,-1.2" --end "2.0,1.2"
```
