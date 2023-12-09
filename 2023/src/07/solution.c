#include "../shared/aoc.h"
#include "../shared/utils.h"
#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef SOLUTION
#define GAMES 1000
#else
#define GAMES 5
#endif

#define CARD_LEN 5

#define CARD_T 10
#define CARD_J 11
#define CARD_Q 12
#define CARD_K 13
#define CARD_A 14

typedef enum
{
  HighCard = 0,
  OnePair = 1,
  TwoPair = 2,
  ThreeOfAKind = 3,
  FullHouse = 4,
  FourOfAKind = 5,
  FiveOfAKind = 6,
} Type;

char
char_to_num (char c)
{
  switch (c)
    {
    case 'T':
      return CARD_T;
    case 'J':
      return CARD_J;
    case 'Q':
      return CARD_Q;
    case 'K':
      return CARD_K;
    case 'A':
      return CARD_A;
    default:
      return 0;
    }
}

typedef struct
{
  int bid;
  Type type;
  int cards[CARD_LEN];
} Game;

Game
game_new_from_line (char *line)
{
  Game game = { 0 };

  for (int i = 0; i < 5; i++)
    {
      char c = line[i];
      if (isdigit (c))
        game.cards[i] = c - '0';
      else
        game.cards[i] = char_to_num (c);
    }
  // skip over game cards and the following space
  line += CARD_LEN + 1;
  // game game bid
  game.bid = atoi (line);

  // calculate card type
  int instances[2][5] = { 0 };
  int numberCount = 0;
  for (int i = 0; i < 5; i++)
    {
      int index = arr_find_index (instances[0], CARD_LEN, game.cards[i]);
      if (index == -1)
        {
          instances[0][numberCount] = game.cards[i];
          instances[1][numberCount++] = 1;
          continue;
        }
      instances[1][index]++;
    }

  int max = arr_max_element (instances[1], 5);

  if (instances[1][0] == 5)
    game.type = FiveOfAKind;
  else if (max == 4)
    game.type = FourOfAKind;
  else if (max == 3 && numberCount == 2)
    game.type = FullHouse;
  else if (max == 3 && numberCount == 3)
    game.type = ThreeOfAKind;
  else if (max == 2 && numberCount == 3)
    game.type = TwoPair;
  else if (max == 2 && numberCount == 4)
    game.type = OnePair;
  else
    game.type = HighCard;
  return game;
}

int
game_cmp (Game a, Game b)
{
  if (a.type != b.type)
    {
      return a.type > b.type;
    }

  int i = 0;
  while (a.cards[i] == b.cards[i])
    i++;
  return a.cards[i] > b.cards[i];
}

void
games_sort (Game *games, int len)
{
  for (int i = 0; i < len; i++)
    {
      for (int j = 0; j < len - 1; j++)
        {
          if (game_cmp (games[j], games[j + 1]))
            {
              Game temp = games[j];
              games[j] = games[j + 1];
              games[j + 1] = temp;
            }
        }
    }
}

unsigned
part_one (char *input)
{
  unsigned sum = 0;
  Game games[GAMES];

  char *line;
  line = strtok (input, "\n");
  int games_n = 0;
  do
    {
      Game game = game_new_from_line (line);
      games[games_n] = game;
      games_n++;
    }
  while ((line = strtok (NULL, "\n")) != NULL);

  games_sort (games, games_n);
  for (int i = 0; i < games_n; i++)
    {
      Game game = games[i];
      sum += game.bid * (i + 1);
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
  print_result (part_one, 6440);
  print_result (part_two, 0);
  return EXIT_SUCCESS;
}
