#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lib.h"

static int line_to_dims(int dims[3], char *line)
{
    dims[0] = atoi(strtok(line, "x"));
    if (dims[0] == 0)
        goto err_cleanup;

    dims[1] = atoi(strtok(NULL, "x"));
    if (dims[1] == 0)
        goto err_cleanup;

    dims[2] = atoi(strtok(NULL, "x"));
    if (dims[2] == 0)
        goto err_cleanup;

    qsort(dims, 3, sizeof(int), cmp_int);
    return 0;

err_cleanup:
    errno = EINVAL;
    return -1;
}

static int part1(const char *input, size_t input_len)
{
    int paper = 0;

    char **lines;
    int line_count = input_to_lines(&lines, input, input_len);
    if (line_count < 0)
        return -1;

    for (int i = 0; i < line_count; i++)
    {
        int dims[3];
        if (line_to_dims(dims, lines[i]))
            goto err_cleanup;

        paper += 2 * dims[0] * dims[1] + 2 * dims[1] * dims[2] + 2 * dims[2] * dims[0] + dims[0] * dims[1];
    }
    goto cleanup;

err_cleanup:
    paper = -1;
cleanup:
    free_lines(lines, line_count);
    return paper;
}

static int part2(const char *input, size_t input_len)
{
    int ribbon = 0;

    char **lines;
    int line_count = input_to_lines(&lines, input, input_len);
    if (line_count < 0)
        return -1;

    for (int i = 0; i < line_count; i++)
    {
        int dims[3];
        if (line_to_dims(dims, lines[i]))
            goto err_cleanup;

        ribbon += 2 * dims[0] + 2 * dims[1] + dims[0] * dims[1] * dims[2];
    }
    goto cleanup;

err_cleanup:
    ribbon = -1;
cleanup:
    free_lines(lines, line_count);
    return ribbon;
}

int day2(const char *filename)
{
    printf("Day 2: I Was Told There Would Be No Math\n");

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
