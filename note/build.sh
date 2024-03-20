#!/bin/sh
f='main'

# generate dvi
platex -interaction=nonstopmode "$f".tex
if [ $? -eq 0 ]; then
    echo "compile 1 is successed!";
else
    echo -e "failure! in compile 1, please read ${f}.log";
    exit 1
fi

grep -q "Rerun to get" ${f}.log || [ -f ${f}.toc ] && platex -interaction=nonstopmode "$f".tex 
if [ $? -eq 0 ]; then 
    echo "compile 2 is successed!";
fi

# generate pdf
dvipdfmx "$f".dvi
        
