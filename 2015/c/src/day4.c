#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <openssl/md5.h>
#include "lib.h"

#define MAX_INT_CHARS 10
static int part1(const char *input, size_t input_len)
{
    char key[input_len + MAX_INT_CHARS];
    size_t key_len = strlen(input);
    strcpy(key, input);

    int i = 1;
    while (true)
    {
        memset(key + key_len, 0, input_len + MAX_INT_CHARS - key_len);
        sprintf(key + key_len, "%d", i);
        unsigned char *sum = MD5((unsigned char *)key, strlen(key), NULL);
        if (sum[0] == 0 && sum[1] == 0 && (sum[2] & 0xf0) == 0)
            return i;

        i++;
    }
}

static int part2(const char *input, size_t input_len, int start)
{
    char key[input_len + MAX_INT_CHARS];
    size_t key_len = strlen(input);
    strcpy(key, input);

    int i = start;
    while (true)
    {
        memset(key + key_len, 0, input_len + MAX_INT_CHARS - key_len);
        sprintf(key + key_len, "%d", i);
        unsigned char *sum = MD5((unsigned char *)key, strlen(key), NULL);
        if (sum[0] == 0 && sum[1] == 0 && sum[2] == 0)
            return i;

        i++;
    }
}

int day4(const char *filename)
{
    printf("Day 4: The Ideal Stocking Stuffer\n");

    char *input;
    int filesize = read_file_to_buffer(&input, filename);
    if (filesize < 0)
        return -1;

    int rval = part1(input, filesize);
    if (rval < 0)
        return rval;
    printf("\tPart 1: %d\n", rval);

    rval = part2(input, filesize, rval);
    if (rval < 0)
        return rval;
    printf("\tPart 2: %d\n", rval);

    free(input);

    return 0;
}
