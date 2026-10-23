#include <R_ext/Rdynload.h>

void R_init_thinkthen_extendr(DllInfo *dll);

void R_init_thinkthen(DllInfo *dll) {
    R_init_thinkthen_extendr(dll);
}
