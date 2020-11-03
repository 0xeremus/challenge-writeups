#include <stdio.h>

int main(void)
{
    int Vin;
    float V1 = 3.56;
    int V3; 
    float Vacc = 0.0;
    float V2 = 0.0;
    float check = 4550.799805;

    scanf("%d", &Vin);
    V3 = (int)((float) Vin + V1); 
    
    while(V2 < (float) V3)
    {
        Vacc = (float) V3 + V2 + Vacc;    
        V2 = V2 + 0.8000001;
    }

    printf("%f\n", Vacc); // added \n
    
    if (Vacc == check)
        printf("Good Password\n"); // added \n
    else 
        printf("Bad Password\n");


    return 0;
}
