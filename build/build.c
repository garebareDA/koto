#include <stdio.h>
#include <stdlib.h>
int function(int n,int a)
{
char tpm0[3] = "\0";
snprintf(tpm0, 3, "%d",n<3);
if(atoi(tpm0)){
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
char tpm1[15] = "\0";
snprintf(tpm1, 15, "%d",number<atoi(aa));
if(atoi(tpm1)){
printf("%s\n", "if!");
}
for(int a = 1;a<5;a++)
{
printf("%s\n", "for!");
}
char tpm2[5] = "\0";
snprintf(tpm2, 5, "%d",1<0+6);
if(atoi(tpm2)){
}

printf("%s\n", "hello world");
printf("%s\n", atoi(aa)? "true": "false");
int c = function(number,1);
printf("%d\n", c);
  return 0;
}