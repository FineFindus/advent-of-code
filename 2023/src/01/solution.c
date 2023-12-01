#include "../shared/aoc.h"
#include <assert.h>
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

unsigned
part_one (char *input)
{
  unsigned sum = 0;
  char *p, *temp;
  p = strtok_r (input, "\n", &temp);
  do
    {
      unsigned firstDigit = 0, lastDigit = 0;
      for (int i = 0; p[i] != 0; i++)
        {
          char c = p[i];
          if (isdigit (c))
            {
              if (firstDigit == 0)
                {
                  firstDigit = c - '0';
                }
              else
                {
                  lastDigit = c - '0';
                }
            }
        }
      if (lastDigit == 0)
        {
          lastDigit = firstDigit;
        }
      sum += firstDigit * 10 + lastDigit;
    }
  while ((p = strtok_r (NULL, "\n", &temp)) != NULL);

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
  test (part_one, 142, part_two, 0);
  print_result (part_one);
  print_result (part_two);
  return EXIT_SUCCESS;
}
