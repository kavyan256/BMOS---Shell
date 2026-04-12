#!/bin/bash

echo "Testing echo command:"
echo Hello World
echo "Multiple" "arguments" "test"

# Test pwd command
echo "Testing pwd command:"
pwd

# Test type command
echo "Testing type command:"
type echo
type pwd
type cd
type nonexistent

# Test cd command (change to home)
echo "Testing cd command:"
cd ~
pwd

# Test output redirection
echo "Testing output redirection:"
echo "This goes to file.txt" > file.txt
echo "This appends to file.txt" >> file.txt

# Test multiple commands in sequence
echo "Testing combined commands:"
pwd
echo "Shell test complete"

# Test exit
exit
