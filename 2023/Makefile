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
	gcc -DDAY='"$*"' -D TEST ${CFLAGS} -o solution src/$*/solution.c src/shared/* && ./solution

solve/%:
	gcc -DDAY='"$*"' ${CFLAGS} -o solution src/$*/solution.c src/shared/* && ./solution

