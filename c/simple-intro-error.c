#include <stdio.h>

int main(){
    int initial_value = 60;
    int items[5] = {1,2,3,4,5};
    items[5] = 6; // put one too many in items
    printf("initial value is %d\n", initial_value);
}