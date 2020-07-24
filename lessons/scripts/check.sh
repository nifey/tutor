#!/bin/bash
rustc $1 -o check_executable 2>/dev/null
if [ $? -eq 0 ]
then
	./check_executable > out
	if [ `diff out $2 | wc -l` -eq 0 ]
	then
		rm check_executable out
		exit 0
	fi
fi
rm check_executable out
exit 1
