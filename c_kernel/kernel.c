extern void print_string();
extern void kmain();

void _start() { kmain(); }

void kmain() {
  char *video = (char *)0xB8000;

  *video = 'H';
  video++;
  *video = 4;

  /* print_string(4, "Hello!"); */

  while (1)
    ;
}

void print_string(int colour, char *message) {
  char *video = (char *)0xB8000;
  char *curr_char = message;

  while (*curr_char) {
    *video = *curr_char;
    /* *video = 'H'; */
    curr_char++;
    video += 1;
    *video = colour;
    video += 1;
    /* video++; */
  }
}
