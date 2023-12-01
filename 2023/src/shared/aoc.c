#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>

#ifndef DAY
#define DAY "01"
#endif

#define INPUT_FILE_PATH "src/" DAY "/input.txt"
#define EXAMPLE_FILE_PATH "src/" DAY "/example.txt"

static char *
read_input (const char *filename)
{
  FILE *file = fopen (filename, "rb");

  if (!file)
    {
      printf ("[ERROR] File not found\n");
      exit (EXIT_FAILURE);
    }

  fseek (file, 0, SEEK_END);
  size_t length = ftell (file);
  fseek (file, 0, SEEK_SET);

  char *buffer = (char *)malloc (length + 1);

  size_t read = length > 0 ? fread (buffer, 1, length, file) : 0;
  if (read != length)
    {
      free (buffer);
      printf ("[ERROR] Failed to read file\n");
      exit (EXIT_FAILURE);
    }

  fclose (file);

  buffer[length] = '\0';

  return buffer;
}

void
test (unsigned (*part_one) (char *), unsigned solution_one,
      unsigned (*part_two) (char *), unsigned solution_two)
{
#ifndef TEST
  return;
#endif

  char *input = read_input (EXAMPLE_FILE_PATH);
  assert (part_one (input) == solution_one);
  assert (part_two (input) == solution_two);
}

void
print_result (unsigned (*func) (char *))
{
#ifdef TEST
  char *filename = EXAMPLE_FILE_PATH;
#else
  char *filename = INPUT_FILE_PATH;
#endif
static int part = 0;
part++;

  struct timeval start, end;

  gettimeofday (&start, NULL);

  char *input = read_input (filename);
  unsigned solution = func (input);
  printf ("🎄 \x1b[1mPart %d\x1b[0m 🎄\n\n", part);

  gettimeofday (&end, NULL);
  long seconds = end.tv_sec - start.tv_sec;
  long microseconds = end.tv_usec - start.tv_usec;
  double elapsed = seconds + microseconds * 1e-6;

  printf ("%u \x1b[3m(Elapsed: %.2lfµs)\x1b[0m\n\n", solution, elapsed * 1e6);
}
