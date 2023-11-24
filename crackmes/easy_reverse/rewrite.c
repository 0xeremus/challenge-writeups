#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Function Prototypes */
void usage(char argv[]);

int main(int argc, char* argv[])
{
    if (argc != 2) 
        usage(argv[0]);
    else if (strlen(argv[1]) != 0xa) 
        usage(argv[0]);
    else if (argv[1][4] != 0x40)
        usage(argv[0]);
    else
        printf("Nice Job!!\n");
        printf("flag{%s}\n", argv[1]);
}

void usage(char argv[])
{
    printf("USAGE: %s <password>\n", argv);
    printf("try again!\n");
    exit(0);
}
