#include "../shared/aoc.h"
#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef SOLUTION
#define NUMS 21
#else
#define NUMS 6
#endif

void
parse_numbers (int *out, char *input)
{
  int index = 0;
  char *p = input;
  while (*p != '\0')
    {
      if (isdigit (*p) || *p == '-')
        {
          int number = strtol (p, &p, 10);
          out[index++] = number;
        }
      else
        {
          p++;
        }
    }
}

int
calculate_next_number (int *arr, unsigned len)
{
  for (int i = 0; i < len - 1; i++)
    {
      int diff = arr[i + 1] - arr[i];
      arr[i] = diff;
    }

  len--;
  int last = arr[len];
  return (arr[0] == 0 && memcmp (arr, arr + 1, (len) * sizeof (arr[0])) == 0)
             ? last
             : last + calculate_next_number (arr, len);
}

unsigned
part_one (char *input)
{
  unsigned sum = 0;
  char *line;
  line = strtok (input, "\n");
  do
    {
      int history[NUMS];
      parse_numbers (history, line);
      sum += calculate_next_number (history, NUMS);
    }
  while ((line = strtok (NULL, "\n")) != NULL);
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
  print_result (part_one, 114);
  print_result (part_two, 0);
  return EXIT_SUCCESS;
}
