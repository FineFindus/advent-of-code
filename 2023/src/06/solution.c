#include "../shared/aoc.h"
#include <ctype.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define SKIP_TO_DIGIT(s)                                                      \
  while (!isdigit (*(s)))                                                     \
    (s)++;

#ifdef SOLUTION
#define RACES 4
#else
#define RACES 3
#endif

void
read_digits (int *arr, char **input)
{
  for (int i = 0; i < RACES; i++)
    {
      SKIP_TO_DIGIT (*input);
      int number = atoi (*input);
      int num_len = floor (log10 (number)) + 1;
      arr[i] = number;
      *input += num_len;
    }
}

unsigned
part_one (char *input)
{
  int times[RACES] = { 0 };
  int record_distance[RACES] = { 0 };
  // skip "Time:      "
  input += 11;
  read_digits (times, &input);
  printf ("Leftoveer input: %s\n", input);

  // skip "Distance:  "
  input += 11;
  read_digits (record_distance, &input);

  unsigned sum = 1;
  for (int i = 0; i < RACES; i++)
    {
      int time = times[i];
      int record = record_distance[i];
      printf ("Time: %d Record: %d\n", time, record);

      double deltaRoot = sqrt (time * time - (4 * record));
      double solutionStart = (((-time) + deltaRoot) / -2);
      double solutionEnd = (((-time) - deltaRoot) / -2);

      solutionStart = (ceil (solutionStart) == solutionStart)
                          ? ceil (solutionStart) + 1
                          : ceil (solutionStart);
      solutionEnd = (ceil (solutionEnd) == solutionEnd)
                        ? ceil (solutionEnd) - 1
                        : (int)solutionEnd;

      sum *= solutionEnd - solutionStart + 1;
    }

  return sum;
}

unsigned
part_two (char *input)
{
  return 0;
}

int
main (int argc, char *argv[])
{
  print_result (part_one, 288);
  print_result (part_two, 0);
  return EXIT_SUCCESS;
}
