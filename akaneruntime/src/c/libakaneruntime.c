#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <stdio.h>
#include <gc.h>

typedef struct ThunkType {
    void *ptr;
    int64_t arity;
    int64_t rank;
    struct ThunkType **args;
} Thunk;

static Thunk *allocThunk() {
    return GC_MALLOC(sizeof(Thunk));
}

static Thunk **allocThunkArray(int64_t len) {
    return GC_MALLOC(sizeof(Thunk *) * len);
}

void __init() {
    GC_INIT();
}

Thunk *__newFnThunk(void *fn_ptr, int64_t arity) {
    Thunk *ptr = allocThunk();
    Thunk **args = allocThunkArray(arity);
    *ptr = (Thunk){
        .ptr = fn_ptr,
        .arity = arity,
        .rank = 0,
        .args = args,
    };
    return ptr;
}

Thunk *__newNextFnThunk(Thunk *thunk, void *fn_ptr, Thunk *arg) {
    Thunk *ptr = allocThunk();
    Thunk **args = allocThunkArray(thunk->arity);
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

Thunk *__newValThunk(int64_t val) {
    Thunk *ptr = allocThunk();
    *ptr = (Thunk){
        .ptr = NULL,
        .arity = 0,
        .rank = val,
        .args = NULL,
    };
    return ptr;
}

Thunk *__callThunk(Thunk *thunk, Thunk *arg) {
    return ((Thunk *(*)(Thunk *, Thunk *))thunk->ptr)(thunk, arg);
}

void __debugPrint(Thunk *thunk) {
    printf("ptr = %p, arity = %lld, rank = %lld\n", thunk->ptr, (long long)thunk->arity, (long long)thunk->rank);
    fflush(stdout);
}
