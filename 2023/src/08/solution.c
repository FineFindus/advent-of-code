#include "../shared/aoc.h"
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
  printf ("Node: %s, L: %s, R: %s\n", node.pos, node.left, node.right);
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

unsigned
part_two (char *input)
{
  return 0;
}

int
main (int argc, char *argv[])
{
  print_result (part_one, 2);
  print_result (part_two, 0);
  return EXIT_SUCCESS;
}
