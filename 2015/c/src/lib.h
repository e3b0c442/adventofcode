#ifndef E3B0C442_ADVENT_LIB_H
#define E3B0C442_ADVENT_LIB_H

#define ESHT_MAX_FACTOR 0.75
#define ESHT_MIN_FACTOR 0.25

typedef struct esht esht;

int read_file_to_buffer(char **buf, const char *filename);
int input_to_lines(char ***dst, const char *src, size_t src_len);
void free_lines(char **lines, size_t line_count);
int cmp_int(const void *a, const void *b);

esht *esht_create();
void *esht_get(esht *table, const char *key, size_t *len);
int esht_keys(esht *table, char ***out);
int esht_update(esht *table, const char *key, void *value, size_t len);
int esht_remove(esht *table, const char *key);
void esht_destroy(esht *table);

#endif // E3B0C442_ADVENT_LIB_H
