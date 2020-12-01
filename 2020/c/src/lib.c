#include <stdio.h>
#include <stdlib.h>
#include "lib.h"

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
