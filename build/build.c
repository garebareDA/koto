#include <stdio.h>
#include <stdlib.h>
int main() {
char string[] = "a";
int number = 1;
int bools = 0;
char aa[20] = "\0";
snprintf(aa, 20, "%d%s%d",number+1,"sonic",1*1);
printf("%s\n", "hello world");
printf("%s\n", aa);
  return 0;
}