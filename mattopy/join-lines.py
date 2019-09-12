#!/usr/bin/env python3

import math
import random

import noise


BACKGROUND = "crimson"
PALETTE = [
    "rgba(255,   0, 0, 1)",
    "rgba(255,  48, 0, 1)",
    "rgba(255,  71, 0, 1)",
    "rgba(255,  88, 0, 1)",
    "rgba(255, 103, 0, 1)",
    "rgba(255, 117, 0, 1)",
    "rgba(255, 130, 0, 1)",
    "rgba(255, 142, 0, 1)",
    "rgba(255, 154, 0, 1)",
    "rgba(255, 165, 0, 1)",
]


def main():
    width = 1920 / 4
    height = 1080 / 4
    padding = 20
    n = 50
    seed = random.random()

    def rand_pt():
        return (
            random.randrange(padding, width - padding),
            random.randrange(padding, height - padding),
        )

    segs = []
    pivot = rand_pt()

    for i in range(len(PALETTE)):
        segs.append(([], PALETTE[i]))
        for i in range(n):
            t = i / n
            p0 = (
                padding + t * (width - padding * 2),
                height / 2 + noise.snoise2(t, seed) * (height / 2 - padding),
            )
            p1 = (
                padding + (t + 1 / n) * (width - padding * 2),
                height / 2 + noise.snoise2(t + 1 / n, seed) * (height / 2 - padding),
            )
            segs[-1][0].append(p0)
            # segs.append(([p0, pivot, p1, p0], "none" or random.choice(PALETTE)))

        segs[-1][0].append(pivot)
        segs[-1][0].append(segs[-1][0][0])

        seed += 0.05

    dump_svg(
        "join-lines.svg",
        (width, height),
        segs,
        background="white" or BACKGROUND,
        stroke="black",
    )


def dist2(a, b):
    return (b[0] - a[0]) ** 2 + (b[1] - a[1]) ** 2


def dump_svg(filename, dimensions, lines, background, stroke="none"):
    width, height = dimensions

    with open(filename, "wt") as fp:
        print(
            """<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox="0 0 {width} {height}">
<rect width="{width}" height="{height}" stroke="none" fill="{background}" />
{lines}
</svg>""".format(
                width=width,
                height=height,
                background=background,
                lines="\n".join(
                    '<polyline points="{points}" stroke="{stroke}" stroke-width="0.1" fill="{fill}" />'.format(
                        points=" ".join("{},{}".format(x, y) for x, y in pts),
                        fill=fill,
                        stroke=stroke,
                    )
                    for (pts, fill) in lines
                ),
            ),
            file=fp,
        )


if __name__ == "__main__":
    main()
