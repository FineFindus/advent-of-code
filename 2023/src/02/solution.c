#include "../shared/aoc.h"
#include <assert.h>
#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

int
parse_set (char *str, int max_red, int max_green, int max_blue)
{
  char *buf, *temp;

  // skip 5 char for "Game "
  str += 5;
  buf = strtok_r (str, ":", &temp);
  int id = atoi (buf);

  char *set = strtok_r (NULL, ";", &temp);
  do
    {
      if (set == NULL)
        continue;
      int red = 0;
      int green = 0;
      int blue = 0;

      char *color = strtok_r (NULL, ",", &set);
      do
        {
          int value = atoi (color);
          while (isalpha (*color) == 0)
            color++;

          if (strcmp (color, "green") == 0 && value <= max_green)
            green += value;
          else if (strcmp (color, "red") == 0 && value <= max_red)
            red += value;
          else if (strcmp (color, "blue") == 0 && value <= max_blue)
            blue += value;
          else
            return 0;
        }
      while ((color = strtok_r (NULL, ",", &set)) != NULL);
    }
  while ((set = strtok_r (NULL, ";", &temp)) != NULL);

  return id;
}

unsigned
part_one (char *input)
{
  unsigned sum = 0;
  char *line, *temp;
  line = strtok_r (input, "\n", &temp);
  do
    {
      sum += parse_set (line, 12, 13, 14);
    }
  while ((line = strtok_r (NULL, "\n", &temp)) != NULL);
  return sum;
}

int
minimum_colors (char *str)
{
  char *temp;

  // skip 5 char for "Game "
  str += 5;
  strtok_r (str, ":", &temp);

  int red = 0;
  int green = 0;
  int blue = 0;
  char *set = strtok_r (NULL, ";", &temp);
  do
    {
      if (set == NULL)
        continue;
      char *color = strtok_r (NULL, ",", &set);
      do
        {
          int value = atoi (color);
          while (isalpha (*color) == 0)
            color++;

          if (strcmp (color, "green") == 0 && value > green)
            green = value;
          else if (strcmp (color, "red") == 0 && value > red)
            red = value;
          else if (strcmp (color, "blue") == 0 && value > blue)
            blue = value;
        }
      while ((color = strtok_r (NULL, ",", &set)) != NULL);
    }
  while ((set = strtok_r (NULL, ";", &temp)) != NULL);

  return red * green * blue;
}

unsigned
part_two (char *input)
{
  // having nearly identical solving functions is pretty wasteful,
  // but I don't want to spent time fixing it…
  unsigned sum = 0;
  char *line, *temp;
  line = strtok_r (input, "\n", &temp);
  do
    {
      sum += minimum_colors (line);
    }
  while ((line = strtok_r (NULL, "\n", &temp)) != NULL);
  return sum;
}

int
main (int argc, char *argv[])
{
  print_result (part_one, 8);
  print_result (part_two, 2286);
  return EXIT_SUCCESS;
}
