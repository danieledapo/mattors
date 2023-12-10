# Matto [![Build Status](https://travis-ci.org/danieledapo/mattors.svg?branch=master)](https://travis-ci.org/danieledapo/mattors)

Have some fun visualizing math.

## Fractals

![redblue dragon](images/redblue-dragon.png)
![mandelbrot](images/mandelbrot.png)
![horns](images/red-horns.png)
![fractal tree](images/fractree.png)
![sierpinski](images/sierpinski.png)

## Quantization

![desert-quantized](images/desert-quantized.jpeg)
![baboon-quantized](images/baboon-quantized.jpeg)

## Primirs

inspired by [primitive](https://github.com/fogleman/primitive).

![rb-primitized](images/rb-primitized.png)
![tiffanys-primitized](images/tiffanys-primitized.png)

## Voronoi

![voronoi](images/voronoi.png)
![voronoi-gradient](images/voronoi-gradient.png)

## Delaunay

![delaunay](images/delaunay.png)

## Patchwork

inspired by [this article](https://mattdesl.svbtle.com/pen-plotter-2).

![patchwork](images/patchwork.png)
![patchwork-filled1](images/patchwork-filled1.png)
![patchwork-filled2](images/patchwork-filled2.png)

## Stippling

![stippling-gradient](images/stippling-gradient.png)
![stippling-rects](images/stippling-rects.png)

## Mondrian

inspired by `Composition in Red, Blue and Yellow` by Mondrian.

![mondrian1](images/mondrian1.png)
![mondrian2](images/mondrian2.png)

## Tangled webs

inspired by https://www.inconvergent.net/2019/a-tangle-of-webs/.

![tangled-webs](images/tangled-web.png)

# Examples

```
# fractals
cargo run -- dragons
cargo run -- horns
cargo run -- julia
cargo run -- julia --iterations 16 mandelbrot
cargo run -- julia --iterations 128 custom -c ' -0.4+0.6i' --start " -3.0,-1.2" --end "2.0,1.2"
cargo run -- sierpinski --fancy
cargo run -- fractal-tree

# quantize
cargo run -- quantize images/desert.jpeg -o images/desert-quantized.jpeg
cargo run -- quantize -d 1 images/baboon.jpeg -o images/baboon-quantized.jpeg

# primirs
cargo run --release -- primirs --shapes 200 --mutations 150 -o images/rb-primitized.png --dx 8 --dy 8 images/rb.png
cargo run --release -- primirs --shapes 200 --mutations 100 --scale-down 2 --dx 16 --dy 16 -o primitized.png images/tiffanys.jpg

# voronoi
cargo run -- voronoi --points 150 -o images/voronoi.png
cargo run -- voronoi --gradient-background --points 150 -o images/voronoi-gradient.png

# delaunay
cargo run -- delaunay --grid-size 50 -o images/delaunay.png

# patchwork
cargo run -- patchwork
cargo run --release -- patchwork -f --points 4000 --width 600 --height 600 --clusters 10

# stippling
cargo run -- stippling gradient -p 1000 -k 5
cargo run -- stippling rects --iterations 1500

# Mondrian
cargo run -- mondrian -w 800 -h 800

# tangled webs
cargo run -- tangled-web

# misc
cargo run -- runes -p 3 -c 26
cargo run -- dither -c 2 images/desert.jpeg
cargo run -- dither -c 5 --rgb images/desert.jpeg
```
