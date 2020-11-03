#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, const char *argv[])
{
    char *FLAG = "NKRI_1";
    int len;
    
    if (argc < 2) 
    {
        printf("CDC2016{flag}: %s flag\n", *argv);
        exit(1);
    }

    len = strlen(argv[1]);

    if (strncmp(argv[1], FLAG, len) != 0)
    {
        printf("FLAG SALAH\n"); // FLAG FALSE
        exit(1);
    }
    else
        printf("FLAG BENAR\n");


    return 0;
}
