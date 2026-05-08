#!/bin/env python3
from __future__ import annotations
import argparse
import typing as t
from pathlib import Path
import subprocess
import contextlib
import shutil
import os

bin = Path("./bin")
def main():
    with contextlib.suppress(FileExistsError):
        os.mkdir(bin)
    p = _get_parser()
    args = p.parse_args()

    if args.build:
        compile(args.target)
    


def _get_parser()->argparse.ArgumentParser:
    parser = argparse.ArgumentParser()
    parser.suggest_on_error = True # If available, enable
    exclusive = parser.add_mutually_exclusive_group()
    exclusive.add_argument(
            "--path",
        help="The path of the compiled binary",
        type=Path,
        # required=True,
# metavar="triple",
    )
    exclusive.add_argument(
        *[
            "--build",
        ],
        help="Have this program auto build",
        action="store_true",
        default=False,
    )
    parser.add_argument(
        *[
        "--target"
        ],
        help="The target triple",
        metavar="TARGET-TRIPLE",
        type=TargetTriple,
        required=True,
        default=TargetTriple("x86_64-unknown-none")
    )
    

    return parser


def compile(triple:"TargetTriple"):
    # import subprocess
    args = ["cargo", "build"]
    args.append(f"--target={triple}")
    if triple.is_uefi:
        args.append("--features=uefi")
    proc:subprocess.CompletedProcess[str]
    try:
        proc = subprocess.run(
        *args,check=True
        )
    except subprocess.CalledProcessError as e:
        raise CompilerFailedError("RUSTC has failed. Either this is a us bug or rust bug.") from e
    
def image(kernel_bin:Path):
    """
    This will create a boot image
    """
    
    
    # UEFI firmwhere
    shutil.copyfile("/usr/share/OVMF/OVMF_CODE.fd",bin)
    shutil.copyfile("/usr/share/OVMF/OVMF_VARS.fd",bin)

    dfile = bin / "disk.dd"
    subprocess.run( # create an empty disk image file, 128 megabytes in size. 
        *["dd","if=/dev/zero",f"of={dfile}", "bs=1048576", "count=128"],
        check=True
    )
    losetup_cmd = [f"losetup -o {2048*512} --sizelimit {8*1024*1024} --show -f ".split(),f"{dfile}"]
    print("We need you to enter your password to allow loop devices")
    e = subprocess.run(
        *losetup_cmd,
    )
    if e.returncode != 0 and "Permission denied" in e.stderr:
        # We use pkexec instead of sudo due to two things
        # 1. We don't need to interact with passwords
        # 2. We don't need to use `sudo -S`

        e = subprocess.run(
            ["pkexec" "--keep-cwd", *losetup_cmd],
            check=True
        )

class TargetTriple:
    __arch:str
    __is_uefi:bool
    def __init__(self,v:str):
        t= v.split("-")
        if len(t) < 3:
            raise ValueError(f"Malformed Target triple {v!r}")
        self.__arch = t[0]
        self.__is_uefi = True if t[-1] == "uefi" else False
    
    def __repr__(self) -> str:
        v = "uefi" if self.__is_uefi else "none"
        arch = self.__arch
        return "{arch}-unknown-{v}"
    @property    
    def arch(self)->str:
        """
        Architecture of the target triple
        """
        return self.__arch
    @arch.setter
    def arch(self,_):
        raise AttributeError("TargetTriple's arch attribute is immutable")

    @arch.deleter
    def arch(self):
        raise AttributeError("TargetTriple's arch attribute is immutable")
    
    def is_uefi(self)->bool:
        # don't need to use a setter
        return self.__is_uefi 


class CompilerFailedError(RuntimeError):
    """
    Used with `--build` flag

    The compiler has failed
    """
    ...


if __name__ == "__main__":
    main()
