#include "../shared/aoc.h"
#include <assert.h>
#include <ctype.h>
#include <math.h>
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

      if (isdigit (c))
        {
          int part_number = atoi (input + i);
          if (part_number != 0)
            {
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
        }
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
  print_result (part_one, 4361);
  print_result (part_two, 0);
  return EXIT_SUCCESS;
}
