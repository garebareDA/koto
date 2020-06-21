#include <stdio.h>
#include <stdlib.h>
int main() {
char string[] = "a";
int number = 1;
int bools = 0;
char aa[14] = "\0";
snprintf(aa, 14, "%d",number+1+1<1*1);
char tmp[15] = "\0";
snprintf(tmp, 15, "%d",number<atoi(aa));
if(atoi(tmp)){
printf("%s\n", "if!");
}
printf("%s\n", "hello world");
printf("%s\n", atoi(aa)? "true": "false");
  return 0;
}