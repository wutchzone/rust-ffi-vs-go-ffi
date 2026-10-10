#include <stdint.h>

#ifndef _VERTIGO_H_
#define _VERTIGO_H_

typedef int (*read_callback)(void *, uint8_t *, int);
typedef int (*write_callback)(void *, const uint8_t *, int);

int rust_transmuxer(read_callback read, write_callback write);

#endif
