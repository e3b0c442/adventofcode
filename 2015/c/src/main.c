#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "days.h"

#ifdef DAYNUM
int main(int argc, char const *argv[])
{
    if (argc < 2)
    {
        fprintf(stderr, "%s\n", strerror(EINVAL));
        return EINVAL;
    }

    int rval = DAYNUM(argv[1]);
    if (rval)
    {
        fprintf(stderr, "%s\n", strerror(rval));
        return rval;
    }

    return 0;
}

#else  // #ifdef DAYNUM
typedef int (*day_f)(const char *);

static const int days_len = 2;
static const day_f days[] = {
    day1,
    day2};

int main(int argc, char const *argv[])
{
    if (argc < 2)
    {
        fprintf(stderr, "%s\n", strerror(EINVAL));
        return EINVAL;
    }

    char *path = calloc(FILENAME_MAX, 1);
    if (path == NULL)
    {
        fprintf(stderr, "%s\n", strerror(errno));
        return errno;
    }

    for (int i = 0; i < days_len; i++)
    {
        sprintf(path, "%s/%d.txt", argv[1], i + 1);
        int rval = days[i](path);
        if (rval)
        {
            fprintf(stderr, "%s\n", strerror(rval));
            free(path);
            return rval;
        }
    }

    free(path);
    return 0;
}
#endif // #ifdef DAYNUM
