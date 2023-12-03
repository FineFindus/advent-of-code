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
  char *line, *temp;
  line = strtok_r (input, "\n", &temp);
  do
    {
      unsigned firstDigit = 0, lastDigit = 0;
      for (int i = 0; line[i] != 0; i++)
        {
          char c = line[i];
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
  while ((line = strtok_r (NULL, "\n", &temp)) != NULL);

  return sum;
}

unsigned
findValue (char *str)
{
  // try to find the substring value
  const char digits[][7] = { "one", "two",   "three", "four", "five",
                             "six", "seven", "eight", "nine" };

  for (int i = 0; i < 9; i++)
    {
      int len = strlen (digits[i]);
      if (strncmp (str, digits[i], len) == 0)
        {
          return i + 1;
        }
    }
  // otherwise return the int value of the first char
  if (isdigit (str[0]))
    {
      return str[0] - '0';
    }
  return 0;
}

unsigned
part_two (char *input)
{
  unsigned sum = 0;
  char *line, *temp;
  line = strtok_r (input, "\n", &temp);
  do
    {
      unsigned firstDigit = 0, lastDigit = 0;
      for (int i = 0; line[i] != 0; i++)
        {
          unsigned value = findValue (line + i);
          if (value != 0)
            {
              if (firstDigit == 0)
                {
                  firstDigit = value;
                  lastDigit = value;
                }
              else
                {
                  lastDigit = value;
                }
            }
        }
      sum += firstDigit * 10 + lastDigit;
    }
  while ((line = strtok_r (NULL, "\n", &temp)) != NULL);
  return sum;
}

int
main (int argc, char *argv[])
{
  print_result (part_one, 142);
  print_result (part_two, 281);
  return EXIT_SUCCESS;
}
