

int strings_demo()
{
    char *str;

    str = (char *)malloc(2);
    strcpy(str, "hello world");

    free(str);
    
    // use after free
    printf("data = %s, addr = %u\n", str, str);

    return (0);
}