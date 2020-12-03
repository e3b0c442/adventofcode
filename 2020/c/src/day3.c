#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lib.h"

static int slope_trees(char **rows, size_t rows_len, int slope[2])
{
    int x = 0, trees = 0;
    int r = slope[0], d = slope[1];
    for (int i = 0; i < rows_len; i += d)
    {
        if (x >= strlen(rows[i]))
            x = x - strlen(rows[i]);
        if (rows[i][x] == '#')
            trees++;
        x += r;
    }
    return trees;
}

static int part1(const char *input, size_t input_len)
{
    char **lines = NULL;
    int line_count = input_to_lines(input, input_len, &lines);
    if (line_count < 0)
        goto err_cleanup;

    int trees = slope_trees(lines, line_count, (int[2]){3, 1});

    free_lines(lines, line_count);
    return trees;

err_cleanup:
    free_lines(lines, line_count);
    return -errno;
}

static int part2(const char *input, size_t input_len)
{
    char **lines = NULL;
    int line_count = input_to_lines(input, input_len, &lines);
    if (line_count < 0)
        goto err_cleanup;

    int slopes[5][2] = {{1, 1}, {3, 1}, {5, 1}, {7, 1}, {1, 2}};
    int trees = 1;
    for (int i = 0; i < 5; i++)
    {
        trees *= slope_trees(lines, line_count, slopes[i]);
    }

    free_lines(lines, line_count);
    return trees;

err_cleanup:
    free_lines(lines, line_count);
    return -errno;
}

int day3(const char *filename)
{
    printf("Day 3: Toboggan Trajectory\n");

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
