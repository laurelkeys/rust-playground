from os import system, path
from sys import argv, platform

# https://benchmarksgame-team.pages.debian.net/benchmarksgame/program/nbody-gcc-8.html
# https://benchmarksgame-team.pages.debian.net/benchmarksgame/description/nbody.html#nbody

dir_path = path.dirname(path.realpath(__file__))
in_windows = (platform == "win32")

src = path.join(dir_path, "nbody.c")
exe = path.join(dir_path, "nbody-c")
ext = ".exe" if in_windows else ""

GCC_ARGS = [
    "-O3",
    "-Wall",
    "-fomit-frame-pointer",
    "-march=native",
    "-funroll-loops",
    "-static",
    src,
    "-o",
    exe + ext,
    "-lm"
]

if __name__ == "__main__":
    if len(argv) == 1:
        system(" ".join(["gcc"] + GCC_ARGS))

    elif argv[1] == "time" and not in_windows:
        system(" ".join(["time", "gcc"] + GCC_ARGS))

    elif argv[1] == "clean":
        system(f"rm {exe + ext}")
