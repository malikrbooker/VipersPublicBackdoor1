#!/usr/bin/python3

# This is a simple Caesar cypher encryption
# script that you can use to evade pre-runtime
# detection.
while True:
    string = input("enter a string: ")
    print(string, "-> ", end="")

    for char in string:
        print(chr(ord(char)+1), end="")
        
    print("\n")