#include <stdio.h>
#include <stdlib.h>
int function(int n,int a)
{
char tmp[3] = "\0";
snprintf(tmp, 3, "%d",n<3);
if(atoi(tmp)){
return 