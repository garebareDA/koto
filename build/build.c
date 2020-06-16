#include <stdio.h>
#include <stdlib.h>
int main() {
char string[] = "a";
int number = 1;
int bools = 0;
char aa[1000] = "\0";
snprintf(aa, 1000, "%d%s",  1 + 1, "sonic");
char stdin[1000];
scanf("%s",stdin);
printf("%s\n", "hello world");
printf("%s\n", stdin);
printf("%s\n", aa);
  return 0;
}