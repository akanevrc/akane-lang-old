#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <stdio.h>

typedef struct ThunkType {
    void *ptr;
    int64_t arity;
    int64_t rank;
    struct ThunkType **args;
} Thunk;

static Thunk *alloc_thunk() {
    return malloc(sizeof(Thunk));
}

static Thunk **alloc_thunk_array(int64_t len) {
    return malloc(sizeof(Thunk *) * len);
}

Thunk *__new_fn_thunk(void *fn_ptr, int64_t arity) {
    Thunk *ptr = alloc_thunk();
    Thunk **args = alloc_thunk_array(arity);
    *ptr = (Thunk){
        .ptr = fn_ptr,
        .arity = arity,
        .rank = 0,
        .args = args,
    };
    return ptr;
}

Thunk *__new_next_fn_thunk(Thunk *thunk, void *fn_ptr, Thunk *arg) {
    Thunk *ptr = alloc_thunk();
    Thunk **args = alloc_thunk_array(thunk->arity);
    memcpy(args, thunk->args, sizeof(Thunk *) * thunk->rank);
    args[thunk->rank] = arg;
    *ptr = (Thunk){
        .ptr = fn_ptr,
        .arity = thunk->arity,
        .rank = thunk->rank + 1,
        .args = args,
    };
    return ptr;
}

Thunk *__new_val_thunk(int64_t val) {
    Thunk *ptr = alloc_thunk();
    *ptr = (Thunk){
        .ptr = NULL,
        .arity = 0,
        .rank = val,
        .args = NULL,
    };
    return ptr;
}

void __delete_thunk(Thunk *thunk) {
    if (thunk->args != NULL) {
        free(thunk->args);
    }
    free(thunk);
}

Thunk *__call_thunk(Thunk *thunk, Thunk *arg) {
    return ((Thunk *(*)(Thunk *, Thunk *))thunk->ptr)(thunk, arg);
}

void __debug_print(Thunk *thunk) {
    printf("ptr = %p, arity = %lld, rank = %lld\n", thunk->ptr, (long long)thunk->arity, (long long)thunk->rank);
    fflush(stdout);
}
