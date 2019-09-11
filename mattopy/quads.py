#!/usr/bin/env python3

import argparse
import math
import random

import noise


THEMES = {
    "flame": {
        "background": "crimson",
        "palette": [
            "rgb(255,0,0)",
            "rgb(255,48,0)",
            "rgb(255,71,0)",
            "rgb(255,88,0)",
            "rgb(255,103,0)",
            "rgb(255,117,0)",
            "rgb(255,130,0)",
            "rgb(255,142,0)",
            "rgb(255,154,0)",
            "rgb(255,165,0)",
        ],
        "stroke": "none",
    },
    "ocean": {
        "background": "rgb(0, 0, 101)",
        "palette": [
            "rgb(0, 0, 128)",
            "rgb(58, 56, 155)",
            "rgb(89, 104, 181)",
            "rgb(114, 155, 208)",
            "rgb(135, 207, 235)",
        ],
        "stroke": "none",
    },
    "acid": {
        "background": "rgb(0, 128, 0)",
        "palette": [
            "rgb(60, 158, 11)",
            "rgb(98, 190, 23)",
            "rgb(135, 222, 35)",
            "rgb(172, 255, 47)",
        ],
        "stroke": "none",
    },
    "bw": {"background": "white", "palette": ["white"], "stroke": "black"},
}


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("--width", default=1920, type=int)
    parser.add_argument("--height", default=1080, type=int)
    parser.add_argument("-l", "--quad-size", default=50, type=int)
    parser.add_argument("-p", "--particles", default=50, type=int)
    parser.add_argument("-s", "--steps", default=50, type=int)
    parser.add_argument("-t", "--theme", choices=list(THEMES.keys()), default="flame")

    return parser.parse_args()


def main():
    args = parse_args()
    padding = 20
    seed = random.random()

    quads = []
    for _ in range(args.particles):
        x, y = (
            random.randrange(padding, args.width - padding),
            random.randrange(padding, args.height - padding),
        )

        for _ in range(args.steps):
            z = 0.5 + 0.5 * noise.snoise2(x / args.width, y / args.height, base=seed)

            if random.random() > 0.6:
                quads.append(((x, y), z))

            x += math.cos(z * 2 * math.pi) * 10
            y += math.sin(z * 2 * math.pi) * 10

    theme = THEMES[args.theme]
    dump_svg(
        "quads.svg",
        (args.width, args.height),
        quads,
        padding=padding,
        quadl=args.quad_size,
        background=theme["background"],
        palette=theme["palette"],
        stroke=theme["stroke"],
    )


def dump_svg(
    filename,
    dimensions,
    quads,
    background,
    palette,
    padding=20,
    quadl=50,
    stroke="none",
):
    width, height = dimensions

    with open(filename, "wt") as fp:
        minl = min(quads, key=lambda q: q[1])[1]
        maxl = max(quads, key=lambda q: q[1])[1]

        def l_of(z):
            if maxl == minl:
                return quadl

            return ((z - minl) / (maxl - minl)) * quadl

        quads = (
            ((cx - l_of(z) / 2, cy - l_of(z) / 2), l_of(z)) for ((cx, cy), z) in quads
        )

        print(
            """<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox="0 0 {width} {height}">
<rect width="{width}" height="{height}" stroke="none" fill="{background}" />
{quads}
</svg>""".format(
                width=width,
                height=height,
                background=background,
                quads="\n".join(
                    '<rect x="{}" y="{}" width="{}" height="{}" fill="{color}" stroke="{stroke}" />'.format(
                        x, y, l, l, color=random.choice(palette), stroke=stroke
                    )
                    for ((x, y), l) in quads
                    if x > padding
                    and x + l < width - padding
                    and y > padding
                    and y + l < height - padding
                ),
            ),
            file=fp,
        )


if __name__ == "__main__":
    main()
