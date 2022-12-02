#! /bin/bash

echo "Compiling..."
ghc Main.hs
echo "Running..."
cat input.txt | ./Main
echo ""