#!/usr/bin/python3

from os import system

compiler = "/bin/x86_64-w64-mingw32-windres"
icon_src = "../resources/src/icon.rc"
icon_dst = "../resources/out/icon.res"
vers_src = "../resources/src/version.rc"
vers_dst = "../resources/out/version.res"

if __name__ == "__main__":

    print("Generating .res files")
    system(compiler + " -Ocoff " + icon_src + " -o " + icon_src)
    system(compiler + " -Ocoff " + vers_src + " -o " + vers_dst)

    print("success fully generated.")