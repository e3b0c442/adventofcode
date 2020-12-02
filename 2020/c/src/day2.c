#include <errno.h>
#include <pcre.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lib.h"

static pcre *pass_re;

static int part1(const char *input, size_t input_len)
{
    char **lines;
    int line_count = input_to_lines(input, input_len, &lines);
    if (line_count < 0)
        goto err_cleanup;

    const char *min_s, *max_s, *ch, *pw;

    int valid = 0;
    for (int i = 0; i < line_count; i++)
    {
        int ovector[15];
        int groups = pcre_exec(pass_re, NULL, lines[i], strlen(lines[i]), 0, 0, ovector, 15);
        if (groups < 5)
            goto err_cleanup;
        if (pcre_get_substring(lines[i], ovector, 5, 1, &min_s) < 1)
            goto err_cleanup;
        if (pcre_get_substring(lines[i], ovector, 5, 2, &max_s) < 1)
            goto err_cleanup;
        if (pcre_get_substring(lines[i], ovector, 5, 3, &ch) < 1)
            goto err_cleanup;
        if (pcre_get_substring(lines[i], ovector, 5, 4, &pw) < 1)
            goto err_cleanup;

        int min = atoi(min_s);
        if (min == 0 && min_s[0] != '0')
            goto err_cleanup;
        int max = atoi(max_s);
        if (max == 0 && max_s[0] != '0')
            goto err_cleanup;

        int count = 0;
        for (int j = 0; j < strlen(pw); j++)
        {
            if (pw[j] == ch[0])
                count++;
        }
        if (min <= count && count <= max)
            valid++;

        pcre_free_substring(min_s);
        pcre_free_substring(max_s);
        pcre_free_substring(ch);
        pcre_free_substring(pw);
    }

    free_lines(lines, line_count);
    return valid;

err_cleanup:
    free_lines(lines, line_count);
    if (min_s != NULL)
        pcre_free_substring(min_s);
    if (max_s != NULL)
        pcre_free_substring(max_s);
    if (ch != NULL)
        pcre_free_substring(ch);
    if (pw != NULL)
        pcre_free_substring(pw);
    if (errno == 0)
        errno = EINVAL;
    return -errno;
}

static int part2(const char *input, size_t input_len)
{
    char **lines;
    int line_count = input_to_lines(input, input_len, &lines);
    if (line_count < 0)
        goto err_cleanup;

    const char *l_s, *r_s, *ch, *pw;

    int valid = 0;
    for (int i = 0; i < line_count; i++)
    {
        int ovector[15];
        int groups = pcre_exec(pass_re, NULL, lines[i], strlen(lines[i]), 0, 0, ovector, 15);
        if (groups < 5)
            goto err_cleanup;
        if (pcre_get_substring(lines[i], ovector, 5, 1, &l_s) < 1)
            goto err_cleanup;
        if (pcre_get_substring(lines[i], ovector, 5, 2, &r_s) < 1)
            goto err_cleanup;
        if (pcre_get_substring(lines[i], ovector, 5, 3, &ch) < 1)
            goto err_cleanup;
        if (pcre_get_substring(lines[i], ovector, 5, 4, &pw) < 1)
            goto err_cleanup;

        int l = atoi(l_s) - 1;
        if (l < 0 && l_s[0] != '0')
            goto err_cleanup;

        int r = atoi(r_s) - 1;
        if (r < 0 && r_s[0] != '0')
            goto err_cleanup;
        char c = ch[0];

        if ((pw[l] == c || pw[r] == c) && pw[l] != pw[r])
            valid++;

        pcre_free_substring(l_s);
        pcre_free_substring(r_s);
        pcre_free_substring(ch);
        pcre_free_substring(pw);
    }

    free_lines(lines, line_count);
    return valid;

err_cleanup:
    free_lines(lines, line_count);
    if (l_s != NULL)
        pcre_free_substring(l_s);
    if (r_s != NULL)
        pcre_free_substring(r_s);
    if (ch != NULL)
        pcre_free_substring(ch);
    if (pw != NULL)
        pcre_free_substring(pw);
    if (errno == 0)
        errno = EINVAL;
    return -errno;
}

int day2(const char *filename)
{
    printf("Day 2: Password Philosophy\n");

    const char *err;
    int erroffset;
    pass_re = pcre_compile("(\\d+)-(\\d+) ([a-z]): (.*)", 0, &err, &erroffset, NULL);
    if (pass_re == NULL)
    {
        errno = EINVAL;
        goto err_cleanup;
    }

    char *input;
    int filesize = read_file_to_buffer(&input, filename);
    if (filesize < 0)
        goto err_cleanup;

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
    pcre_free(pass_re);
    return 0;

err_cleanup:
    if (pass_re != NULL)
        pcre_free(pass_re);
    if (input != NULL)
        free(input);
    return -errno;
}
