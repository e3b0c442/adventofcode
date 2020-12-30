#ifndef E3B0C442_ADVENT_LIB_H
#define E3B0C442_ADVENT_LIB_H

int read_file_to_buffer(char **buf, const char *filename);
int input_to_lines(char ***dst, const char *src, size_t src_len);
void free_lines(char **lines, size_t line_count);
int cmp_int(const void *a, const void *b);

#endif // E3B0C442_ADVENT_LIB_H
