
void arrays_demo()
{
    int numbers[5] = {10, 20, 30, 40, 50};
    numbers[4] = 500;
    //numbers[5] = 1000;

    for (int i = 0; i <= 5; i++)
    {
        // illegal memory access on final iteration
        printf("\nnum: %d, addr: %u", numbers[i], &numbers[i]);
    }
}