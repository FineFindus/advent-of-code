#include <stdio.h>
#include <stdlib.h>
#include <string.h>
typedef struct
{
  void *data;
  int element_size;
  int cap;
  int len;
} Vec;

void
vec_init (Vec *vec, int capacity, int element_size)
{
  vec->data = malloc (capacity * element_size);
  if (vec->data == NULL)
    {
      fprintf (stderr, "Failed to allocate vec");
      exit (EXIT_FAILURE);
    }

  vec->element_size = element_size;
  vec->cap = capacity;
  vec->len = 0;
}

void
vec_push (Vec *vec, const void *value)
{
  if (vec->cap == vec->len)
    {
      vec->cap *= 2;
      vec->data = realloc (vec->data, vec->cap * vec->element_size);
      if (vec->data == NULL)
        {
          fprintf (stderr, "Memory reallocation error\n");
          exit (EXIT_FAILURE);
        }
    }

  void *dest = (char *)vec->data + vec->len * vec->element_size;
  memcpy (dest, value, vec->element_size);

  vec->len++;
}

void
vec_free (Vec *vec)
{
  free (vec->data);
  vec->data = NULL;
  vec->cap = vec->len = 0;
}
