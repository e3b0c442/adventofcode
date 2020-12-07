#ifndef LIB_H
#define LIB_H

#include <stddef.h>

#define ESHT_MAX_FACTOR 0.75
#define ESHT_MIN_FACTOR 0.25

typedef struct esht esht;

void free_lines(char **lines, size_t line_count);
int input_to_lines(const char *input, size_t input_len, char ***output);
int input_to_records(const char *input, size_t input_len, char ***output);
int read_file_to_buffer(char **buf, const char *filename);
esht *esht_create();
void *esht_get(esht *table, char *key, size_t *len);
int esht_keys(esht *table, char ***out);
int esht_update(esht *table, char *key, void *value, size_t len);
int esht_remove(esht *table, char *key);
void esht_destroy(esht *table);

#endif // LIB_H
