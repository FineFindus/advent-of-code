CFLAGS = -lm -std=gnu11 -march=native -Ofast -Wall -Werror -pedantic

fmt:
	find . -type f -iname '*.h' -o -iname '*.c' | xargs clang-format -i --style=GNU

scaffold/%:
	mkdir src/$*
	# download input file
	aoc --year 2023 -d $* --input-file ./src/$*/input.txt download
	#create empty example file
	touch src/$*/example.txt
	#setup template file
	cp src/template.c src/$*/solution.c

test/%:
	gcc -DDAY='"$*"' ${CFLAGS} -o solution src/$*/solution.c src/shared/* && ./solution

solve/%:
	gcc -DDAY='"$*"' -D SOLUTION ${CFLAGS} -o solution src/$*/solution.c src/shared/* && ./solution

gdb/%:
	gcc -DDAY='"$*"' ${CFLAGS} -o solution -g src/$*/solution.c src/shared/* && gdb solution

gdb-solve/%:
	gcc -DDAY='"$*"' -D SOLUTION ${CFLAGS} -o solution -g src/$*/solution.c src/shared/* && gdb solution
