#include <stdio.h>

#define MAX 64

int main(void)
{
    char *a = "Flag{E4sy_ch4ll}", b[MAX];
     
    printf("Hello !\n");
    printf("Give Me Your Flag\n");
    printf("Check Flag: ");
    scanf("%s", b);

    if (*b == *a) printf("G00d\n");
    else printf("Bad\n");
}
