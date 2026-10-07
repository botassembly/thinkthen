#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <dlfcn.h>
#include <unistd.h>
#include <string.h>
struct reader { pthread_t thread; void *token; void (*cancel)(void *); char *signal; };
static void *read_signal(void *opaque) {
    struct reader *r=opaque;
    while(access(r->signal,F_OK)) usleep(10000);
    r->cancel(r->token);
    char *ack=malloc(strlen(r->signal)+7); if(!ack) abort(); strcpy(ack,r->signal); strcat(ack,".fired"); FILE *f=fopen(ack,"w"); if(!f || fclose(f)) abort(); free(ack);
    return NULL;
}
void *cancel_reader(void *token, const char *library, const char *signal) {
    struct reader *r=malloc(sizeof *r); if(!r) abort();
    void *lib=dlopen(library,RTLD_NOW|RTLD_NOLOAD); if(!lib) abort();
    *(void **)(&r->cancel)=dlsym(lib,"thinkthen_cancel"); if(!r->cancel) abort();
    dlclose(lib); r->token=token; r->signal=strdup(signal); if(!r->signal) abort();
    if(pthread_create(&r->thread,NULL,read_signal,r)) abort();
    return r;
}
void cancel_join(void *opaque) {
    struct reader *r=opaque; if(pthread_join(r->thread,NULL)) abort(); free(r->signal); free(r);
}
