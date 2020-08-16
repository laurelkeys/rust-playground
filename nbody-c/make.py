import os

from sys import argv, platform

# https://benchmarksgame-team.pages.debian.net/benchmarksgame/program/nbody-gcc-8.html
# https://benchmarksgame-team.pages.debian.net/benchmarksgame/description/nbody.html#nbody

WIN = platform == "win32"

SRC = "nbody.c"
EXE = "nbody-c"
EXT = ".exe" if WIN else ""

GCC_ARGS = [
    "-O3",
    "-Wall",
    "-fomit-frame-pointer",
    "-march=native",
    "-funroll-loops",
    "-static",
    SRC,
    "-o",
    EXE + EXT,
    "-lm"
]

if __name__ == "__main__":
    if len(argv) == 1:
        os.system(" ".join(["gcc"] + GCC_ARGS))

    elif argv[1] == "time" and platform != "win32":
        os.system(" ".join(["time", "gcc"] + GCC_ARGS))

    elif argv[1] == "clean":
        os.system(f"rm {EXE + EXT}")
