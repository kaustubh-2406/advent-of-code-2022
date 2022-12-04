#include <stdio.h>
#include <stdlib.h>  // atoi
#include <ctype.h>   // isdigit
#include <string.h>  // for strncat
#include <stdbool.h> // true and false
#include <assert.h>  // assert statement

// Used to move one step back in a file stream..
void move_one_step_back(FILE *f)
{
  fseek(f, -1L, SEEK_CUR);
}

// Consumes Integer if given the file input stream
// Caution: Can only consume a maximum of 256 character long int(s).
int consume_int(FILE *f)
{
  char buffer[256] = "";

  while (true)
  {
    char c = getc(f);

    if (isdigit(c))
      // add the character to buffer string..
      strncat(buffer, &c, 1);
    else
      break;
  }

  move_one_step_back(f);
  return atoi(buffer);
}

int is_eof(FILE *f)
{
  char c = getc(f);
  if (c != EOF)
  {
    move_one_step_back(f);
    return false;
  }
  return true;
}

struct Data
{
  int start;
  int end;
};

struct Data parse_line(FILE *f)
{
  int start = consume_int(f);
  char _ = getc(f); // consume ","
  int end = consume_int(f);
  char __ = getc(f); // consume newline character

  struct Data d = {start, end};

  return d;
}

int main()
{
  FILE *f = fopen("input.txt", "r");

  // throw error if file is not found..
  assert(f != NULL);

  int overlapped_entries = 0;
  int any_overlap = 0;

  while (true)
  {
    if (is_eof(f))
      break;

    struct Data first = parse_line(f);
    struct Data second = parse_line(f);

    int a1 = first.start;
    int a2 = first.end;
    int b1 = second.start;
    int b2 = second.end;

    // part 1 logic...
    if ((a1 <= b1 && a2 >= b2) || (a1 >= b1 && a2 <= b2))
      overlapped_entries += 1;

    // part 2 logic...
    if (
        (b1 >= a1 && b1 <= a2) ||
        (b2 >= a1 && b2 <= a2) ||
        (a1 >= b1 && a1 <= b2) ||
        (a2 >= b1 && a2 <= b2))
      any_overlap += 1;
  }

  printf("Part 1 answer: %d\n", overlapped_entries);
  printf("Part 2 answer: %d\n", any_overlap);
  return 0;
}