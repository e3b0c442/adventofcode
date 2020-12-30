#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include "lib.h"

static int part1(const char *input, size_t input_len)
{
    int floor = 0;
    for (int i = 0; i < input_len; i++)
    {
        switch (input[i])
        {
        case '(':
            floor++;
            break;
        case ')':
            floor--;
            break;
        default:
            errno = EINVAL;
            return -1;
        }
    }

    return floor;
}

static int part2(const char *input, size_t input_len)
{
    int floor = 0;
    for (int i = 0; i < input_len; i++)
    {
        switch (input[i])
        {
        case '(':
            floor++;
            break;
        case ')':
            floor--;
            break;
        default:
            errno = EINVAL;
            return -1;
        }

        if (floor == -1)
            return i + 1;
    }
    return -EINVAL;
}

int day1(const char *filename)
{
    printf("Day 1: Not Quite Lisp\n");

    char *input;
    int filesize = read_file_to_buffer(&input, filename);
    if (filesize < 0)
        return -1;

    int rval = part1(input, filesize);
    if (rval < 0)
        return rval;
    printf("\tPart 1: %d\n", rval);

    rval = part2(input, filesize);
    if (rval < 0)
        return rval;
    printf("\tPart 2: %d\n", rval);

    free(input);

    return 0;
}
