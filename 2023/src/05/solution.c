#include "../shared/aoc.h"
#include <ctype.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef SOLUTION
#define SEED_NUM 20
#define RANGE_MAX_LEN 50
#else
#define SEED_NUM 4
#define RANGE_MAX_LEN 4
#endif

#define SKIP_WHITESPACES(s)                                                   \
  while (*(s) == ' ')                                                         \
    (s)++;

#define SEED_TO_SOIL_HEADER "seed-to-soil map:"
#define SOIL_TO_FERTILIZER_HEADER "soil-to-fertilizer map:"
#define FERTILIZER_TO_WATER_HEADER "fertilizer-to-water map:"
#define WATER_TO_LIGHT_HEADER "water-to-light map:"
#define LIGHT_TO_TEMPERATURE_HEADER "light-to-temperature map:"
#define TEMPERATURE_TO_HUMIDITY_HEADER "temperature-to-humidity map:"
#define HUMIDITY_TO_LOCATION_HEADER "humidity-to-location map:"

void
read_numbers (char *input, int *out)
{
  if (input == NULL || out == NULL)
    return;

  int i = 0;
  while (input[0] != '\0' && input[0] != '\n')
    {
      out[i] = atoi (input);
      i++;
      int number_len = out[i] == 0 ? 1 : floor (log10 (out[i])) + 1;
      input += number_len;
      SKIP_WHITESPACES (input);
      printf ("Input: %s\n", input);
    }
}

typedef struct
{
  long dst_start;
  long src_start;
  long len;
} Range;

Range
range_new_from_line (char *line)
{
  Range range;
  unsigned numbers[3] = { 0 };
  for (int i = 0; i < 3; i++)
    {
      numbers[i] = atol (line);
      int number_len = numbers[i] == 0 ? 1 : floor (log10 (numbers[i])) + 1;
      line += number_len + 1;
    }

  range.dst_start = numbers[0];
  range.src_start = numbers[1];
  range.len = numbers[2];
  return range;
}

int
range_is_empty (Range range)
{
  return range.src_start == 0 && range.dst_start == 0 && range.len == 0;
}

unsigned
part_one (char *input)
{
  char *line;
  line = strtok (input, "\n");
  // parse starting seeds
  // skip "seeds: "
  line += 7;
  // store them as longs, as they can be pretty large
  long seeds[SEED_NUM] = { 0 };
  // scan seeds
  for (int i = 0; i < SEED_NUM; i++)
    {
      SKIP_WHITESPACES (line);
      long number = atol (line);
      seeds[i] = number;
      int number_len = floor (log10 (number)) + 1;
      line += number_len;
      printf ("Seed: %ld\n", seeds[i]);
    }

  // read ranges
  // set up ranges
  Range ranges[7][RANGE_MAX_LEN] = { 0 };
  int map_index = -1;
  int range_index = 0;
  while ((line = strtok (NULL, "\n")) != NULL)
    {
      if (isdigit (line[0]))
        {
          Range range = range_new_from_line (line);
          ranges[map_index][range_index] = range;
          range_index++;
        }
      else if (line[0] != '\n' && isalpha (line[0]))
        {
          map_index++;
          range_index = 0;
        }
    }
  char *mapNames[]
      = { "seed-to-soil map:\n",         "soil-to-fertilizer map:\n",
          "fertilizer-to-water map:\n",  "water-to-light map:\n",
          "light-to-temperature map:\n", "temperature-to-humidity map:\n",
          "humidity-to-location map:\n", "" };

  for (int i = 0; i < 7; i++)
    {

      for (int k = 0; k < SEED_NUM; k++)
        {

          long seed = seeds[k];
          printf ("%s", mapNames[i]);
          for (int j = 0; j < RANGE_MAX_LEN; j++)
            {
              Range range = ranges[i][j];

              if (range_is_empty (range))
                continue;

              if (seed >= range.src_start
                  && seed <= range.src_start + range.len)
                {
                  long move = range.dst_start - range.src_start;
                  seeds[k] = seed + move;
                  printf ("%ld is in range %ld..%ld, mapped to %ld\n", seed,
                          range.src_start, range.src_start + range.len,
                          seeds[k]);
                }
            }
        }
    }

  long smallest = seeds[0];
  for (int i = 1; i < SEED_NUM; i++)
    {
      if (seeds[i] < smallest)
        smallest = seeds[i];
    }
  return smallest;
}

unsigned
part_two (char *input)
{
  return 0;
}

int
main (int argc, char *argv[])
{
  print_result (part_one, 35);
  print_result (part_two, 0);
  return EXIT_SUCCESS;
}
