#include <stdint.h>

#ifndef _VERTIGO_H_
#define _VERTIGO_H_

typedef int (*callback)(void *, void *, int);

int rust_transmuxer(callback read, callback write);

#endif
