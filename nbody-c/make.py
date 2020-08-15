import os

from sys import argv

# https://benchmarksgame-team.pages.debian.net/benchmarksgame/program/nbody-gcc-8.html
# https://benchmarksgame-team.pages.debian.net/benchmarksgame/description/nbody.html#nbody

EXECUTABLE = "nbody-c.exe"

if __name__ == "__main__":
    if len(argv) > 1 and argv[1] == "clean":
        os.system(f"rm {EXECUTABLE}")
    else:
        os.system(
            " ".join(
                [
                    "gcc",
                    "-pipe",
                    "-Wall",
                    "-O3",
                    "-fomit-frame-pointer",
                    "-march=ivybridge",
                    "-funroll-loops",
                    "-static",
                    "main.c",
                    "-o",
                    EXECUTABLE,
                    "-lm"
                ]
            )
        )
