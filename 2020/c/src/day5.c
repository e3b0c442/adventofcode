#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include "lib.h"

int intcmp(const void *a, const void *b)
{
    int ia = *(const int *)a, ib = *(const int *)b;
    if (ia < ib)
        return -1;
    if (ia > ib)
        return 1;
    return 0;
}

static int part1(const char *input, size_t input_len, int **output)
{
    char **lines = NULL;
    int *ids = NULL;
    int line_count = input_to_lines(input, input_len, &lines);
    if (line_count < 0)
        goto err_cleanup;

    ids = calloc(line_count, sizeof(int));
    if (ids == NULL)
        goto err_cleanup;

    for (int i = 0; i < line_count; i++)
    {
        int row_bottom = 0, row_top = 127;
        for (int j = 0; j < 7; j++)
        {
            int split = (row_top + 1 - row_bottom) / 2;
            switch (lines[i][j])
            {
            case 'F':
                row_top -= split;
                break;
            case 'B':
                row_bottom += split;
                break;
            default:
                goto err_cleanup;
            }
        }
        if (row_bottom != row_top)
        {
            printf("row bottom %d top %d\n", row_bottom, row_top);
            goto err_cleanup;
        }

        int col_bottom = 0, col_top = 7;
        for (int j = 7; j < 10; j++)
        {
            int split = (col_top + 1 - col_bottom) / 2;
            switch (lines[i][j])
            {
            case 'L':
                col_top -= split;
                break;
            case 'R':
                col_bottom += split;
                break;
            default:
                goto err_cleanup;
            }
        }
        if (col_bottom != col_top)
        {
            printf("col bottom %d top %d\n", col_bottom, col_top);
            goto err_cleanup;
        }

        ids[i] = row_top * 8 + col_top;
    }
    qsort(ids, line_count, sizeof(int), intcmp);
    free_lines(lines, line_count);
    *output = ids;
    return line_count;

err_cleanup:
    free_lines(lines, line_count);
    if (ids != NULL)
        free(ids);
    return -errno;
}

static int part2(int *ids, size_t ids_len)
{
    for (int i = 0; i < ids_len - 1; i++)
        if (ids[i + 1] != ids[i] + 1)
            return ids[i] + 1;

    return -1;
}

int day5(const char *filename)
{
    printf("Day 5: Binary Boarding\n");

    char *input;
    int filesize = read_file_to_buffer(&input, filename);
    if (filesize < 0)
        return -errno;

    int *ids;
    int ids_len = part1(input, filesize, &ids);
    if (ids_len < 0)
        return ids_len;
    printf("\tPart 1: %d\n", ids[ids_len - 1]);

    int rval = part2(ids, ids_len);
    if (rval < 0)
        return rval;
    printf("\tPart 2: %d\n", rval);

    free(input);
    free(ids);

    return 0;
}
