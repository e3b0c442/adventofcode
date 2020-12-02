#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lib.h"

static int input_to_ints(const char *input, size_t input_len, int **output)
{
    char **lines;
    int line_count = input_to_lines(input, input_len, &lines);
    if (line_count < 0)
        goto err_cleanup;

    int *entries = calloc(line_count, sizeof(int));
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

static int part1(const char *input, size_t input_len)
{
    int rval = -2;

    int *entries;
    int line_count = input_to_ints(input, input_len, &entries);
    if (line_count < 0)
        return line_count;

    for (int i = 0; i < line_count; i++)
        for (int j = i + 1; j < line_count; j++)
            if (entries[i] + entries[j] == 2020)
            {
                rval = entries[i] * entries[j];
                goto cleanup;
            }
cleanup:
    free(entries);
    return rval;
}

static int part2(const char *input, size_t input_len)
{
    int rval = -2;

    int *entries;
    int line_count = input_to_ints(input, input_len, &entries);
    if (line_count < 0)
        return line_count;

    for (int i = 0; i < line_count; i++)
        for (int j = i + 1; j < line_count; j++)
            for (int k = j + 1; k < line_count; k++)
                if (entries[i] + entries[j] + entries[k] == 2020)
                {
                    rval = entries[i] * entries[j] * entries[k];
                    goto cleanup;
                }
cleanup:
    free(entries);
    return rval;
}

int day1(const char *filename)
{
    printf("Day 1: Report Repair\n");

    char *input;
    int filesize = read_file_to_buffer(&input, filename);
    if (filesize < 0)
        return -errno;

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
