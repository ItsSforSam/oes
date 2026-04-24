#!/bin/env python3
from __future__ import annotations
import argparse
import typing as t
from pathlib import Path
def main():
    p = _get_parser()


def _get_parser()->argparse.ArgumentParser:
    parser = argparse.ArgumentParser()
    parser.suggest_on_error = True # If available, enable

    parser.add_argument(
        *[
            "--path"
        ],
        help="The path of the compiled binary"
# metavar="triple",
        type=Path
    )
    parser.add_argument(
        *[""
        "--target"
        ],
        help="The target triple",
        metavar="TARGET-TRIPLE"
    )


    return parser

class TargetTriple:
    __arch:str
    __is_uefi:bool
    def __init__(self,v:str):
        ...
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


if __name__ == "__main__":
    main()
