#include "ThinkThen.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <time.h>

static TTClient *e;
static const char *barrier;
static void require(int good,const char *note) { if (!good) { fprintf(stderr,"FAIL: %s\n",note); exit(1); } }
static void path(char *buf,size_t cap,const char *prefix,const char *state) { snprintf(buf,cap,"%s/%s%s",barrier,prefix,state); }
static void arrived(const char *state) { char b[512]; path(b,sizeof b,"arrived-",state); for(int i=0;i<2000 && access(b,F_OK);i++) usleep(5000); require(!access(b,F_OK),"counted arrival"); }
static void release(const char *state) { char b[512]; path(b,sizeof b,"release-",state); int fd=open(b,O_WRONLY|O_CREAT,0600); require(fd>=0,"release barrier"); close(fd); }
/* Facts are JSON text; fact_is compares one member's literal text, and a NULL
 * want means the member is absent. */
static int fact_is(const char *facts,const char *key,const char *want) {
    TTJSON *tree=facts?tt_json_parse(facts,strlen(facts)):NULL; const TTJSON *v=tt_json_get(tree,key);
    int same=want ? v && v->text && !strcmp(v->text,want) : !v; tt_json_free(tree); return same;
}
static void result(const char *text,int expected,int64_t deadline,TTToken *tok) {
    TTDecision a={123,-1}; TTFailure f={0}; char *facts=NULL;
    int rc=[e decide:"Is it?" text:text length:strlen(text) deadline:deadline token:tok answer:&a facts:&facts failure:&f];
    if(rc!=expected) fprintf(stderr,"state=%s code=%d expected=%d error=%s\n",text,rc,expected,f.message);
    require(rc==expected,"scalar status");
    require(expected ? a.outcome==123 && a.probability==-1 : a.outcome==(strcmp(text,"no")==0 ? 0 : 1),"scalar result");
    require(!expected || f.kind==expected,"copied same-thread error code");
    if (!expected) require(fact_is(facts,"records","1") && fact_is(facts,"requests_sent","1") && fact_is(facts,"cache_answers","0") && fact_is(facts,"model","jev-1.13.0"),"scalar facts");
    else require(!facts,"failed call has no success facts");
    free(facts);
    tt_failure_clear(&f);
}
static void json(const char *request,const char *expect) {
 TTFailure f={0};char *s=[e json:request deadline:-1 token:nil failure:&f];
 if(!s)fprintf(stderr,"json failure: %d %s\n",f.kind,f.message);
 require(s != NULL,"JSON verb result");
 TTJSON *tree=tt_json_parse(s,strlen(s));const TTJSON *value=tt_json_get(tree,"value");
 if(!value)fprintf(stderr,"invalid call envelope: %s\n",s);
 require(value != NULL,"JSON call envelope value");
 const TTJSON *facts=tt_json_get(tree,"facts");
 require(facts && !strcmp(tt_json_get(facts,"model")->text,"jev-1.13.0"),"call facts model");
 require(!strcmp(tt_json_get(facts,"records")->text,!strcmp(expect,"filter-one")||!strcmp(expect,"rank-one")?"2":"1") &&
         !strcmp(tt_json_get(facts,"requests_sent")->text,!strcmp(expect,"entities")?"2":"1") &&
         !strcmp(tt_json_get(facts,"cache_answers")->text,"0"),"exact call facts counters");
 const char *member=!strcmp(expect,"entities")||!strcmp(expect,"edges")?expect:NULL;
 if(!strcmp(expect,"check"))require(value->type==TTJSONArray && value->count==1 && tt_json_get(value->children[0],"check"),"annotate row");
 else if(member){const TTJSON *list=tt_json_get(value,member);require(list && list->type==TTJSONArray && list->count>0,"recognize or relate list");}
 else if(!strcmp(expect,"probability")){
  require(value->type==TTJSONObject && tt_json_get(value,"schema") &&
          !strcmp(tt_json_get(value,"schema")->text,"thinkthen.result/1") &&
          tt_json_get(value,"value") && tt_json_get(value,"value")->type==TTJSONBoolean &&
          !strcmp(tt_json_get(value,"value")->text,"true") &&
          tt_json_get(value,"answer") && tt_json_get(value,"answer")->type==TTJSONObject &&
          !strcmp(tt_json_get(tt_json_get(value,"answer"),"kind")->text,"yes_no") &&
          !strcmp(tt_json_get(tt_json_get(value,"answer"),"probability")->text,"0.9") &&
          !strcmp(tt_json_get(value,"threshold")->text,"0.5") &&
          !strcmp(tt_json_get(tt_json_get(value,"question"),"verb")->text,"decide"),"detailed decision true at 0.9");
 }else if(!strcmp(expect,"found")){
  require(value->type==TTJSONObject && !strcmp(tt_json_get(value,"index")->text,"0") &&
          !strcmp(tt_json_get(value,"unit")->text,"find-one") &&
          !strcmp(tt_json_get(value,"probability")->text,"0.9"),"find unit with its place and probability");
 }else if(!strcmp(expect,"first")){
  if(value->type==TTJSONString)require(!strcmp(value->text,expect),"literal JSON choice value");
  else {require(value->type==TTJSONArray && value->count==2 &&
                 !strcmp(value->children[0]->text,"first") &&
                 !strcmp(value->children[1]->text,"second"),"literal ordered tag labels");}
 }else if(!strcmp(expect,"0"))require(value->type==TTJSONNumber && !strcmp(value->text,"0.1"),"literal score 0.1");
 else if(!strcmp(expect,"filter-one"))
  require(value->type==TTJSONArray && value->count==2 &&
          !strcmp(value->children[0]->text,"filter-one") &&
          !strcmp(value->children[1]->text,"filter-two"),"literal ordered record answers");
 else if(!strcmp(expect,"rank-one"))
  require(value->type==TTJSONArray && value->count==2 &&
          !strcmp(tt_json_get(value->children[0],"index")->text,"0") &&
          !strcmp(tt_json_get(value->children[0],"record")->text,"rank-one") &&
          !strcmp(tt_json_get(value->children[0],"probability")->text,"0.9") &&
          !strcmp(tt_json_get(value->children[1],"index")->text,"1") &&
          !strcmp(tt_json_get(value->children[1],"record")->text,"rank-two"),"rank rows with place and probability");
 else require(strstr(s,expect) != NULL,"JSON verb result literal");
 tt_json_free(tree);free(s);tt_failure_clear(&f);
}
typedef struct { const char *state; TTToken *token; int64_t deadline; int code; TTDecision answer; char *facts; TTFailure failure; } Scalar;
static void *caller(void *v) { Scalar *s=v; s->answer=(TTDecision){123,-1}; s->code=[e decide:"Is it?" text:s->state length:strlen(s->state) deadline:s->deadline token:s->token answer:&s->answer facts:&s->facts failure:&s->failure]; return NULL; }
typedef struct {const char **texts; size_t *lengths; size_t n; TTToken *token; int code; TTDecision answers[3]; char *facts; TTFailure failure;} Bulk;
static void *bulkcaller(void *v) { Bulk *b=v; b->code=[e many:"Is it?" texts:b->texts lengths:b->lengths count:b->n deadline:-1 token:b->token answers:b->answers facts:&b->facts failure:&b->failure]; return NULL; }
int main(int argc,char **argv) {
 barrier=getenv("TT_BARRIER_DIR"); require(barrier!=NULL,"barrier"); e=[TTClient create]; require(e!=nil,"engine"); require(sizeof(TTDecision)==16,"ABI layout");
 const char *mode=argc>1?argv[1]:"basic";
 if(!strcmp(mode,"basic")) {
  result("no",0,-1,nil); result("unsure",0,-1,nil);
  json("{\"decide\":\"Is it?\",\"evidence\":\"json-decide\",\"details\":true}","probability");
  json("{\"choose\":\"Which team?\",\"options\":{\"first\":{\"what\":\"First team\",\"not_for\":\"Second\",\"examples\":[\"A\"]},\"second\":\"Second team\"},\"evidence\":\"choose\"}","first");
  json("{\"tag\":\"Which labels?\",\"labels\":{\"first\":\"First label\",\"second\":\"Second label\"},\"evidence\":\"tag\"}","first");
  json("{\"score\":\"What level?\",\"levels\":[\"Low.\",\"High.\"],\"evidence\":\"score\"}","0");
  json("{\"filter\":\"Is it?\",\"records\":[\"filter-one\",\"filter-two\"]}","filter-one");
  json("{\"rank\":\"Is it?\",\"records\":[\"rank-one\",\"rank-two\"]}","rank-one");
  json("{\"find\":\"Which line?\",\"units\":[\"find-one\",\"find-two\"]}","found");
  json("{\"annotate\":{\"version\":1,\"questions\":{\"check\":{\"decide\":\"Is it?\"}}},\"records\":[\"annotate-one\"]}","check");
  json("{\"recognize\":{\"kinds\":{\"person\":\"A person's name.\"}},\"version\":1,\"evidence\":\"Maria Chen\"}","entities");
  json("{\"relate\":{\"relations\":[{\"name\":\"caused_by\",\"source\":\"alert\",\"target\":\"alert\"}]},\"version\":1,\"records\":[{\"name\":\"First\",\"kind\":\"alert\"},{\"name\":\"Second\",\"kind\":\"alert\"}]}","edges");
  puts("OBJC_BASIC_PASS");
 } else if(!strcmp(mode,"bulk")) {
  const char *ts[]={"first","second","third"}; size_t ns[]={5,6,5}; TTDecision a[3]={{123,-1},{123,-1},{123,-1}}; TTFailure f={0}; char *facts=NULL;
  require([e many:"Is it?" texts:ts lengths:ns count:3 deadline:-1 token:nil answers:a facts:&facts failure:&f]==0,"bulk");
  require(a[0].probability==0.9 && a[1].probability==0.1 && a[2].probability==0.6 && fact_is(facts,"records","3") && fact_is(facts,"requests_sent","1") && fact_is(facts,"cache_answers","0"),"bulk order and facts"); free(facts);tt_failure_clear(&f); puts("OBJC_BULK_PASS");
  require([e many:"Is it?" texts:ts lengths:ns count:3 deadline:-1 token:nil answers:a facts:&facts failure:&f]==0 &&
          fact_is(facts,"records","3") && fact_is(facts,"requests_sent","0") && fact_is(facts,"cache_answers","3") && a[2].probability==0.6,
          "identical bulk answers each question from the cache"); free(facts);tt_failure_clear(&f);
  require([e many:"Is it?" texts:NULL lengths:NULL count:0 deadline:-1 token:nil answers:NULL facts:&facts failure:&f]==0 &&
          fact_is(facts,"records","0") && fact_is(facts,"requests_sent","0") && fact_is(facts,"cache_answers","0") && fact_is(facts,"model",NULL) && fact_is(facts,"input_tokens",NULL),
          "empty bulk has no model or send"); free(facts);tt_failure_clear(&f);
 } else if(!strcmp(mode,"strict")) {
  TTToken *t=[TTToken create]; Scalar s={.state="hold-scalar",.token=t,.deadline=-1}; pthread_t p;
  require(!pthread_create(&p,NULL,caller,&s),"pthread"); arrived("hold-scalar"); [t fire]; [t fire]; release("hold-scalar"); pthread_join(p,NULL);
  require(s.code==5 && s.failure.kind==5 && !s.failure.retryable && s.failure.message && s.answer.outcome==123 && s.answer.probability==-1,"strict scalar cancellation"); free(s.facts);tt_failure_clear(&s.failure); [t dealloc]; result("recovery-scalar",0,-1,nil); puts("STRICT_OBJC_CANCEL_PASS");
 } else if(!strcmp(mode,"held")) {
  TTToken *t=[TTToken create]; const char *ts[]={"hold-bulk-1","hold-bulk-2"}; size_t ns[]={11,11}; Bulk b={.texts=ts,.lengths=ns,.n=2,.token=t,.answers={{123,-1},{123,-1}}}; pthread_t p;
  require(!pthread_create(&p,NULL,bulkcaller,&b),"pthread bulk"); arrived("hold-bulk-1"); [t fire]; release("hold-bulk-1"); /* second row may never send after cancellation */
  pthread_join(p,NULL); require(b.code==5 && b.failure.kind==5 && !b.failure.retryable && b.answers[0].outcome==123 && b.answers[0].probability==-1 && b.answers[1].outcome==123 && b.answers[1].probability==-1,"held bulk"); free(b.facts);tt_failure_clear(&b.failure); [t dealloc];
  Scalar d={.state="hold-deadline",.deadline=150}; require(!pthread_create(&p,NULL,caller,&d),"pthread deadline"); arrived("hold-deadline"); pthread_join(p,NULL); release("hold-deadline");
  require(d.code==3 && d.answer.outcome==123 && d.answer.probability==-1,"held deadline"); free(d.facts);tt_failure_clear(&d.failure); result("recovery-held",0,-1,nil); puts("OBJC_HELD_PASS");
 } else if(!strcmp(mode,"boundaries")) {
  const char data[]={'a',0,'b'}; TTDecision a={123,-1}; TTFailure f={0}; char *facts=NULL;
  require([e decide:"Is it?" text:data length:3 deadline:-1 token:nil answer:&a facts:&facts failure:&f]==0 && a.outcome==1,"embedded NUL evidence"); free(facts); facts=NULL;tt_failure_clear(&f);
  const char invalid[]="{\"decide\":\"Is it?\"}\0junk";
  require([e jsonBytes:invalid length:sizeof(invalid)-1 deadline:-1 token:nil failure:&f]==NULL && f.kind==1,"reject JSON C-string with embedded NUL"); tt_failure_clear(&f);
  const char badQuestion[]="Is it?\0junk"; a=(TTDecision){123,-1};
  require([e decideBytes:badQuestion questionLength:sizeof(badQuestion)-1 text:"no" length:2 deadline:-1 token:nil answer:&a facts:&facts failure:&f]==1 && a.outcome==123,"reject question C-string with embedded NUL"); tt_failure_clear(&f);
  result("negative-budget",1,-2,nil); result("spent-budget",3,0,nil);
  TTToken *t=[TTToken create]; [t fire]; result("prefired",5,-1,t); [t dealloc];
  result("status-401",2,-1,nil); result("failure-two",2,-1,nil);
  TTClient *other=[TTClient create]; require(other!=nil,"second engine");
  a=(TTDecision){123,-1}; require([other decide:"Is it?" text:"other-engine-error" length:18 deadline:-1 token:nil answer:&a facts:&facts failure:&f]==2,"other engine failure");
  result("after-error",0,-1,nil);
  [other dealloc]; require(f.message && strstr(f.message,"401") && a.outcome==123,"copied typed error after later call and close");
  TTJSON *failed=tt_json_parse(f.facts_json,strlen(f.facts_json));
  require(failed && !strcmp(tt_json_get(failed,"records")->text,"0") &&
          !strcmp(tt_json_get(failed,"requests_sent")->text,"1"),"first failed row sends one request and completes no record");
  tt_json_free(failed); tt_failure_clear(&f);
  puts("OBJC_BOUNDARIES_PASS");
 } else if(!strcmp(mode,"concurrent")) {
  Scalar s[3]={{.state="failure-one",.deadline=-1},{.state="failure-two",.deadline=-1},{.state="success",.deadline=-1}}; pthread_t p[3]; for(int i=0;i<3;i++)require(!pthread_create(&p[i],NULL,caller,&s[i]),"pthread caller");
  for(int i=0;i<3;i++)pthread_join(p[i],NULL);
  require(s[0].code==2 && s[1].code==2 && s[2].code==0 && strstr(s[0].failure.message,"401") && strstr(s[1].failure.message,"403"),"thread local failures");
  require(fact_is(s[2].facts,"records","1") && fact_is(s[2].facts,"requests_sent","1"),"concurrent owned success facts");
  for(int i=0;i<3;i++){free(s[i].facts);tt_failure_clear(&s[i].failure);} puts("OBJC_CONCURRENT_PASS");
 } else if(!strcmp(mode,"overlap")) {
  Scalar s[2]={{.state="hold-overlap-a",.deadline=-1},{.state="hold-overlap-b",.deadline=-1}}; pthread_t p[2];
  require(!pthread_create(&p[0],NULL,caller,&s[0]) && !pthread_create(&p[1],NULL,caller,&s[1]),"overlap callers");
  arrived("hold-overlap-a"); arrived("hold-overlap-b");
  release("hold-overlap-a"); release("hold-overlap-b");
  pthread_join(p[0],NULL); pthread_join(p[1],NULL);
  require(s[0].code==0 && s[1].code==0 && fact_is(s[0].facts,"requests_sent","1") && fact_is(s[1].facts,"requests_sent","1"),
          "two held typed calls completed independently");
  [e dealloc]; e=nil;
  require(fact_is(s[0].facts,"model","jev-1.13.0") && fact_is(s[1].facts,"model","jev-1.13.0"),"owned facts survive close");
  for(int i=0;i<2;i++){free(s[i].facts);tt_failure_clear(&s[i].failure);}
  puts("OBJC_HELD_OVERLAP_PASS");
 } else if(!strcmp(mode,"typed-json")) {
  TTFailure f={0}; char *facts=NULL; char *out=NULL; size_t n=777;
  require([e recognize:"{\"version\":1,\"recognize\":{\"kinds\":{\"person\":\"A person's name.\"}}}" text:"John Smith" length:10 result:&out size:&n deadline:-1 token:nil facts:&facts failure:&f]==0 && out && n==strlen(out) && strstr(out,"entities"),"typed recognize"); TTJSON *recognized=tt_json_parse(out,n); require(tt_json_get(recognized,"entities") && fact_is(facts,"records","1") && fact_is(facts,"requests_sent","2"),"recognized entities and facts"); tt_json_free(recognized); free(out); free(facts);tt_failure_clear(&f);
  /* Shared non-BMP offset case: scalar positions, never UTF-16 or UTF-8 byte offsets. */
  const char *emoji="🧬"; out=NULL; n=777;
  require([e recognize:"{\"version\":1,\"recognize\":{}}" text:emoji length:strlen(emoji) result:&out size:&n deadline:-1 token:nil facts:&facts failure:&f]==0 && out,"emoji recognition");
  recognized=tt_json_parse(out,n); require(recognized != NULL,"emoji entity JSON");
  const TTJSON *entities=tt_json_get(recognized,"entities");
  require(entities && entities->count>0,"emoji entity present");
  const TTJSON *one=entities->children[0];
  require(!strcmp(tt_json_get(one,"start")->text,"0") && !strcmp(tt_json_get(one,"end")->text,"1") && !strcmp(tt_json_get(one,"length")->text,"1"),"non-BMP scalar offsets 0..1");
  tt_json_free(recognized);free(out);free(facts);tt_failure_clear(&f);
  const char *ts[]={"{\"name\":\"Third\",\"kind\":\"alert\"}","{\"name\":\"Fourth\",\"kind\":\"alert\"}"}; size_t ns[]={strlen(ts[0]),strlen(ts[1])}; out=NULL; n=777;
  require([e relate:"{\"version\":1,\"relate\":{\"relations\":[{\"name\":\"caused_by\",\"source\":\"alert\",\"target\":\"alert\"}]}}" texts:ts lengths:ns count:2 result:&out size:&n deadline:-1 token:nil facts:&facts failure:&f]==0 && out && n==strlen(out) && strstr(out,"edges"),"typed relate"); TTJSON *related=tt_json_parse(out,n); require(tt_json_get(related,"edges") && fact_is(facts,"records","1"),"related edges and logical question count"); tt_json_free(related); free(out); free(facts);tt_failure_clear(&f); puts("OBJC_TYPED_JSON_PASS");
 } else if(!strcmp(mode,"reverse")) {
  const char *ts[]={"hold-reverse-1","hold-reverse-2"}; size_t ns[]={14,14}; Bulk b={.texts=ts,.lengths=ns,.n=2,.answers={{123,-1},{123,-1}}}; pthread_t p;
  require(!pthread_create(&p,NULL,bulkcaller,&b),"reverse thread"); arrived("hold-reverse-1"); arrived("hold-reverse-2"); release("hold-reverse-2"); usleep(50000); release("hold-reverse-1"); pthread_join(p,NULL);
  require(b.code==0 && b.answers[0].probability==0.9 && b.answers[1].probability==0.1 && fact_is(b.facts,"records","2"),"reverse order and facts"); free(b.facts);tt_failure_clear(&b.failure); puts("OBJC_REVERSE_PASS");
 } else require(0,"unknown mode");
 if(e) [e dealloc]; return 0;
}
