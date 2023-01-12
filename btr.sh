#!/bin/bash
# I'm not shell script expert take with a grain of salt

function usage {
        echo "Usage: $(basename $0) [-aucfpbtrd]" 2>&1
        echo 'Build, Test, and Run Example from Ray Tracer Challenge.'
        echo ''
        echo '   -a          All (Update, Clean, Format, Clippy, Build, Test, Run)'
        echo '   -u          Cargo Update.'
        echo '   -c          Cargo clean.'
        echo '   -f          Cargo format.'
        echo '   -p          Cargo clippy.'
        echo '   -b          Cargo build.'
        echo '   -t          Cargo test.'
        echo '   -r          Run examples.'
        echo ''
        echo '   -o          Optomized build (--release).'
        echo ''
        echo '   -d          Delete PPM files.'
        
        exit 1
}


UPDATE='false'
CLEAN='false'
FORMAT='false'
CLIPPY='false'
BUILD='false'
TEST='false'
RUN='false'

DELETE='false'
RELEASE='false'

NOARGS='true'

optstring=":aucfpbtrod"
while getopts ${optstring} arg; do
  case ${arg} in
    a)
      UPDATE='true'
      CLEAN='true'
      FORMAT='true'
      CLIPPY='true'
      BUILD='true'
      TEST='true'
      RUN='true'
      ;;
    u)
      UPDATE='true'
      ;;
    c)
      CLEAN='true'
      ;;
    f)
      FORMAT='true'
      ;;
    p)
      CLIPPY='true'
      ;;
    b)
      BUILD='true'
      ;;
    t)
      TEST='true'
      ;;
    r)
      RUN='true'
      ;;  
    d)
      DELETE='true'
      ;;
    o)
      RELEASE='true'
      ;;      
    :)
      echo "$0: Must supply an argument to -$OPTARG." >&2
      exit 1
      ;;
    ?)
      usage
      exit 1
      ;;
  esac
  NOARGS='false'
done

if [[ "${NOARGS}" == true ]];then 
  usage
  exit 1
fi

chapter=("01" "02" "03" "04" "05" "06" "07" "08" "09" "10a" "10b" "11" "12" "13" "14" "15" "16")

folder_counter=0
for i in "${chapter[@]}"
do
    printf '***************\n'
    printf 'Chapter %s\n' $i
    printf '***************\n'
    printf '\n'
    printf '\n'

    cd chapter_$i
  
    if [[ "${UPDATE}" == true ]];then
      cargo update
    fi
    if [[ "${CLEAN}" == true ]];then
      cargo clean
    fi
    if [[ "${FORMAT}" == true ]];then
      cargo fmt
    fi
    if [[ "${CLIPPY}" == true ]];then
      cargo clippy
    fi

    if [[ "${BUILD}" == true ]];then
      cargo build --all
      [ $? -eq 0 ]  || exit 1
    fi

    if [[ "${RELEASE}" == true ]];then
      cargo build --all --release
      [ $? -eq 0 ]  || exit 1
    fi

    if [[ "${TEST}" == true ]];then
      cargo t --all
      [ $? -eq 0 ]  || exit 1
    fi
    
    if [[ "${RUN}" == true ]];then
      # run all prior example upto the current chapter index
      counter=0
      while [ $counter -le $folder_counter ]
      do
          cargo run --example chapter_"${chapter[counter]}" --release
          ((counter++))
      done
    fi

    if [[ "${CLEAN}" == true ]];then
      cargo clean
    fi

    if [[ "${DELETE}" == true ]];then
      rm *.ppm
    fi

    cd ..

    ((folder_counter++))
done