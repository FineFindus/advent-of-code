CFLAGS = -lm -std=gnu11 -march=native -Ofast -Wall -Werror -pedantic -fsanitize=address,undefined -g -fopenmp

fmt:
	find . -type f -iname '*.h' -o -iname '*.c' | xargs clang-format -i --style=GNU

scaffold/%:
	mkdir src/$*
	#create empty example file
	touch src/$*/example.txt
	#setup template file
	cp src/template.c src/$*/solution.c
	# download input file
	aoc --year 2023 -d $* --input-file ./src/$*/input.txt download

test/%:
	gcc -DDAY='"$*"' ${CFLAGS} -o solution src/$*/solution.c src/shared/* && ./solution

solve/%:
	gcc -DDAY='"$*"' -D SOLUTION ${CFLAGS} -o solution src/$*/solution.c src/shared/* && ./solution

gdb/%:
	gcc -DDAY='"$*"' ${CFLAGS} -o solution -g src/$*/solution.c src/shared/* && gdb solution

gdb-solve/%:
	gcc -DDAY='"$*"' -D SOLUTION ${CFLAGS} -o solution -g src/$*/solution.c src/shared/* && gdb solution
