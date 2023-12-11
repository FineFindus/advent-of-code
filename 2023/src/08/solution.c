#include "../shared/aoc.h"
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define ID_SIZE 4

typedef struct
{
  char pos[ID_SIZE];
  char left[ID_SIZE];
  char right[ID_SIZE];
} Node;

Node
node_new_from_str (char *input)
{
  Node node = { 0 };
  strncpy (node.pos, input, 3);
  // skip 'AAA = ('
  input += 7;
  strncpy (node.left, input, 3);

  // skip 'BBB, '
  input += 5;
  strncpy (node.right, input, 3);
  return node;
}

Node
nodes_find_node (char *value, Node *nodes, unsigned len)
{
  for (int i = 0; i < len; i++)
    {
      Node node = nodes[i];
      if (strncmp (node.pos, value, 3) == 0)
        {
          return node;
        }
    }
  fprintf (stderr, "Failed to get value");
  exit (1);
}

int
nodes_find_node_index (char *value, Node *nodes, unsigned len)
{
  for (int i = 0; i < len; i++)
    {
      Node node = nodes[i];
      if (strncmp (node.pos, value, 3) == 0)
        {
          return i;
        }
    }
  return -1;
}

long
node_cycle_length (int starting_index, Node *nodes, char *input)
{
  long steps = 0;
  Node node = nodes[starting_index];
  char *directions = strtok (input, "\n");
  while (node.pos[2] != 'Z')
    {
      char *nextpose;
      if (*directions == '\0')
        directions = strtok (input, "\n");

      if (*directions == 'R')
        nextpose = node.right;
      else
        nextpose = node.left;

      directions++;
      steps++;
      node = nodes_find_node (nextpose, nodes, 1024);
    }
  return steps;
}

unsigned
part_one (char *input)
{
  unsigned steps = 0;
  char *line, *directions;
  directions = strtok (input, "\n");

  // skip empty line
  line = strtok (NULL, "\n");

  Node nodes[1024];
  int i = 0;
  do
    {
      Node node = node_new_from_str (line);
      nodes[i++] = node;
    }
  while ((line = strtok (NULL, "\n")) != NULL);

  Node node = nodes_find_node ("AAA", nodes, 1024);
  while (strncmp (node.pos, "ZZZ", 3))
    {
      char *nextpose;
      if (*directions == '\0')
        directions = strtok (input, "\n");

      if (*directions == 'R')
        nextpose = node.right;
      else
        nextpose = node.left;

      directions++;
      steps++;
      node = nodes_find_node (nextpose, nodes, 1024);
    }

  return steps;
}

long
gcd (long a, long b)
{
  while (b != 0)
    {
      long temp = b;
      b = a % b;
      a = temp;
    }
  return a;
}

long
lcm (long a, long b)
{
  return (a * b) / gcd (a, b);
}

unsigned
part_two (char *input)
{
  char *line;
  line = strtok (input, "\n");

  // skip empty line
  line = strtok (NULL, "\n");

  Node nodes[1024];
  int nodes_index = 0;
  int nodes_indices[6] = { -1 };
  for (int i = 0; i < 6; i++)
    nodes_indices[i] = -1;

  int starting_index = 0;
  do
    {
      Node node = node_new_from_str (line);
      nodes[nodes_index] = node;

      if (node.pos[2] == 'A')
        {
          nodes_indices[starting_index++] = nodes_index;
        }
      nodes_index++;
    }
  while ((line = strtok (NULL, "\n")) != NULL);

  // calculate cycle
  long long cycle_lengths[6];
  for (int i = 0; i < 6; i++)
    {
      int index = nodes_indices[i];
      if (index == -1)
        break;
      cycle_lengths[i] = node_cycle_length (index, nodes, input);

      // printf ("For %s cycle: %ld\n", nodes[nodes_indices[i]].pos,
      //         cycle_lengths[i]);
    }

  // find least common multiple
  long long total_steps = cycle_lengths[0];

  for (int i = 1; i < 6; i++)
    {
      if (cycle_lengths[i] == 0)
        continue;

      total_steps = lcm (total_steps, cycle_lengths[i]);
    }

  // template does not easily allow for returning value larger than UNIT_MAX,
  // so print the result instead
  printf ("%lld\n", total_steps);
  return total_steps;
}

int
main (int argc, char *argv[])
{
  print_result (part_one, 2);
  print_result (part_two, 6);
  return EXIT_SUCCESS;
}
