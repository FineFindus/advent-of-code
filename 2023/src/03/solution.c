#include "../shared/aoc.h"
#include <assert.h>
#include <ctype.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

unsigned
part_one (char *input)
{
  unsigned sum = 0;
  unsigned total_len = strlen (input);
  unsigned line_len = 0;
  while (input[line_len] != '\n')
    {
      line_len++;
    }

  for (int i = 0; i < total_len; i++)
    {
      char c = input[i];

      if (c == '\n')
        {
          continue;
        }

      if (!isdigit (c))
        continue;

      int part_number = atoi (input + i);
      int part_num_len = floor (log10 (abs (part_number))) + 1;
      // check characters around
      for (int k = i - 1; k <= i + part_num_len; k++)
        {
          for (int j = -1; j < 2; j++)
            {
              int target_index = k + (j * line_len + j);

              if (target_index < 0 || target_index > total_len)
                continue;

              char check = input[target_index];
              if (check == '.' || check == '\n' || isdigit (check))
                continue;

              sum += part_number;
              goto break_inner_loop;
            }
        }
    break_inner_loop:
      i += part_num_len;
    }
  return sum;
}

bool
in_array (int val, int *arr, int n)
{
  for (size_t i = 0; i < n; i++)
    {
      if (arr[i] == val)
        {
          return true;
        }
    }
  return false;
}

unsigned
part_two (char *input)
{
  unsigned sum = 0;
  unsigned total_len = strlen (input);
  unsigned line_len = 0;
  while (input[line_len] != '\n')
    {
      line_len++;
    }

  for (int i = 0; i < total_len; i++)
    {
      char c = input[i];

      if (c != '*')
        continue;

      // found a potential gear, check for numbers around
      int firstValue = 0;
      int secondValue = 0;
      int check_indices[10] = { 0 };
      int last_index = 0;
      for (int k = i - 1; k <= i + 1; k++)
        {
          for (int j = -1; j < 2; j++)
            {
              int target_index = k + (j * line_len + j);

              if (target_index < 0 || target_index >= total_len)
                continue;

              // check if checked char is a number
              char check = input[target_index];
              // printf ("%c ", check);
              if (!isdigit (check))
                continue;

              // found a number, walk in either direction and try read
              int walk_index = target_index;
              // go left until no number
              while (isdigit (input[walk_index - 1]))
                {
                  walk_index--;
                }

              int value = atoi (input + walk_index);

              if (value == 0 || in_array (walk_index, check_indices, 10))
                continue;

              check_indices[last_index] = walk_index;
              last_index++;

              if (firstValue == 0)
                {
                  firstValue = value;
                }
              else if (secondValue == 0)
                {

                  secondValue = value;
                }
              else
                {
                  // found more than two part numbers
                  goto skip;
                }
            }
        }

      if (secondValue != 0)
        {
          sum += firstValue * secondValue;
        }
    skip:
      i += 1;
    }
  return sum;
}

int
main (int argc, char *argv[])
{
  print_result (part_one, 4361);
  print_result (part_two, 467835);
  return EXIT_SUCCESS;
}
