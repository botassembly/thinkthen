#include "thinkthen.h"
#include <assert.h>
#include <stdio.h>
#include <string.h>
int main(void) {
    thinkthen_engine *e=thinkthen_engine_new(); assert(e);
    thinkthen_answer a={123,-1};
    const char text[]="caf\303\251";
    int rc=thinkthen_decide(e,"Is it?",text,sizeof(text)-1,&a);
    if (rc) { fprintf(stderr,"%d %s\n",rc,thinkthen_error_message(e)); return 1; }
    assert(a.outcome==THINKTHEN_YES && a.probability>0.8);
    thinkthen_engine_free(e);
    puts("OBJC_DIRECT_PASS");
}
