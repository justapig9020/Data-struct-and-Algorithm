#include "vector.h"
#include <strings.h>
#include <sys/_types/_size_t.h>
#include <string.h>

#define index_of(vec, idx) (void*)((char *)vec->stack + vec->entry_size * idx)

struct Vector {
    void *stack;
    size_t capacity;
    size_t len;
    size_t entry_size;
};

vec_t new_vec(size_t entry_size)
{
    return vec_with_capacity(DEFAULT_CAPACITY, entry_size);
}

vec_t vec_with_capacity(size_t capa, size_t entry_size)
{
    vec_t vec = malloc(sizeof(*vec));
    if (!vec)
        return NULL;
    vec->stack = malloc(entry_size * capa);
    if (!vec->stack) {
        free(vec);
        return NULL;
    }
    vec->capacity = capa;
    vec->len = 0;
    vec->entry_size = entry_size;
    return vec;
}

bool push(vec_t vec, void *item)
{
    if (!vec || !item)
        return false;

    if (vec->len == vec->capacity) {
        size_t new_capa = vec->capacity * 2;
        void *new_buf = malloc(new_capa * vec->entry_size);
        if (!new_buf)
            return false;
        memcpy(new_buf, vec->stack, vec->len * vec->entry_size);
        free(vec->stack);
        vec->stack = new_buf;
        vec->capacity = new_capa;
    }
    memcpy(index_of(vec, vec->len), item, vec->entry_size);
    vec->len += 1;
    return true;
}

bool pop(vec_t vec, void *ret)
{
    if (!vec || !ret)
        return false;

    if (0 == vec->len)
        return false;
    vec->len -= 1;
    memcpy(ret, index_of(vec, vec->len), vec->entry_size);
    return true;
}

size_t len(vec_t vec)
{
    return vec->len;
}

void *into_array(vec_t vec, size_t *len)
{
    if (!vec || !len)
        return NULL;
    void *ret = vec->stack;
    *len = vec->len;
    free(vec);
    return ret;
}

void free_vec(vec_t vec) {
    if (!vec)
        return;
    free(vec->stack);
    free(vec);
}
