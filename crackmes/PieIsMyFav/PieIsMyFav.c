
#include <stdio.h>

int main(void)
{
    int a = 0x64, b = 0xe, c = 0, input = 0;

    printf("Welcome to the wonderful world of assembly\n");
    printf("Qual o numero magico?: ");
    scanf("%d", &input);

    c = a;
    a += a;
    a += c;
    a += b;
    a /= c;

    if (input == a) printf("Essa eh a sua flag!\n");
    else printf("Try Harder\n");

    return 0;
}



