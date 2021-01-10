#include <stdint.h>

#ifndef _VERTIGO_H_
#define _VERTIGO_H_

extern int go_read(void *, void *, int);

extern int go_write(void *, void *, int);

int go_transmuxer(void *go_struct);

#endif
