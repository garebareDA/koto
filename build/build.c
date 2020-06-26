#include <stdio.h>
#include <stdlib.h>
int main() {
int number = 1;
char string[34] = "\0";
snprintf(string, 34, "%s%d%s%d","hello, world",1+1+1,"mozirel",123);
printf("%s\n", string);
int vector[5] = {1,2,3,4,5,};
int index = vector[1];
printf("%d\n", index);
int cal = number+1-1/2*5;;
printf("%d\n", cal);
cal++;