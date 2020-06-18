#include <stdio.h>
#include <stdlib.h>
int main() {
char string[] = "a";
int number = 1;
int bools = 0;
char aa[28] = "\0";
snprintf(aa, 28, "%s%d%s%d%d","sonic",1+1,"sonic",1,number);
printf("%s\n", "hello world");
printf("%d\n", aa);
  return 0;
}