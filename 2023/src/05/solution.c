#include "../shared/aoc.h"
#include <ctype.h>
#include <limits.h>
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

void
parse_ranges (Range ranges[7][RANGE_MAX_LEN])
{
  char *line;
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
}

void
map_ranges (Range ranges[7][RANGE_MAX_LEN], long *seeds, int seed_len)
{
  for (int i = 0; i < 7; i++)
    {

      for (int k = 0; k < seed_len; k++)
        {
          long seed = seeds[k];
          for (int j = 0; j < RANGE_MAX_LEN; j++)
            {
              Range range = ranges[i][j];

              if (range_is_empty (range))
                continue;

              if (seed >= range.src_start
                  && seed < range.src_start + range.len)
                {
                  long move = range.dst_start - range.src_start;
                  seeds[k] = seed + move;
                }
            }
        }
    }
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
    }

  // read ranges
  Range ranges[7][RANGE_MAX_LEN] = { 0 };
  parse_ranges (ranges);

  // map to new ranges
  map_ranges (ranges, seeds, SEED_NUM);

  // find smallest location
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
    }

  // read ranges
  Range ranges[7][RANGE_MAX_LEN] = { 0 };
  parse_ranges (ranges);

  long smallest = LONG_MAX;
#pragma omp parallel for
  for (int i = 0; i < SEED_NUM; i += 2)
    {
      long seed_start = seeds[i];
      long seed_length = seeds[i + 1];

      long current_seed[1] = { 0 };
      for (int k = 0; k < seed_length; k++)
        {
          current_seed[0] = seed_start + seed_length - k - 1;
          map_ranges (ranges, current_seed, 1);
          if (current_seed[0] < smallest)
            smallest = current_seed[0];
        }
    }

  // find smallest location
  return smallest;
}

int
main (int argc, char *argv[])
{
  print_result (part_one, 35);
  print_result (part_two, 46);
  return EXIT_SUCCESS;
}
