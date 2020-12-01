#!/bin/bash
# BTS script will Build, Test, and Run all examples
# by changing to a chapter folder and running various
# cargo commands to build, test, and the examples.

chapter=9

folder_counter=1
while [ $folder_counter -le $chapter ]
do
folder=chapter_`printf %02d $folder_counter`

cd $folder
cargo build --all
[ $? -eq 0 ]  || exit 1
cargo t --all
[ $? -eq 0 ]  || exit 1

counter=1
while [ $counter -le $folder_counter ]
do
prg=chapter_`printf %02d $counter`
echo $prg
cargo run --example $prg
((counter++))
done

cd ..
((folder_counter++))
done

find . -type f -name '*.ppm' -delete
