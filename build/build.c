#include <stdio.h>
#include <stdlib.h>
int function(int n,int a)
{
char tmp[3] = "\0";
snprintf(tmp, 3, "%d",n<3);
if(atoi(tmp)){
return 1;
}
int b = 1;
return b;
}
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
for(int a = 1;a<5;a++)
{
printf("%s\n", "for!");
}

printf("%s\n", "hello world");
printf("%s\n", atoi(aa)? "true": "false");
int c=function(number,1);
printf("%d\n", atoi(c));
  return 0;
}