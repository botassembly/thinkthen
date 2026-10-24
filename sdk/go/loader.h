#ifndef THINKTHEN_GO_LOADER_H
#define THINKTHEN_GO_LOADER_H
#include "thinkthen.h"
const char *tt_load(const char *path);
int32_t tt_client_open(const uint8_t *, size_t, thinkthen_client **, thinkthen_buffer *);
void tt_client_free(thinkthen_client *);
int32_t tt_call_open(const thinkthen_client *, const uint8_t *, size_t, uint32_t, thinkthen_call **, thinkthen_buffer *);
int32_t tt_send_text(thinkthen_call *, const uint8_t *, size_t, thinkthen_buffer *);
int32_t tt_send_json(thinkthen_call *, const uint8_t *, size_t, thinkthen_buffer *);
void tt_end_input(thinkthen_call *);
int32_t tt_next(thinkthen_call *, thinkthen_row *, thinkthen_buffer *);
void tt_cancel(thinkthen_call *);
void tt_free(thinkthen_call *);
void tt_buffer_free(thinkthen_buffer *);
void tt_row_free(thinkthen_row *);
#endif
