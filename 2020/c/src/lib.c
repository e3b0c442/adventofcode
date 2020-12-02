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
    char *next = NULL;
    int i = 0;
    while (input_line)
    {
        next = input_line + strlen(input_line) + 1;
        char *line = calloc(strlen(input_line) + 1, sizeof(char));
        if (line == NULL)
            goto err_cleanup;
        memcpy(line, input_line, strlen(input_line) + 1);
        lines[i] = line;
        i++;
        input_line = strtok(next, "\n");
    }
    *output = lines;
    return line_count;

err_cleanup:
    free_lines(lines, line_count);
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
