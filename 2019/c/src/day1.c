#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lib.h"

static int input_to_ints(const char *input, size_t input_len, int **output)
{
    char **lines = NULL;
    int *entries = NULL;
    int line_count = input_to_lines(input, input_len, &lines);
    if (line_count < 0)
        goto err_cleanup;

    entries = calloc(line_count, sizeof(int));
    if (entries == NULL)
        goto err_cleanup;

    for (int i = 0; i < line_count; i++)
    {
        entries[i] = atoi(lines[i]);
        if (entries[i] == 0 && lines[i][0] != '0')
        {
            errno = -EINVAL;
            goto err_cleanup;
        }
    }
    free_lines(lines, line_count);

    *output = entries;
    return line_count;

err_cleanup:
    free_lines(lines, line_count);
    if (entries == NULL)
        free(entries);
    return -errno;
}

static int recurse_fuel(int val)
{
    return val / 3 - 2 <= 0 ? 0 : val / 3 - 2 + recurse_fuel(val / 3 - 2);
}

static int part1(const char *input, size_t input_len)
{
    int *mods;
    int line_count = input_to_ints(input, input_len, &mods);
    if (line_count < 0)
        return line_count;

    int fuel = 0;
    for (int i = 0; i < line_count; i++)
        fuel += mods[i] / 3 - 2;

    //free(mods);
    return fuel;
}

static int part2(const char *input, size_t input_len)
{
    int *mods;
    int line_count = input_to_ints(input, input_len, &mods);
    if (line_count < 0)
        return line_count;

    int fuel = 0;
    for (int i = 0; i < line_count; i++)
        fuel += recurse_fuel(mods[i]);

    free(mods);
    return fuel;
}

int day1(const char *filename)
{
    printf("Day 1: The Tyranny of the Rocket Equation\n");

    char *input;
    int filesize = read_file_to_buffer(&input, filename);
    if (filesize < 0)
        return -errno;

    int rval = part1(input, filesize);
    if (rval < 0)
    {
        errno = -rval;
        goto err_cleanup;
    }
    printf("\tPart 1: %d\n", rval);

    rval = part2(input, filesize);
    if (rval < 0)
    {
        errno = -rval;
        goto err_cleanup;
    }
    printf("\tPart 2: %d\n", rval);

    free(input);
    return 0;

err_cleanup:
    if (input != NULL)
        free(input);
    return -errno;
}
