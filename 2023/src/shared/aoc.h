#include <stdio.h>
#include <stdlib.h>
#include <time.h>

void print_result (unsigned (*func) (char *));

void test (unsigned (*part_one) (char *), unsigned solution_one,
           unsigned (*part_two) (char *), unsigned solution_two);
