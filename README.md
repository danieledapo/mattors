# Matto

Have some fun visualizing math.

## Fractals

![redblue dragon](images/redblue-dragon.png)
![mandelbrot](images/mandelbrot.png)

## Quantization

![desert-quantized](images/desert-quantized.jpeg)
![baboon-quantized](images/baboon-quantized.jpeg)

# Examples

```
# fractals
cargo run -- dragons
cargo run -- julia
cargo run -- julia --iterations 16 mandelbrot
cargo run -- julia --iterations 128 custom -c ' -0.4+0.6i' --start " -3.0,-1.2" --end "2.0,1.2"

# quantize
cargo run -- quantize images/desert.jpeg -o images/desert-quantized.jpeg
cargo run -- quantize -d 1 images/baboon.jpeg -o images/baboon-quantized.jpeg
```
