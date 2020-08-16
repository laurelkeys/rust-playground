from os import system, path
from sys import argv, platform

# http://cliffle.com/p/dangerust

dir_path = path.dirname(path.realpath(__file__))
in_windows = (platform == "win32")

src = path.join(dir_path, "nbody.rs")
exe = path.join(dir_path, "nbody-rs")
ext = ".exe" if in_windows else ""

RUSTC_ARGS = [
    "-C",
    "opt-level=3",
    "-C",
    "target-cpu=native",
    "-C",
    "codegen-units=1",
    src,
    "-o",
    exe + ext
]

if __name__ == "__main__":
    if len(argv) == 1:
        system(" ".join(["rustc"] + RUSTC_ARGS))

    elif argv[1] == "time" and not in_windows:
        system(" ".join(["time", "rustc"] + RUSTC_ARGS))

    elif argv[1] == "clean":
        system(f"rm {exe + ext} {exe}.pdb")
