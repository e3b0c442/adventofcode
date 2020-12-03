void free_lines(char **lines, size_t line_count);
int input_to_lines(const char *input, size_t input_len, char ***output);
int read_file_to_buffer(char **buf, const char *filename);
