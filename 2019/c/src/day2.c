#include <errno.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lib.h"

static int input_to_intcode(const char *input, size_t input_len, int **output)
{
    //copy input to avoid mangling
    char input_cpy[input_len + 1];
    memcpy(input_cpy, input, input_len + 1);

    //get the number of ints for correct allocation
    int ic = 1;
    for (int i = 0; i < input_len; i++)
        if (input_cpy[i] == ',')
            ic++;

    //allocate the array
    int *code = calloc(ic, sizeof(int));
    if (code == NULL)
        goto err_cleanup;

    //parse the code
    char *istr = strtok(input_cpy, ",");
    int i = 0;
    while (istr != NULL)
    {
        code[i] = atoi(istr);
        if (code[i] == 0 && istr[0] != '0')
            goto err_cleanup;
        istr = strtok(NULL, ",");
        i++;
    }
    *output = code;
    return ic;

err_cleanup:
    if (code != NULL)
        free(code);
    return -errno;
}

static int run_intcode(int *intcode, int code_len)
{
    //copy to avoid mangling
    int ic[code_len];
    memcpy(ic, intcode, code_len * sizeof(int));

    int ip = 0;
    while (1)
    {
        switch (ic[ip])
        {
        case 1:
            ic[ic[ip + 3]] = ic[ic[ip + 1]] + ic[ic[ip + 2]];
            break;
        case 2:
            ic[ic[ip + 3]] = ic[ic[ip + 1]] * ic[ic[ip + 2]];
            break;
        case 99:
            return ic[0];
        default:
            errno = EINVAL;
            return -1;
        }
        ip += 4;
    }
}

static int part1(const char *input, size_t input_len)
{
    int *intcode;
    int int_count = input_to_intcode(input, input_len, &intcode);
    if (int_count < 0)
        goto err_cleanup;

    intcode[1] = 12;
    intcode[2] = 2;

    int rval = run_intcode(intcode, int_count);
    if (rval < 0)
        goto err_cleanup;
    free(intcode);
    return rval;

err_cleanup:
    if (intcode == NULL)
        free(intcode);
    return -errno;
}

static int part2(const char *input, size_t input_len)
{

    int *intcode;
    int int_count = input_to_intcode(input, input_len, &intcode);
    if (int_count < 0)
        goto err_cleanup;

    for (int i = 0; i < 100; i++)
    {
        for (int j = 0; j < 100; j++)
        {
            intcode[1] = i;
            intcode[2] = j;
            int rval = run_intcode(intcode, int_count);
            if (rval < 0)
                goto err_cleanup;
            if (rval == 19690720)
            {
                free(intcode);
                return 100 * i + j;
            }
        }
    }

err_cleanup:
    if (intcode == NULL)
        free(intcode);
    return -errno;
}

int day2(const char *filename)
{
    printf("Day 2: 1202 Program Alarm\n");

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
