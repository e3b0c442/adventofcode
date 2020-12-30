#include <errno.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lib.h"

static int part1(const char *input, size_t input_len)
{
    int houses = 0;

    int x = 0, y = 0, min_x = 0, max_x = 0, min_y = 0, max_y = 0;
    for (int i = 0; i < input_len; i++)
    {
        switch (input[i])
        {
        case '^':
            y += 1;
            if (y > max_y)
                max_y = y;
            break;
        case '>':
            x += 1;
            if (x > max_x)
                max_x = x;
            break;
        case 'v':
            y -= 1;
            if (y < min_y)
                min_y = y;
            break;
        case '<':
            x -= 1;
            if (x < min_x)
                min_x = x;
            break;
        default:
            errno = EINVAL;
            return -1;
        }
    }

    int w = max_x - min_x + 1, h = max_y - min_y + 1;
    bool grid[w][h];
    memset(grid, 0, w * h * sizeof(bool));

    x = -min_x, y = -min_y;

    grid[x][y] = true;
    for (int i = 0; i < input_len; i++)
    {
        switch (input[i])
        {
        case '^':
            y += 1;
            break;
        case '>':
            x += 1;
            break;
        case 'v':
            y -= 1;
            break;
        case '<':
            x -= 1;
            break;
        }
        grid[x][y] = true;
    }

    for (int i = 0; i < w; i++)
        for (int j = 0; j < h; j++)
            if (grid[i][j])
                houses++;

    return houses;
}

static int part2(const char *input, size_t input_len)
{
    int houses = 0;

    int x[2] = {0}, y[2] = {0}, min_x = 0, max_x = 0, min_y = 0, max_y = 0, s = 0;
    for (int i = 0; i < input_len; i++)
    {
        switch (input[i])
        {
        case '^':
            y[s] += 1;
            if (y[s] > max_y)
                max_y = y[s];
            break;
        case '>':
            x[s] += 1;
            if (x[s] > max_x)
                max_x = x[s];
            break;
        case 'v':
            y[s] -= 1;
            if (y[s] < min_y)
                min_y = y[s];
            break;
        case '<':
            x[s] -= 1;
            if (x[s] < min_x)
                min_x = x[s];
            break;
        default:
            errno = EINVAL;
            return -1;
        }
        s ^= 1;
    }

    int w = max_x - min_x + 1, h = max_y - min_y + 1;
    bool grid[w][h];
    memset(grid, 0, w * h * sizeof(bool));

    x[0] = -min_x, x[1] = -min_x, y[0] = -min_y, y[1] = -min_y, s = 0;

    grid[x[s]][y[s]] = true;
    for (int i = 0; i < input_len; i++)
    {
        switch (input[i])
        {
        case '^':
            y[s] += 1;
            break;
        case '>':
            x[s] += 1;
            break;
        case 'v':
            y[s] -= 1;
            break;
        case '<':
            x[s] -= 1;
            break;
        }
        grid[x[s]][y[s]] = true;
        s ^= 1;
    }

    for (int i = 0; i < w; i++)
        for (int j = 0; j < h; j++)
            if (grid[i][j])
                houses++;

    return houses;
}

int day3(const char *filename)
{
    printf("Day 3: Perfectly Spherical Houses in a Vacuum\n");

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
