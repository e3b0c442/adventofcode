#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lib.h"

static int input_to_records(const char *input, size_t input_len, char ***output)
{
    //copy the input buffer so we don't mangle the original
    char input_cpy[input_len + 1];
    memcpy(input_cpy, input, input_len + 1);

    //get the number of records to appropriately size the array
    int record_count = 1;
    for (int i = 0; i < input_len - 1; i++)
        if (input_cpy[i] == '\n' && input_cpy[i + 1] == '\n')
            record_count++;

    //allocate the lines buffer
    char **records = calloc(record_count, sizeof(char *));
    if (records == NULL)
        goto err_cleanup;

    //tokenize the string manually because of the multichar delimiter
    int cursor = 0;
    int ix = 0;
    for (int i = 0; i < input_len; i++)
    {
        if (input_cpy[i] == '\n' && input_cpy[i + 1] == '\n')
        {
            input_cpy[i] = '\0';
            input_cpy[i + 1] = '\0';
            i++;
            char *record = calloc(i - cursor, sizeof(char));
            if (record == NULL)
                goto err_cleanup;
            memcpy(record, input_cpy + cursor, i - cursor);
            records[ix] = record;
            ix++;
            cursor = i + 1;
        }
    }
    char *record = calloc(input_len + 1 - cursor, sizeof(char));
    if (record == NULL)
        goto err_cleanup;
    memcpy(record, input_cpy + cursor, input_len + 1 - cursor);
    records[ix] = record;

    //replace remaining newlines with spaces
    for (int i = 0; i < record_count; i++)
        for (int j = 0; j < strlen(records[i]); j++)
            if (records[i][j] == '\n')
                records[i][j] = ' ';

    *output = records;
    return record_count;

err_cleanup:
    free_lines(records, record_count);
    return -errno;
}

static esht *record_to_passport(const char *input)
{
    //copy the input buffer so we don't mangle the original
    char input_cpy[strlen(input) + 1];
    memcpy(input_cpy, input, strlen(input) + 1);

    //count the fields
    int fields_count = 1;
    for (int i = 0; i < strlen(input); i++)
        if (input_cpy[i] == ' ')
            fields_count++;

    //create a fields buffer
    char *fields[fields_count];
    char *field = strtok(input_cpy, " ");
    int i = 0;
    while (field)
    {
        fields[i] = field;
        i++;
        field = strtok(NULL, " ");
    }

    //parse add the fields to the hash table
    esht *passport = esht_create();
    for (int i = 0; i < fields_count; i++)
    {
        char *key = strtok(fields[i], ":");
        char *val = strtok(NULL, ":");

        int r = esht_update(passport, key, val, strlen(val) + 1);
        if (r < 0)
        {
            esht_destroy(passport);
            return NULL;
        }
    }

    return passport;
}

static int part1(const char *input, size_t input_len)
{
    char **records;
    int records_count = input_to_records(input, input_len, &records);
    if (records_count < 0)
        goto err_cleanup;

    char *fields[] = {"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"};
    const int fields_count = 7;
    int valid = 0;
    for (int i = 0; i < records_count; i++)
    {
        esht *passport = record_to_passport(records[i]);
        for (int j = 0; j < fields_count; j++)
        {
            char *field = esht_get(passport, fields[j], NULL);
            if (field == NULL)
                goto cleanup;
            free(field);
        }
        valid++;
    cleanup:
        esht_destroy(passport);
    }

    free_lines(records, records_count);
    return valid;

err_cleanup:
    free_lines(records, records_count);
    return -errno;
}

static int part2(const char *input, size_t input_len)
{
    char **records;
    int records_count = input_to_records(input, input_len, &records);
    if (records_count < 0)
        goto err_cleanup;

    int valid = 0;
    char *field = NULL;
    int parsed;
    char *colors[] = {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"};
    int colors_len = 7;
    for (int i = 0; i < records_count; i++)
    {
        esht *passport = record_to_passport(records[i]);
        field = esht_get(passport, "byr", NULL);
        if (field == NULL)
            goto cleanup;
        if (strlen(field) != 4)
            goto cleanup;
        parsed = atoi(field);
        if (parsed < 1920 || parsed > 2002)
            goto cleanup;

        free(field);
        field = esht_get(passport, "iyr", NULL);
        if (field == NULL)
            goto cleanup;
        if (strlen(field) != 4)
            goto cleanup;
        parsed = atoi(field);
        if (parsed < 2010 || parsed > 2020)
            goto cleanup;

        free(field);
        field = esht_get(passport, "eyr", NULL);
        if (field == NULL)
            goto cleanup;
        if (strlen(field) != 4)
            goto cleanup;
        parsed = atoi(field);
        if (parsed < 2020 || parsed > 2030)
            goto cleanup;

        free(field);
        field = esht_get(passport, "hgt", NULL);
        if (field == NULL)
            goto cleanup;
        if (strlen(field) < 3 || strlen(field) > 64)
            goto cleanup;

        char *unit = field + (strlen(field) - 2);
        char val[63] = {0};
        strncpy(val, field, strlen(field) - 2);
        parsed = atoi(val);
        if (!strcmp(unit, "cm"))
        {
            if (parsed < 150 || parsed > 193)
                goto cleanup;
        }
        else if (!strcmp(unit, "in"))
        {
            if (parsed < 59 || parsed > 76)
                goto cleanup;
        }
        else
            goto cleanup;

        free(field);
        field = esht_get(passport, "hcl", NULL);
        if (field == NULL)
            goto cleanup;
        if (strlen(field) != 7)
            goto cleanup;
        if (field[0] != '#')
            goto cleanup;
        parsed = (int)strtol(field + 1, NULL, 16);
        if (parsed == 0 && strcmp("000000", field + 1) != 0)
            goto cleanup;

        free(field);
        field = esht_get(passport, "ecl", NULL);
        if (field == NULL)
            goto cleanup;
        parsed = 0;
        for (int j = 0; j < colors_len; j++)
            if (!strcmp(colors[j], field))
                parsed = 1;
        if (!parsed)
            goto cleanup;

        free(field);
        field = esht_get(passport, "pid", NULL);
        if (field == NULL)
            goto cleanup;
        if (strlen(field) != 9)
            goto cleanup;
        parsed = atoi(field);
        if (parsed == 0)
            goto cleanup;
        valid++;

    cleanup:
        if (field != NULL)
            free(field);
        esht_destroy(passport);
    }

    free_lines(records, records_count);
    return valid;

err_cleanup:
    free_lines(records, records_count);
    return -errno;
}

int day4(const char *filename)
{
    printf("Day 4: Passport Processing\n");

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
