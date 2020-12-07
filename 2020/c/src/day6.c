#include <errno.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lib.h"

static int part1(const char *input, size_t input_len)
{
    char **groups = NULL, **lines = NULL, **keys = NULL;
    esht *map = NULL;
    int line_count = 0, key_count = 0;
    int groups_count = input_to_records(input, input_len, &groups);

    if (groups_count < 0)
        goto err_cleanup;

    int sum = 0;
    for (int i = 0; i < groups_count; i++)
    {
        line_count = input_to_lines(groups[i], strlen(groups[i]), &lines);
        if (line_count < 0)
            goto err_cleanup;

        map = esht_create();

        for (int j = 0; j < line_count; j++)
        {
            for (int k = 0; k < strlen(lines[j]); k++)
            {
                char key[2] = {lines[j][k], '\0'};
                bool val = true;
                int rval = esht_update(map, key, &val, sizeof(bool));
                if (rval < 0)
                    goto err_cleanup;
            }
        }
        key_count = esht_keys(map, &keys);
        sum += key_count;
        for (int j = 0; j < key_count; j++)
            free(keys[j]);
        free(keys);
        free_lines(lines, line_count);
        esht_destroy(map);
        map = NULL;
        keys = NULL;
    }
    free_lines(groups, groups_count);
    return sum;

err_cleanup:
    free_lines(groups, groups_count);
    free_lines(lines, line_count);
    if (map != NULL)
        esht_destroy(map);
    if (keys != NULL)
    {
        for (int i = 0; i < key_count; i++)
            if (keys[i] != NULL)
                free(keys[i]);
        free(keys);
    }

    return -errno;
}

static int part2(const char *input, size_t input_len)
{
    char **groups = NULL, **lines = NULL, **keys = NULL;
    esht *map = NULL;
    int *val = NULL;

    int line_count = 0, key_count = 0;
    int groups_count = input_to_records(input, input_len, &groups);

    if (groups_count < 0)
        goto err_cleanup;

    int sum = 0;
    for (int i = 0; i < groups_count; i++)
    {
        line_count = input_to_lines(groups[i], strlen(groups[i]), &lines);
        if (line_count < 0)
            goto err_cleanup;

        map = esht_create();
        if (map == NULL)
            goto err_cleanup;

        for (int j = 0; j < line_count; j++)
        {
            for (int k = 0; k < strlen(lines[j]); k++)
            {
                char key[2] = {lines[j][k], '\0'};
                int *v = esht_get(map, key, NULL);
                int val;
                if (v == NULL)
                    val = 1;
                else
                    val = *v + 1;
                free(v);

                int rval = esht_update(map, key, &val, sizeof(bool));
                if (rval < 0)
                    goto err_cleanup;
            }
        }
        key_count = esht_keys(map, &keys);
        for (int j = 0; j < key_count; j++)
        {
            val = esht_get(map, keys[j], NULL);
            if (val == NULL)
                goto err_cleanup;
            if (*val == line_count)
                sum++;
            free(val);
        }
        for (int j = 0; j < key_count; j++)
            free(keys[j]);
        free(keys);
        free_lines(lines, line_count);
        esht_destroy(map);
        map = NULL;
        keys = NULL;
    }
    free_lines(groups, groups_count);
    return sum;

err_cleanup:
    free_lines(groups, groups_count);
    free_lines(lines, line_count);
    if (val != NULL)
        free(val);
    if (map != NULL)
        esht_destroy(map);
    if (keys != NULL)
    {
        for (int i = 0; i < key_count; i++)
            if (keys[i] != NULL)
                free(keys[i]);
        free(keys);
    }

    return -errno;
}

int day6(const char *filename)
{
    printf("Day 6: Custom Customs\n");

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
