#pragma once

#include <stdlib.h>
#include <stdbool.h>

#define DEFAULT_CAPACITY 5

struct Vector;
typedef struct Vector * vec_t;

/* new_vec() construct a vector with default
 * capacity
 */
vec_t new_vec(size_t entry_size);

/* vec_with_capacity() construct a vector with given
 * capacity
 */
vec_t vec_with_capacity(size_t capa, size_t entry_size);

/* push() push an item into end of the vector */
bool push(vec_t vec, void *item);

/* pop() pop an item from end of the vector
 * and save the item into ret
 */
bool pop(vec_t vec, void *ret);

/* len() return the lenght of vector */
size_t len(vec_t vec);

/* into_array() consume (distruct) a vector and generate
 * an array*/
void *into_array(vec_t vec, size_t *len);

/* free_vec() distruct a vector */
void free_vec(vec_t vec);
