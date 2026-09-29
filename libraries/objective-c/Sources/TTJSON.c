/* MIT license: Copyright (c) 2026 local experiment 295 contributors. See LICENSE. */
#include "TTJSON.h"
#include <ctype.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
typedef struct { const char *p,*end; unsigned depth; } Cursor;
void tt_json_free(TTJSON *n) { if (!n) return; for(size_t i=0;i<n->count;i++){ free(n->keys?n->keys[i]:NULL); tt_json_free(n->children[i]); } free(n->keys); free(n->children); free(n->text); free(n); }
static void ws(Cursor *c) { while(c->p<c->end && (*c->p==' '||*c->p=='\n'||*c->p=='\r'||*c->p=='\t')) c->p++; }
static int hex4(Cursor *c,unsigned *value) {
    if(c->end-c->p<4)return 0;
    *value=0;
    for(int i=0;i<4;i++){
        unsigned char ch=(unsigned char)*c->p++;
        if(ch>='0'&&ch<='9')*value=(*value<<4)|(ch-'0');
        else if(ch>='a'&&ch<='f')*value=(*value<<4)|(ch-'a'+10);
        else if(ch>='A'&&ch<='F')*value=(*value<<4)|(ch-'A'+10);
        else return 0;
    }
    return 1;
}
static char *str(Cursor *c) {
    if(c->p==c->end || *c->p++!='"')return NULL;
    char *s=malloc((size_t)(c->end-c->p)+1),*out=s;
    if(!s)return NULL;
    while(c->p<c->end){
        unsigned char ch=(unsigned char)*c->p++;
        if(ch=='"'){*out=0;return s;}
        if(ch<32)break;
        if(ch=='\\'){
            if(c->p==c->end)break;
            ch=(unsigned char)*c->p++;
            if(ch=='u'){
                unsigned scalar,low;
                if(!hex4(c,&scalar))break;
                if(scalar>=0xd800&&scalar<=0xdbff){
                    if(c->end-c->p<2||c->p[0]!='\\'||c->p[1]!='u')break;
                    c->p+=2;
                    if(!hex4(c,&low)||low<0xdc00||low>0xdfff)break;
                    scalar=0x10000+((scalar-0xd800)<<10)+(low-0xdc00);
                }else if(scalar>=0xdc00&&scalar<=0xdfff)break;
                if(!scalar)break; /* C-string keys cannot contain NUL */
                if(scalar<0x80)*out++=(char)scalar;
                else if(scalar<0x800){*out++=(char)(0xc0|(scalar>>6));*out++=(char)(0x80|(scalar&63));}
                else if(scalar<0x10000){*out++=(char)(0xe0|(scalar>>12));*out++=(char)(0x80|((scalar>>6)&63));*out++=(char)(0x80|(scalar&63));}
                else {*out++=(char)(0xf0|(scalar>>18));*out++=(char)(0x80|((scalar>>12)&63));*out++=(char)(0x80|((scalar>>6)&63));*out++=(char)(0x80|(scalar&63));}
                continue;
            }
            if(!strchr("\"\\/bfnrt",ch))break;
            if(ch=='n')ch='\n';else if(ch=='t')ch='\t';else if(ch=='r')ch='\r';
            else if(ch=='b')ch='\b';else if(ch=='f')ch='\f';
        }
        if(ch>=0x80){
            unsigned remain=ch>=0xf0?3:ch>=0xe0?2:ch>=0xc2?1:0;
            if(!remain||c->end-c->p<(ptrdiff_t)remain)break;
            if((remain==3&&ch>0xf4)||
               (ch==0xe0&&(unsigned char)c->p[0]<0xa0)||
               (ch==0xed&&(unsigned char)c->p[0]>=0xa0)||
               (ch==0xf0&&(unsigned char)c->p[0]<0x90)||
               (ch==0xf4&&(unsigned char)c->p[0]>=0x90))break;
            int valid=1;for(unsigned i=0;i<remain;i++)if(((unsigned char)c->p[i]&0xc0)!=0x80)valid=0;
            if(!valid)break;
            *out++=(char)ch;for(unsigned i=0;i<remain;i++)*out++=*c->p++;
        }else *out++=(char)ch;
    }
    free(s);return NULL;
}
static TTJSON *value(Cursor *c);
static int append(TTJSON *n,TTJSON *child,char *key){ size_t len=n->count+1;TTJSON **p=realloc(n->children,len*sizeof(*p));if(!p)return 0;n->children=p;
 if(n->type==TTJSONObject){char **q=realloc(n->keys,len*sizeof(*q));if(!q)return 0;n->keys=q;}
 n->children[n->count]=child;if(n->keys)n->keys[n->count]=key;n->count=len;return 1; }
static TTJSON *value(Cursor *c){ ws(c);if(c->p==c->end || ++c->depth>64)return NULL;
 TTJSON *n=calloc(1,sizeof(*n));if(!n)return NULL;char ch=*c->p;
 if(ch=='{'||ch=='['){n->type=ch=='{'?TTJSONObject:TTJSONArray;c->p++;ws(c);char close=ch=='{'?'}':']';
  if(c->p<c->end&&*c->p==close){c->p++;goto done;}
  for(;;){char *key=NULL;if(ch=='{'){key=str(c);if(!key)goto bad;for(size_t i=0;i<n->count;i++)if(!strcmp(n->keys[i],key)){free(key);goto bad;}ws(c);if(c->p==c->end||*c->p++!=':'){free(key);goto bad;}}
   TTJSON *child=value(c);if(!child){free(key);goto bad;}if(!append(n,child,key)){free(key);tt_json_free(child);goto bad;}
   ws(c);if(c->p==c->end)goto bad;char sep=*c->p++;if(sep==close)break;if(sep!=',')goto bad;ws(c);
  }
 } else if(ch=='"'){n->type=TTJSONString;n->text=str(c);if(!n->text)goto bad;}
 else if(ch=='-'||(ch>='0'&&ch<='9')){n->type=TTJSONNumber;const char *start=c->p;
  if(*c->p=='-')c->p++;if(c->p==c->end)goto bad;
  if(*c->p=='0')c->p++;else if(*c->p>='1'&&*c->p<='9')while(c->p<c->end&&isdigit((unsigned char)*c->p))c->p++;else goto bad;
  if(c->p<c->end&&*c->p=='.'){c->p++;if(c->p==c->end||!isdigit((unsigned char)*c->p))goto bad;while(c->p<c->end&&isdigit((unsigned char)*c->p))c->p++;}
  if(c->p<c->end&&(*c->p=='e'||*c->p=='E')){c->p++;if(c->p<c->end&&(*c->p=='+'||*c->p=='-'))c->p++;if(c->p==c->end||!isdigit((unsigned char)*c->p))goto bad;while(c->p<c->end&&isdigit((unsigned char)*c->p))c->p++;}
  n->text=strndup(start,(size_t)(c->p-start));if(!n->text||!isfinite(strtod(n->text,NULL)))goto bad;
 }else{const char *lit=ch=='t'?"true":ch=='f'?"false":ch=='n'?"null":NULL;if(!lit||(size_t)(c->end-c->p)<strlen(lit)||memcmp(c->p,lit,strlen(lit)))goto bad;c->p+=strlen(lit);n->type=ch=='n'?TTJSONNull:TTJSONBoolean;if(n->type==TTJSONBoolean){n->text=strdup(lit);if(!n->text)goto bad;}}
done:c->depth--;return n;
bad:tt_json_free(n);c->depth--;return NULL;
}
TTJSON *tt_json_parse(const char *s,size_t len){if(!s)return NULL;Cursor c={s,s+len,0};TTJSON *n=value(&c);ws(&c);if(c.p!=c.end){tt_json_free(n);return NULL;}return n;}
const TTJSON *tt_json_get(const TTJSON *n,const char *key){if(!n||n->type!=TTJSONObject)return NULL;for(size_t i=0;i<n->count;i++)if(!strcmp(n->keys[i],key))return n->children[i];return NULL;}
static int has(const TTJSON *n,const char *key,TTJSONType type){const TTJSON *v=tt_json_get(n,key);return v&&v->type==type;}
const TTJSON *tt_json_call_value(const TTJSON *root){
 if(!root||root->type!=TTJSONObject||root->count!=2)return NULL;
 const TTJSON *v=tt_json_get(root,"value"),*f=tt_json_get(root,"facts");
 if(!v||!f||f->type!=TTJSONObject||f->count<4||f->count>7||
    !has(f,"records",TTJSONNumber)||!has(f,"requests_sent",TTJSONNumber)||
    !has(f,"cache_answers",TTJSONNumber)||!has(f,"seconds",TTJSONNumber))return NULL;
 for(size_t i=0;i<f->count;i++){
  const char *k=f->keys[i];const TTJSON *n=f->children[i];
  if(!strcmp(k,"records")||!strcmp(k,"requests_sent")||!strcmp(k,"cache_answers")||!strcmp(k,"input_tokens")||!strcmp(k,"output_tokens")){
   if(n->type!=TTJSONNumber||!n->text||strtod(n->text,NULL)<0||floor(strtod(n->text,NULL))!=strtod(n->text,NULL))return NULL;
  }else if(!strcmp(k,"seconds")){
   if(n->type!=TTJSONNumber||!n->text||strtod(n->text,NULL)<0)return NULL;
  }else if(!strcmp(k,"model")){
   if(n->type!=TTJSONString)return NULL;
  }else return NULL;
 }
 return v;
}
static int failure(const TTJSON *n){
 const TTJSON *f=tt_json_get(n,"failed");if(n->type!=TTJSONObject||n->count!=1||!f||f->type!=TTJSONObject||f->count!=2||!has(f,"kind",TTJSONString)||!has(f,"cause",TTJSONString))return 0;
 const char *kind=tt_json_get(f,"kind")->text;
 return !strcmp(kind,"usage")||!strcmp(kind,"backend")||!strcmp(kind,"deadline")||!strcmp(kind,"local")||!strcmp(kind,"cancelled")||!strcmp(kind,"defect");
}
static int field(const TTJSON *n){if(n->type==TTJSONNull||n->type==TTJSONBoolean||n->type==TTJSONString||n->type==TTJSONNumber)return 1;if(failure(n))return 1;
 if(n->type!=TTJSONArray)return 0;for(size_t i=0;i<n->count;i++)if(n->children[i]->type!=TTJSONString)return 0;return 1;}
int tt_json_answer_shape(const TTJSON *root,const char *kind){if(!root||!kind)return 0;
 if(!strcmp(kind,"annotate")){ if(root->type!=TTJSONArray && root->type!=TTJSONObject)return 0;
  if(root->type==TTJSONArray){for(size_t i=0;i<root->count;i++)if(!tt_json_answer_shape(root->children[i],kind))return 0;return 1;}
  for(size_t i=0;i<root->count;i++)if(!field(root->children[i]))return 0;return 1; }
 if(root->type!=TTJSONObject)return 0;
 if(!strcmp(kind,"recognize")){const TTJSON *entities=tt_json_get(root,"entities"),*relations=tt_json_get(root,"relations");if(!entities||entities->type!=TTJSONArray)return 0;
  for(size_t i=0;i<entities->count;i++){const TTJSON *e=entities->children[i];if(e->type!=TTJSONObject||!has(e,"text",TTJSONString)||!has(e,"kind",TTJSONString)||!has(e,"start",TTJSONNumber)||!has(e,"end",TTJSONNumber)||!has(e,"length",TTJSONNumber)||!has(e,"strength",TTJSONNumber))return 0;
   double start=strtod(tt_json_get(e,"start")->text,NULL),end=strtod(tt_json_get(e,"end")->text,NULL),len=strtod(tt_json_get(e,"length")->text,NULL);if(start<0||end<start||len!=end-start||floor(start)!=start||floor(end)!=end)return 0;}
  return !relations || relations->type==TTJSONArray; }
 if(!strcmp(kind,"relate")){const TTJSON *edges=tt_json_get(root,"edges");if(!edges||edges->type!=TTJSONArray)return 0;for(size_t i=0;i<edges->count;i++){const TTJSON *e=edges->children[i];if(e->type!=TTJSONObject||!has(e,"relation",TTJSONString)||!tt_json_get(e,"source")||!tt_json_get(e,"target")||!has(e,"probability",TTJSONNumber))return 0;}return 1;}
 return 0;
}
