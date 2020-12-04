#!/bin/bash
# BTS script will Build, Test, and Run all examples
# by changing to a chapter folder and running various
# cargo commands to build, test, and the examples.

chapter=("01" "02" "03" "04" "05" "06" "07" "08" "09" "10a" "10b" "11" "12" "13")

folder_counter=0
for i in "${chapter[@]}"
do
    cd chapter_$i
  
    cargo build --all
    [ $? -eq 0 ]  || exit 1
    
    cargo t --all
    [ $? -eq 0 ]  || exit 1

    counter=0
    while [ $counter -le $folder_counter ]
    do
        cargo run --example chapter_"${chapter[counter]}" --release
        ((counter++))
    done

    cd ..

    ((folder_counter++))
done

while [[ $# -gt 0 ]]
do
key="$1"

case $key in
    -d|--delete)
    shift
    find . -type d \( -name target -o -path name \) -prune -false -o -name '*.ppm' -print | xargs rm
    ;;
esac
done

