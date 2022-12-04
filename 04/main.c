#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <string.h>
#include <stdbool.h>
#include <assert.h>

// Can only consume a maximum of 256 character long int(s).
int consume_int(FILE *f)
{
  char str[256] = "";

  while (true)
  {
    const char c = getc(f);

    if (isdigit(c))
      strncat(str, &c, 1);
    else
      break;
  }

  fseek(f, -1L, SEEK_CUR);
  return atoi(str);
}

int is_end(FILE *f)
{
  char c = getc(f);
  if (c != EOF)
  {
    fseek(f, -1L, SEEK_CUR);
    return false;
  }
  return true;
}

int main()
{
  FILE *f = fopen("input.txt", "r");

  // throw error if file is not found..
  assert(f != NULL);

  int overlapped_entries = 0;

  while (true)
  {
    if (is_end(f))
      break;

    int a1 = consume_int(f);
    char _ = getc(f); // consume ","
    int a2 = consume_int(f);
    char __ = getc(f); // consume newline character

    int b1 = consume_int(f);
    char ___ = getc(f); // consume ","
    int b2 = consume_int(f);
    char ____ = getc(f); // consume newline character

    // part 1 logic...
    if ((a1 <= b1 && a2 >= b2) || (a1 >= b1 && a2 <= b2))
      overlapped_entries += 1;
  }

  printf("overlaps: %d\n", overlapped_entries);
  return 0;
}