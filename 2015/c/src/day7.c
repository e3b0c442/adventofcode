#include <errno.h>
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <pcre.h>
#include "lib.h"

static pcre *gates_re;
static pcre *gate_re;

static uint16_t gate_exec(esht *gates, esht *cache, const char *key)
{
    int rval = 0;
    char *gate = NULL;
    const char *ls = NULL;
    const char *c = NULL;
    const char *rs = NULL;
    uint16_t r = 0, l = 0;

    uint16_t *cached = esht_get(cache, key, NULL);
    if (cached != NULL)
    {
        rval = *cached;
        free(cached);
        goto cleanup;
    }

    gate = esht_get(gates, key, NULL);
    if (gate == NULL)
        goto err_cleanup;

    int ovector[12];
    int matches = pcre_exec(gate_re, NULL, gate, strlen(gate), 0, 0, ovector, 12);
    if (matches < 4)
        goto err_cleanup;

    if (pcre_get_substring(gate, ovector, matches, 3, &rs) < 1)
        goto err_cleanup;
    r = (uint16_t)atoi(rs);
    if (r == 0 && rs[0] != '0')
        r = gate_exec(gates, cache, rs);

    if (pcre_get_substring(gate, ovector, matches, 2, &c) < 0)
        goto err_cleanup;

    if (strlen(c) == 0)
    {
        rval = r;
        goto set_cache;
    }

    if (pcre_get_substring(gate, ovector, matches, 1, &ls) < 0)
        goto err_cleanup;

    if (strlen(ls) > 0)
    {
        l = (uint16_t)atoi(ls);
        if (l == 0 && ls[0] != '0')
            l = gate_exec(gates, cache, ls);
    }

    if (!strcmp(c, "AND"))
        rval = l & r;
    else if (!strcmp(c, "OR"))
        rval = l | r;
    else if (!strcmp(c, "NOT"))
        rval = ~r;
    else if (!strcmp(c, "LSHIFT"))
        rval = l << r;
    else if (!strcmp(c, "RSHIFT"))
        rval = l >> r;
    else
        goto err_cleanup;

set_cache:
    esht_update(cache, key, &rval, sizeof(uint16_t));
    goto cleanup;
err_cleanup:
    rval = 0;
    errno = EBADMSG;
cleanup:
    if (gate != NULL)
        free(gate);
    if (ls != NULL)
        pcre_free_substring(ls);
    if (c != NULL)
        pcre_free_substring(c);
    if (rs != NULL)
        pcre_free_substring(rs);
    return rval;
}

int lines_to_gate_map(esht *gates, char **lines, size_t lines_len)
{
    int rval = 0;
    const char *key = NULL;
    const char *gate = NULL;
    for (int i = 0; i < lines_len; i++)
    {
        int ovector[9];
        int matches = pcre_exec(gates_re, NULL, lines[i], strlen(lines[i]), 0, 0, ovector, 9);
        if (matches < 3)
            goto err_cleanup;
        const char *key = NULL;
        const char *gate = NULL;
        if (pcre_get_substring(lines[i], ovector, matches, 1, &gate) < 1)
            goto err_cleanup;
        if (pcre_get_substring(lines[i], ovector, matches, 2, &key) < 1)
            goto err_cleanup;

        if (esht_update(gates, key, (char *)gate, strlen(gate) + 1))
            goto err_cleanup;

        pcre_free_substring(key);
        pcre_free_substring(gate);
    }
    goto cleanup;

err_cleanup:
    rval = -1;
cleanup:
    if (key != NULL)
        pcre_free_substring(key);
    if (gate != NULL)
        pcre_free_substring(gate);
    return rval;
}

static int part1(const char *input, size_t input_len)
{
    int rval = 0;
    char **lines = NULL;
    esht *gates = NULL, *cache = NULL;

    int lines_len = input_to_lines(&lines, input, input_len);
    if (lines_len < 1)
        goto err_cleanup;

    gates = esht_create();
    if (gates == NULL)
        goto err_cleanup;

    if (lines_to_gate_map(gates, lines, lines_len))
        goto err_cleanup;

    cache = esht_create();
    rval = (int)gate_exec(gates, cache, "a");
    goto cleanup;

err_cleanup:
    rval = -1;
cleanup:
    if (lines != NULL)
        free_lines(lines, lines_len);
    if (gates != NULL)
        esht_destroy(gates);
    if (cache != NULL)
        esht_destroy(cache);
    return rval;
}

static int part2(const char *input, size_t input_len, int prev)
{
    int rval = 0;
    char **lines = NULL;
    esht *gates = NULL, *cache = NULL;

    int lines_len = input_to_lines(&lines, input, input_len);
    if (lines_len < 1)
        goto err_cleanup;

    gates = esht_create();
    if (gates == NULL)
        goto err_cleanup;

    if (lines_to_gate_map(gates, lines, lines_len))
        goto err_cleanup;

    cache = esht_create();
    if (esht_update(cache, "b", (uint16_t *)&prev, sizeof(uint16_t)))
        goto err_cleanup;
    rval = (int)gate_exec(gates, cache, "a");
    goto cleanup;

err_cleanup:
    rval = -1;
cleanup:
    if (lines != NULL)
        free_lines(lines, lines_len);
    if (gates != NULL)
        esht_destroy(gates);
    if (cache != NULL)
        esht_destroy(cache);
    return rval;
}

int day7(const char *filename)
{
    printf("Day 6: Probably a Fire Hazard\n");
    int rc = 0;

    const char *error;
    int erroffset;
    char *input = NULL;

    gates_re = pcre_compile("(.*) -> ([a-z]+)", 0, &error, &erroffset, NULL);
    if (gates_re == NULL)
    {
        errno = ENOTRECOVERABLE;
        goto err_cleanup;
    }
    gate_re = pcre_compile("(?:([a-z]+|[0-9]+) )?(?:(AND|OR|NOT|RSHIFT|LSHIFT) )?([a-z]+|[0-9]+)", 0, &error, &erroffset, NULL);
    if (gate_re == NULL)
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

    rval = part2(input, filesize, rval);
    if (rval < 0)
        goto err_cleanup;
    printf("\tPart 2: %d\n", rval);

    goto cleanup;

err_cleanup:
    rval = -1;
cleanup:
    if (gates_re != NULL)
        pcre_free(gates_re);
    if (gate_re != NULL)
        pcre_free(gate_re);
    if (input != NULL)
        free(input);

    return rc;
}
