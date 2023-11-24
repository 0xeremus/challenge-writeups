
#include <stdio.h>
#include <string.h>

#define MAX 48 // Arbitary I have no idea what this should be

int main(void)
{
    char password[MAX] = "easycrack\n", test[MAX];
    
     printf("Input the password: ");
     scanf("%s", test);

     if (!strcmp(test, password)) printf("Wrong password, try another.\n");
     else printf("Correct password\n");
}

