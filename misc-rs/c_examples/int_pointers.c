#include <stdio.h>

void int_pointer_demo()
{
    int a = 10;
    int *ref_a = &a;

    //printf("%x\n", ref_a);
    printf("Addr of a: %u Val of a via deref:%d\n", &a, *&a);

    printf("\nBefore Null: addr: %u, addr_of_ptr: %u deref_val: %u\n", 
        ref_a, &ref_a, *ref_a); 
    
    ref_a = NULL;
    
    // deref a null pointer
    // asking for seg fault
    printf("After null: addr: %u, addr_of_ptr: %u\n", ref_a, &ref_a);  
    printf("deref val:%d\n", *ref_a); 

}
