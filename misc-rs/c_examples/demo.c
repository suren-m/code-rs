#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include "int_pointers.c"
#include "structs.c"
#include "strings.c"
#include "arrays.c"

int main()
{
    printf("\n..starting..\n\n");

    //arrays_demo();
    //int_pointer_demo();
    //strings_demo();
    struct_demo();

    // Strings are just char arrays terminated by \0
    printf("\n\n..done.\0.I won't print 😀\n");
}