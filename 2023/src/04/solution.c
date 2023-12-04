#include "../shared/aoc.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#ifdef TEST
#define WINNING_NUMBERS 5
#define CARD_NUMBERS 8
#else
#define WINNING_NUMBERS 10
#define CARD_NUMBERS 25
#endif /* ifdef TEST */

void read_numbers (char **input, int *arr, char delim);

typedef struct
{
  int id;
  int winning_numbers[WINNING_NUMBERS];
  int numbers[25];
} Card;

Card
card_new (char *input)
{
  Card card;
  // skip "Card "
  input += 5;

  // read id, and skip to first winning number
  int id = atoi (input);
  card.id = id;
  input = strstr (input, ":");
  input += 2;

  // read winning numbers
  read_numbers (&input, card.winning_numbers, '|');

  // skip over '| '
  input += 2;

  // read card numbers
  read_numbers (&input, card.numbers, '\n');

  return card;
}

void
read_numbers (char **input, int *arr, char delim)
{
  if (input == NULL || arr == NULL)
    return;

  int i = 0;
  while (*input[0] != '\0' && *input[0] != delim)
    {
      int number = atoi (*input);
      arr[i] = number;
      i++;

      if (strlen (*input) <= 3)
        {
          break;
        }
      *input += 3;
    }
}

int
find_index (int *arr, int target, unsigned len)
{
  if (arr == NULL)
    return -1;

  for (unsigned i = 0; i < len; i++)
    {
      if (arr[i] == target)
        {
          return i;
        }
    }

  return -1;
}

unsigned
part_one (char *input)
{
  unsigned sum = 0;
  char *line, *temp;
  line = strtok_r (input, "\n", &temp);
  do
    {
      unsigned points = 0;
      Card card = card_new (line);
      for (int i = 0; i < CARD_NUMBERS; i++)
        {
          int number = card.numbers[i];
          int index
              = find_index (card.winning_numbers, number, WINNING_NUMBERS);
          if (index != -1)
            {
              if (points == 0)
                points = 1;
              else
                points *= 2;
            }
        }
      sum += points;
    }
  while ((line = strtok_r (NULL, "\n", &temp)) != NULL);
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
  print_result (part_one, 13);
  print_result (part_two, 0);
  return EXIT_SUCCESS;
}
