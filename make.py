from os import system, path
from sys import platform
from argparse import ArgumentParser

# http://cliffle.com/p/dangerust
# https://benchmarksgame-team.pages.debian.net/benchmarksgame/program/nbody-gcc-8.html
# https://benchmarksgame-team.pages.debian.net/benchmarksgame/description/nbody.html#nbody

dir_path = path.dirname(path.realpath(__file__))
in_windows = platform == "win32"

C, Rust = "c", "rust"

subdir = { C: "nbody-c", Rust: path.join("nbody-rs", "src")}
src = { C: path.join(dir_path, subdir[C], "main.c"), Rust: path.join(dir_path, subdir[Rust], "main.rs") }
exe = { C: path.join(dir_path, subdir[C], "nbody-c"), Rust: path.join(dir_path, subdir[Rust], "nbody-rs") }
ext = ".exe" if in_windows else ""

compile_ = {
    C: [
        "gcc",
        "-O3",
        "-fomit-frame-pointer",
        "-march=native",
        "-funroll-loops",
        "-static",
        src[C], "-o", exe[C] + ext,
        "-lm",
    ],
    Rust: [
        "rustc",
        "-C", "opt-level=3",
        "-C", "target-cpu=native",
        "-C", "codegen-units=1",
        src[Rust], "-o", exe[Rust] + ext,
    ]
}

if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("rule", nargs="?", choices=["clean", "time"])
    parser.add_argument("--c_only", "-c", action="store_true")
    parser.add_argument("--rust_only", "-rs", "-rust", action="store_true")

    args = parser.parse_args()

    if args.rule == "clean":
        if not args.rust_only: system(f"rm {exe[C] + ext}")
        if not args.c_only: system(f"rm {exe[Rust] + ext} {exe[Rust]}.pdb")

    elif args.rule == "time":
        if in_windows:
            print("The `time` command only works in *nix")
        else:
            if not args.rust_only: system(" ".join(["time"] + compile_[C]))
            if not args.c_only: system(" ".join(["time"] + compile_[Rust]))

    else:
        if not args.rust_only: system(" ".join(compile_[C]))
        if not args.c_only: system(" ".join(compile_[Rust]))
