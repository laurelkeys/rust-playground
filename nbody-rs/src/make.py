import os

from sys import argv, platform

# http://cliffle.com/p/dangerust

WIN = platform == "win32"

SRC = "nbody.rs"
EXE = "nbody-rs"
EXT = ".exe" if WIN else ""

RUSTC_ARGS = [
    "-C",
    "opt-level=3",
    "-C",
    "target-cpu=native",
    "-C",
    "codegen-units=1",
    SRC,
    "-o",
    EXE + EXT
]

if __name__ == "__main__":
    if len(argv) == 1:
        os.system(" ".join(["rustc"] + RUSTC_ARGS))

    elif argv[1] == "time" and platform != "win32":
        os.system(" ".join(["time", "rustc"] + RUSTC_ARGS))

    elif argv[1] == "clean":
        os.system(f"rm {EXE + EXT} {EXE}.pdb")
