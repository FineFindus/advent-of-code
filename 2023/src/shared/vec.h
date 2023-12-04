#ifndef VECTOR_H
#define VECTOR_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct
{
  void *data;
  int element_size;
  int len;
} Vec;

void vec_init (Vec *vec, int capacity, int element_size);
void vec_push (Vec *vec, const void *value);
void vec_free (Vec *vec);

#endif // VECTOR_H
