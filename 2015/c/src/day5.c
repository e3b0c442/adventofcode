#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pcre.h>
#include "lib.h"

static int part1(const char *input, size_t input_len)
{
    const char *error;
    int erroffset;
    int good = 0;

    pcre *nice_re = pcre_compile("^(?!.*(?:ab|cd|pq|xy))(?=.*(.)\\1)(?=(?:.*[aeiou]){3,}).*$", 0, &error, &erroffset, NULL);
    if (nice_re == NULL)
    {
        errno = ENOTRECOVERABLE;
        goto err_cleanup;
    }

    char **lines;
    size_t lines_len = input_to_lines(&lines, input, input_len);
    if (lines_len < 0)
        goto err_cleanup;

    for (int i = 0; i < lines_len; i++)
    {
        if (!pcre_exec(nice_re, NULL, lines[i], strlen(lines[i]), 0, 0, NULL, 0))
            good++;
    }

    goto cleanup;

err_cleanup:
    good = -1;
cleanup:
    if (nice_re != NULL)
        pcre_free(nice_re);
    if (lines != NULL)
        free_lines(lines, lines_len);
    return good;
}

static int part2(const char *input, size_t input_len)
{
    const char *error;
    int erroffset;
    int good = 0;

    pcre *nice_re = pcre_compile("^(?=.*(..).*\\1)(?=.*(.).\\2).*$", 0, &error, &erroffset, NULL);
    if (nice_re == NULL)
    {
        errno = ENOTRECOVERABLE;
        goto err_cleanup;
    }

    char **lines;
    size_t lines_len = input_to_lines(&lines, input, input_len);
    if (lines_len < 0)
        goto err_cleanup;

    for (int i = 0; i < lines_len; i++)
    {
        if (!pcre_exec(nice_re, NULL, lines[i], strlen(lines[i]), 0, 0, NULL, 0))
            good++;
    }

    goto cleanup;

err_cleanup:
    good = -1;
cleanup:
    if (nice_re != NULL)
        pcre_free(nice_re);
    if (lines != NULL)
        free_lines(lines, lines_len);
    return good;
}

int day5(const char *filename)
{
    printf("Day 5: Doesn't He Have Intern-Elves For This?\n");

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
