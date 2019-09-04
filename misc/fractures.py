#!/usr/bin/env python3

import argparse
import copy
import glob
import math
import os
import random
import shutil
import subprocess
import tempfile


PALETTE = [
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
]


def lerp(a, b, t):
    ax, ay = a
    bx, by = b
    cx = ax + (bx - ax) * t
    cy = ay + (by - ay) * t
    return (cx, cy)


def dist2(a, b):
    ax, ay = a
    bx, by = b
    return (bx - ax) ** 2 + (by - ay) ** 2


class Polyline:
    def __init__(self, points, color=None, gen=0):
        self.points = points
        self.color = color
        self.gen = gen

        self.min_x = min((p[0] for p in self.points))
        self.min_y = min((p[1] for p in self.points))
        self.max_x = max((p[0] for p in self.points))
        self.max_y = max((p[1] for p in self.points))

    def opposite(self, i):
        return (i + len(self) // 2) % len(self)

    def after(self, i):
        return self[(i + 1) % len(self)]

    def __len__(self):
        return len(self.points)

    def __getitem__(self, k):
        return self.points[k]

    def __iter__(self):
        return iter(self.points)

    def bbox_area(self):
        return (self.max_x - self.min_x) * (self.max_y - self.min_y)


def run(polygons, cuts, t1, t2):
    # poly_i = random.randrange(0, len(polygons))
    poly_i = max(range(len(polygons)), key=lambda p: polygons[p].bbox_area())

    poly = polygons[poly_i]
    assert len(poly) >= 3, len(poly)

    edge0 = max(
        range(len(poly)),
        key=lambda i: min(
            dist2(poly[i], poly.after(i)),
            dist2(poly[poly.opposite(i)], poly.after(poly.opposite(i))),
        ),
    )
    edge1 = poly.opposite(edge0)
    edge0, edge1 = (edge0, edge1) if edge0 < edge1 else (edge1, edge0)

    newp0 = lerp(poly[edge0], poly.after(edge0), t1)
    newp1 = lerp(poly[edge1], poly.after(edge1), t2)

    newpoly0 = []
    newpoly1 = []

    for (i, p0) in enumerate(poly):
        if i == edge0:
            newpoly0.extend([p0, newp0])
            newpoly1.append(newp0)
            continue

        if i == edge1:
            newpoly0.append(newp1)
            newpoly1.extend([p0, newp1])
            continue

        if i < edge0 or i > edge1:
            newpoly0.append(p0)

        if edge0 < i < edge1:
            newpoly1.append(p0)

    polygons[poly_i] = polygons[-1]
    polygons.pop()

    polygons.append(Polyline(newpoly0, color=random.choice(PALETTE), gen=poly.gen + 1))
    polygons.append(Polyline(newpoly1, color=random.choice(PALETTE), gen=poly.gen + 1))

    cuts.append(Polyline([newp0, newp1], color="black", gen=poly.gen + 1))


def circle_subdivisions(cx, cy, r, steps):
    return Polyline(
        [
            (
                cx + r * math.cos(i / (steps - 1) * 2 * math.pi),
                cy + r * math.sin(i / (steps - 1) * 2 * math.pi),
            )
            for i in range(steps)
        ]
    )


def svg_polygon(points, stroke="black", fill=None, stroke_width=2):
    return '<polygon points="{}" fill="{}" stroke="{}" stroke-width="{}" shape-rendering="geometricPrecision" />'.format(
        " ".join(("{},{}".format(x, y) for (x, y) in points)),
        "none" if fill is None else fill,
        stroke,
        stroke_width,
    )


def dump_svg(filename, dimensions, polygons, cuts):
    width, height = dimensions

    with open(filename, "wt") as fp:
        max_gen = max(cuts, key=lambda c: c.gen).gen

        print(
            """<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox="0 0 {width} {height}">
<rect width="{width}" height="{height}" stroke="none" fill="crimson" />
{polygons}
{cuts}
</svg>""".format(
                width=width,
                height=height,
                polygons="\n".join(
                    (svg_polygon(poly, fill=poly.color) for poly in polygons)
                ),
                cuts="\n".join(
                    (
                        svg_polygon(
                            cut,
                            stroke=cut.color,
                            stroke_width=10 - 9.9 * cut.gen / max(max_gen, 1),
                        )
                        for cut in cuts
                    )
                ),
            ),
            file=fp,
        )


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("-w", "--width", default=1920, help="width of the svg")
    parser.add_argument("--height", default=1080, help="height of the svg")
    parser.add_argument(
        "-n",
        "--iterations",
        default=None,
        help="number of iterations of the algorithm, random if not specified",
    )
    parser.add_argument(
        "-s",
        "--shape",
        choices=["rect", "circle"],
        default=None,
        help="shape of the boundary of the fragments, random if not specified",
    )
    parser.add_argument(
        "--frames",
        action="store_true",
        default=False,
        help="also dump the intermediate svg frames used to render the video",
    )
    parser.add_argument(
        "--no-video",
        action="store_true",
        default=False,
        help="generate a video showing how the final shape came to be, requires ffmpeg",
    )
    parser.add_argument(
        "-o",
        "--output",
        default="fractures",
        help="the name of the final svg and mp4 without extension",
    )
    return parser.parse_args()


def main():
    args = parse_args()
    width = args.width
    height = args.height
    iterations = args.iterations or random.randrange(80, 200)

    if args.shape == "rect" or (args.shape is None and random.random() > 0.5):
        polys = [
            Polyline(
                [(0, 0), (0, height), (width, height), (width, 0)],
                color=random.choice(PALETTE),
            )
        ]
    else:
        polys = [
            circle_subdivisions(
                width / 2, height / 2, min(height, height) / 2 - 20, steps=50
            )
        ]

    cuts = []
    for poly in polys:
        cuts.append(copy.deepcopy(poly))
        cuts[-1].color = "black"

    cwd = os.getcwd()

    with tempfile.TemporaryDirectory() as tmpdir:
        os.chdir(tmpdir)

        dump_svg("{}-0.svg".format(args.output), (width, height), polys, cuts)
        for i in range(iterations):
            t1 = 0.25 + 0.5 * random.random()
            # t2 = 0.25 + 0.5 * random.random()
            t2 = t1
            run(polys, cuts, t1, t2)

            dump_svg(
                "{}-{}.svg".format(args.output, i + 1), (width, height), polys, cuts
            )

        shutil.copy(
            "{}-{}.svg".format(args.output, iterations),
            "{}.svg".format(os.path.join(cwd, args.output)),
        )

        if not args.no_video:
            # fmt: off
            subprocess.check_call(
                [
                    "ffmpeg",
                    "-framerate", "25",
                    "-i", "{}-%00d.svg".format(args.output),
                    "-c:v", "libx264",
                    "-pix_fmt", "yuv420p",
                    "-y", "{}.mp4".format(args.output),
                ],
            )
            # fmt: on

            shutil.copy(
                "{}.mp4".format(args.output),
                "{}.mp4".format(os.path.join(cwd, args.output)),
            )

        if args.frames:
            for f in glob.iglob("{}*.svg".format(args.output)):
                shutil.copy(f, os.path.join(cwd, f))


if __name__ == "__main__":
    main()
