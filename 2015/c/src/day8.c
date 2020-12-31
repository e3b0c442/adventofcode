#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lib.h"

static int part1(const char *input, size_t input_len)
{
    int code = 0;
    int mem = 0;
    int in = false;

    for (int i = 0; i < input_len; i++)
    {
        if (!in)
        {
            if (input[i] == '"')
            {
                in = true;
                code++;
            }
        }
        else
        {
            switch (input[i])
            {
            case '"':
                in = false;
                code++;
                break;

            case '\\':
                code++;
                mem++;
                i++;
                switch (input[i])
                {
                case '\\':
                case '"':
                    code++;
                    break;
                case 'x':
                    code += 3;
                    i += 2;
                    break;
                default:
                    return -1;
                }
                break;
            default:
                code++;
                mem++;
            }
        }
    }
    return code - mem;
}

static int part2(const char *input, size_t input_len)
{
    char **lines = NULL;
    int lines_len = input_to_lines(&lines, input, input_len);
    if (lines_len < 1)
        return -1;

    int raw = 0;
    int encoded = 0;
    for (int i = 0; i < lines_len; i++)
    {
        encoded += 2;
        for (int j = 0; j < strlen(lines[i]); j++)
        {
            raw += 1;
            switch (lines[i][j])
            {
            case '"':
            case '\\':
                encoded += 2;
                break;
            default:
                encoded += 1;
            }
        }
    }
    free_lines(lines, lines_len);
    return encoded - raw;
}

int day8(const char *filename)
{
    printf("Day 8: Matchsticks\n");

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
