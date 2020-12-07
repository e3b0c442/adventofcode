#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lib.h"

void free_lines(char **lines, size_t line_count)
{
    if (lines != NULL)
    {
        for (int i = 0; i < line_count; i++)
            if (lines[i] != NULL)
                free(lines[i]);
        free(lines);
    }
}

int input_to_lines(const char *input, size_t input_len, char ***output)
{
    //copy the input buffer so we don't mangle the original
    char input_cpy[input_len + 1];
    memcpy(input_cpy, input, input_len + 1);

    //get the number of lines to appropriately size the array
    int line_count = 1;
    for (int i = 0; i < input_len; i++)
        if (input_cpy[i] == '\n')
            line_count++;

    //allocate the lines buffer
    char **lines = calloc(line_count, sizeof(char *));
    if (lines == NULL)
        goto err_cleanup;

    //tokenize the string, allocate each line and add it to the lines array
    char *input_line = strtok(input_cpy, "\n");
    int i = 0;
    while (input_line)
    {
        char *line = calloc(strlen(input_line) + 1, sizeof(char));
        if (line == NULL)
            goto err_cleanup;
        memcpy(line, input_line, strlen(input_line) + 1);
        lines[i] = line;
        i++;
        input_line = strtok(NULL, "\n");
    }
    *output = lines;
    return line_count;

err_cleanup:
    free_lines(lines, line_count);
    return -errno;
}

int input_to_records(const char *input, size_t input_len, char ***output)
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

    *output = records;
    return record_count;

err_cleanup:
    free_lines(records, record_count);
    return -errno;
}

int read_file_to_buffer(char **buf, const char *filename)
{
    *buf = NULL;
    int rval;

    // open the file
    FILE *f = fopen(filename, "r");
    if (f == NULL)
        goto err_cleanup; // errno is set by fopen()

    // get the length
    if (fseek(f, 0L, SEEK_END))
        goto err_cleanup; // errno is set by fseek()
    long filesize = ftell(f);
    if (filesize < 0)
        goto err_cleanup; // errno is set by ftell()
    rewind(f);

    // allocate the buffer
    *buf = calloc(filesize + 1, 1);
    if (*buf == NULL)
        goto err_cleanup; /* errno is set by malloc() */

    /* read the file into the buffer */
    size_t rd = fread(*buf, 1, filesize, f);
    if (rd < filesize)
        if (ferror(f))
            goto err_cleanup;

    /* file is read into the buffer, return the number of bytes read */
    rval = rd;
    goto cleanup;

err_cleanup:
    rval = -1;
    if (*buf != NULL)
        free(*buf);
    *buf = NULL;
cleanup:
    if (f != NULL)
        fclose(f);
    return rval;
}

typedef struct esht_entry
{
    char *k;
    void *v;
    size_t l;
    struct esht_entry *n;
} esht_entry;

struct esht
{
    size_t len;
    size_t cap;
    esht_entry **entries;
};

unsigned long esht_hash(char *str)
{
    unsigned long hash = 5381;
    int c = 0;

    while ((c = *str++))
        hash = ((hash << 5) + hash) ^ c;

    return hash;
}

esht *esht_create()
{
    esht *table = NULL;

    table = calloc(1, sizeof(esht));
    if (table == NULL)
        return NULL;

    table->cap = 1;
    table->len = 0;
    table->entries = calloc(1, sizeof(esht_entry *));
    if (table->entries == NULL)
    {
        free(table);
        return NULL;
    }

    return table;
}

static int esht_resize(esht *table, size_t new_cap)
{
    int i = 0, ii = 0;
    esht_entry *cur = NULL, *next = NULL, **old = NULL;

    esht_entry **new_entries = calloc(new_cap, sizeof(esht_entry *));
    if (new_entries == NULL)
        return 1;

    for (i = 0; i < table->cap; i++)
    {
        next = table->entries[i];
        while (next != NULL)
        {
            cur = next;
            next = cur->n;

            ii = esht_hash(cur->k) % new_cap;
            cur->n = new_entries[ii];
            new_entries[ii] = cur;
        }
    }

    old = table->entries;
    table->entries = new_entries;
    table->cap = new_cap;
    free(old);

    return 0;
}

int esht_keys(esht *table, char ***output)
{
    int key_count = 0;
    for (int i = 0; i < table->cap; i++)
    {
        esht_entry *entry = table->entries[i];
        while (entry != NULL)
        {
            key_count++;
            entry = entry->n;
        }
    }
    if (key_count == 0)
    {
        *output = NULL;
        return 0;
    }
    char **keys = calloc(key_count, sizeof(char *));
    if (keys == NULL)
        return -1;

    int key_i = 0;
    for (int i = 0; i < table->cap; i++)
    {
        esht_entry *entry = table->entries[i];
        while (entry != NULL)
        {
            char *key = calloc(strlen(entry->k) + 1, sizeof(char));
            if (key == NULL)
            {
                for (int ii = 0; ii < key_i; ii++)
                    free(keys[ii]);
                free(keys);
                return -1;
            }
            strncpy(key, entry->k, strlen(entry->k));
            keys[key_i] = key;
            key_i++;

            entry = entry->n;
        }
    }
    *output = keys;
    return key_count;
}

esht_entry *esht_get_entry(esht *table, char *key)
{
    unsigned long i = 0;
    esht_entry *e = NULL;

    i = esht_hash(key) % table->cap;
    e = table->entries[i];
    while (e != NULL)
    {
        if (!strcmp(key, e->k))
            return e;
        e = e->n;
    }

    return NULL;
}

void *esht_get(esht *table, char *key, size_t *len)
{
    if (table == NULL)
        return NULL;
    esht_entry *e = NULL;
    void *r = NULL;

    e = esht_get_entry(table, key);
    if (e == NULL)
    {
        if (len != NULL)
            *len = 0;
        return NULL;
    }

    r = calloc(e->l, 1);
    if (r == NULL)
    {
        if (len != NULL)
            *len = 0;
        return NULL;
    }
    memcpy(r, e->v, e->l);

    if (len != NULL)
        *len = e->l;
    return r;
}

int esht_update(esht *table, char *key, void *value, size_t len)
{
    unsigned long i = 0;
    esht_entry *e = NULL;
    void *v = NULL, *k = NULL, *old = NULL;

    e = esht_get_entry(table, key);
    if (e != NULL)
    {
        v = calloc(len, 1);
        if (v == NULL)
            return 1;
        memcpy(v, value, len);

        old = e->v;
        e->l = len;
        e->v = v;
        free(old);

        return 0;
    }

    e = calloc(1, sizeof(esht_entry));
    if (e == NULL)
        return 1;

    v = calloc(len, 1);
    if (v == NULL)
    {
        free(e);
        return 1;
    }
    memcpy(v, value, len);

    k = calloc(strlen(key) + 1, 1);
    if (k == NULL)
    {
        free(e);
        free(v);
        return 1;
    }
    strcpy(k, key);

    e->k = k;
    e->v = v;
    e->l = len;

    i = esht_hash(key) % table->cap;
    e->n = table->entries[i];
    table->entries[i] = e;
    table->len++;

    if ((float)table->len / (float)table->cap > ESHT_MAX_FACTOR)
        if (esht_resize(table, table->cap * 2))
            return 1;

    return 0;
}

int esht_remove(esht *table, char *key)
{
    unsigned long i = 0;
    esht_entry *e = NULL;

    i = esht_hash(key) % table->cap;
    e = table->entries[i];
    if (e == NULL)
        return 1;

    if (!strcmp(e->k, key))
    {
        table->entries[i] = e->n;
        goto cleanup;
    }

    while (e->n != NULL)
    {
        if (!strcmp(e->n->k, key))
        {
            e->n = e->n->n;
            goto cleanup;
        }
    }

    return 1;

cleanup:
    free(e->k);
    free(e->v);
    free(e);

    table->len--;
    if ((float)table->len / (float)table->cap < ESHT_MIN_FACTOR)
        if (esht_resize(table, table->cap / 2))
            return 1;

    return 0;
}

void esht_destroy(esht *table)
{
    int i = 0;
    esht_entry *e = NULL, *n = NULL;

    for (i = 0; i < table->cap; i++)
    {
        e = table->entries[i];
        while (e != NULL)
        {
            n = e->n;
            free(e->k);
            free(e->v);
            free(e);
            e = n;
        }
    }

    free(table->entries);
    free(table);
}
