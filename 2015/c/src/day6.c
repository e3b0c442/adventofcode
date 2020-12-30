#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pcre.h>
#include "lib.h"

static pcre *instr_re;

typedef enum
{
    turn_on,
    turn_off,
    toggle
} command;

typedef struct
{
    command cmd;
    int ox;
    int oy;
    int dx;
    int dy;
} instruction;

static int parse_instr(instruction *instr, char *line)
{
    int rc = 0;
    int ovector[18];
    const char *cmd = NULL;
    const char *oxs = NULL;
    const char *oys = NULL;
    const char *dxs = NULL;
    const char *dys = NULL;

    int matches = pcre_exec(instr_re, NULL, line, strlen(line), 0, 0, ovector, 18);
    if (matches < 6)
        goto err_cleanup;
    if (pcre_get_substring(line, ovector, matches, 1, &cmd) < 1)
        goto err_cleanup;
    if (pcre_get_substring(line, ovector, matches, 2, &oxs) < 1)
        goto err_cleanup;
    if (pcre_get_substring(line, ovector, matches, 3, &oys) < 1)
        goto err_cleanup;
    if (pcre_get_substring(line, ovector, matches, 4, &dxs) < 1)
        goto err_cleanup;
    if (pcre_get_substring(line, ovector, matches, 5, &dys) < 1)
        goto err_cleanup;

    int ox = atoi(oxs), oy = atoi(oys), dx = atoi(dxs), dy = atoi(dys);
    if ((ox == 0 && oxs[0] != '0') || (oy == 0 && oys[0] != '0') || (dx == 0 && dxs[0] != '0') || (dy == 0 && dys[0] != '0'))
        goto err_cleanup;

    if (!strcmp("turn on", cmd))
        instr->cmd = turn_on;
    else if (!strcmp("turn off", cmd))
        instr->cmd = turn_off;
    else if (!strcmp("toggle", cmd))
        instr->cmd = toggle;
    else
        goto err_cleanup;

    instr->ox = ox;
    instr->oy = oy;
    instr->dx = dx;
    instr->dy = dy;
    goto cleanup;

err_cleanup:
    errno = EBADMSG;
    rc = -1;
cleanup:
    if (cmd != NULL)
        pcre_free_substring(cmd);
    if (oxs != NULL)
        pcre_free_substring(oxs);
    if (oys != NULL)
        pcre_free_substring(oys);
    if (dxs != NULL)
        pcre_free_substring(dxs);
    if (dys != NULL)
        pcre_free_substring(dys);
    return rc;
}

static int part1(const char *input, size_t input_len)
{
    int lit = 0;

    char **lines;
    int lines_len = input_to_lines(&lines, input, input_len);
    if (lines_len < 0)
        goto err_cleanup;

    char grid[1000][1000];
    memset(grid, 0, 1000 * 1000 * sizeof(char));
    for (int i = 0; i < lines_len; i++)
    {
        instruction instr;
        if (parse_instr(&instr, lines[i]) < 0)
            goto err_cleanup;

        for (int x = instr.ox; x <= instr.dx; x++)
        {
            for (int y = instr.oy; y <= instr.dy; y++)
            {
                switch (instr.cmd)
                {
                case turn_on:
                    grid[x][y] = 1;
                    break;
                case turn_off:
                    grid[x][y] = 0;
                    break;
                case toggle:
                    grid[x][y] ^= 1;
                    break;
                };
            }
        }
    }

    for (int i = 0; i < 1000; i++)
        for (int j = 0; j < 1000; j++)
            lit += grid[i][j];

    goto cleanup;

err_cleanup:
    lit = -1;
cleanup:
    if (lines != NULL)
        free_lines(lines, lines_len);
    return lit;
}

static int part2(const char *input, size_t input_len)
{
    int lit = 0;

    char **lines;
    int lines_len = input_to_lines(&lines, input, input_len);
    if (lines_len < 0)
        goto err_cleanup;

    char grid[1000][1000];
    memset(grid, 0, 1000 * 1000 * sizeof(char));
    for (int i = 0; i < lines_len; i++)
    {
        instruction instr;
        if (parse_instr(&instr, lines[i]) < 0)
            goto err_cleanup;

        for (int x = instr.ox; x <= instr.dx; x++)
        {
            for (int y = instr.oy; y <= instr.dy; y++)
            {
                switch (instr.cmd)
                {
                case turn_on:
                    grid[x][y] += 1;
                    break;
                case turn_off:
                    if (grid[x][y] > 0)
                        grid[x][y] -= 1;
                    break;
                case toggle:
                    grid[x][y] += 2;
                    break;
                };
            }
        }
    }

    for (int i = 0; i < 1000; i++)
        for (int j = 0; j < 1000; j++)
            lit += grid[i][j];

    goto cleanup;

err_cleanup:
    lit = -1;
cleanup:
    if (lines != NULL)
        free_lines(lines, lines_len);
    return lit;
}

int day6(const char *filename)
{
    printf("Day 6: Probably a Fire Hazard\n");
    int rc = 0;

    const char *error;
    int erroffset;
    char *input = NULL;
    instr_re = pcre_compile("(turn off|turn on|toggle) (\\d{1,3}),(\\d{1,3}) through (\\d{1,3}),(\\d{1,3})", 0, &error, &erroffset, NULL);
    if (instr_re == NULL)
    {
        errno = ENOTRECOVERABLE;
        goto err_cleanup;
    }

    int filesize = read_file_to_buffer(&input, filename);
    if (filesize < 0)
        goto err_cleanup;

    int rval = part1(input, filesize);
    if (rval < 0)
        goto err_cleanup;
    printf("\tPart 1: %d\n", rval);

    rval = part2(input, filesize);
    if (rval < 0)
        goto err_cleanup;
    printf("\tPart 2: %d\n", rval);

    goto cleanup;

err_cleanup:
    rval = -1;
cleanup:
    if (instr_re != NULL)
        pcre_free(instr_re);
    if (input != NULL)
        free(input);

    return rc;
}
