/* usage: cc -O1 -pthread -I<snap>/contract/include churn.c -L<libdir> -lthinkthen -o churn && NT=32 ITERS=20000 LD_LIBRARY_PATH=<libdir> ./churn  (70 engines, distinct timeouts, refused base: stand-in state-table churn) */
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include "thinkthen.h"
#define NE 70
static thinkthen_engine *e[NE];
static void *w(void *a){unsigned s=(unsigned)(size_t)a;thinkthen_answer o;int n=atoi(getenv("ITERS")?getenv("ITERS"):"3000");
 for(int i=0;i<n;i++){s=s*1103515245u+12345u;thinkthen_decide(e[(s>>8)%NE],"Refund?","please refund",13,&o);}return 0;}
int main(void){setenv("THINKTHEN_BASE_URL","http://127.0.0.1:9/v1",1);setenv("THINKTHEN_MAX_RETRIES","0",1);
 for(int i=0;i<NE;i++){char b[16];snprintf(b,16,"%d",100+i);setenv("THINKTHEN_TIMEOUT_SECS",b,1);e[i]=thinkthen_engine_new();}
 int NT=atoi(getenv("NT")?getenv("NT"):"8");pthread_t t[64];for(int i=0;i<NT;i++)pthread_create(&t[i],0,w,(void*)(size_t)(i+1));for(int i=0;i<NT;i++)pthread_join(t[i],0);
 thinkthen_answer o;int rc=thinkthen_decide(e[0],"Refund?","x",1,&o);printf("done rc=%d %s\n",rc,thinkthen_error_message(e[0]));return 0;}
