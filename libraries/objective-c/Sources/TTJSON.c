/* MIT license: Copyright (c) 2026 local experiment 295 contributors. See LICENSE. */
#include "TTJSON.h"
#include <ctype.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
typedef struct { const char *p,*end; unsigned depth; } Cursor;
void tt_json_free(TTJSON *n) { if (!n) return; for(size_t i=0;i<n->count;i++){ free(n->keys?n->keys[i]:NULL); tt_json_free(n->children[i]); } free(n->keys); free(n->key_lengths); free(n->children); free(n->text); free(n); }
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
static char *str(Cursor *c,size_t *length) {
    if(c->p==c->end || *c->p++!='"')return NULL;
    char *s=malloc((size_t)(c->end-c->p)+1),*out=s;
    if(!s)return NULL;
    while(c->p<c->end){
        unsigned char ch=(unsigned char)*c->p++;
        if(ch=='"'){*length=(size_t)(out-s);*out=0;return s;}
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
static int append(TTJSON *n,TTJSON *child,char *key,size_t key_length){ size_t len=n->count+1;TTJSON **p=realloc(n->children,len*sizeof(*p));if(!p)return 0;n->children=p;
 if(n->type==TTJSONObject){char **q=realloc(n->keys,len*sizeof(*q));if(!q)return 0;n->keys=q;size_t *lengths=realloc(n->key_lengths,len*sizeof(*lengths));if(!lengths)return 0;n->key_lengths=lengths;}
 n->children[n->count]=child;if(n->keys){n->keys[n->count]=key;n->key_lengths[n->count]=key_length;}n->count=len;return 1; }
static TTJSON *value(Cursor *c){ ws(c);if(c->p==c->end || ++c->depth>64)return NULL;
 TTJSON *n=calloc(1,sizeof(*n));if(!n)return NULL;char ch=*c->p;
 if(ch=='{'||ch=='['){n->type=ch=='{'?TTJSONObject:TTJSONArray;c->p++;ws(c);char close=ch=='{'?'}':']';
  if(c->p<c->end&&*c->p==close){c->p++;goto done;}
  for(;;){char *key=NULL;size_t key_length=0;if(ch=='{'){key=str(c,&key_length);if(!key)goto bad;for(size_t i=0;i<n->count;i++)if(n->key_lengths[i]==key_length&&!memcmp(n->keys[i],key,key_length)){free(key);goto bad;}ws(c);if(c->p==c->end||*c->p++!=':'){free(key);goto bad;}}
   TTJSON *child=value(c);if(!child){free(key);goto bad;}if(!append(n,child,key,key_length)){free(key);tt_json_free(child);goto bad;}
   ws(c);if(c->p==c->end)goto bad;char sep=*c->p++;if(sep==close)break;if(sep!=',')goto bad;ws(c);
  }
 } else if(ch=='"'){n->type=TTJSONString;n->text=str(c,&n->text_length);if(!n->text)goto bad;}
 else if(ch=='-'||(ch>='0'&&ch<='9')){n->type=TTJSONNumber;const char *start=c->p;
  if(*c->p=='-')c->p++;if(c->p==c->end)goto bad;
  if(*c->p=='0')c->p++;else if(*c->p>='1'&&*c->p<='9')while(c->p<c->end&&isdigit((unsigned char)*c->p))c->p++;else goto bad;
  if(c->p<c->end&&*c->p=='.'){c->p++;if(c->p==c->end||!isdigit((unsigned char)*c->p))goto bad;while(c->p<c->end&&isdigit((unsigned char)*c->p))c->p++;}
  if(c->p<c->end&&(*c->p=='e'||*c->p=='E')){c->p++;if(c->p<c->end&&(*c->p=='+'||*c->p=='-'))c->p++;if(c->p==c->end||!isdigit((unsigned char)*c->p))goto bad;while(c->p<c->end&&isdigit((unsigned char)*c->p))c->p++;}
  n->text_length=(size_t)(c->p-start);n->text=strndup(start,n->text_length);if(!n->text||!isfinite(strtod(n->text,NULL)))goto bad;
 }else{const char *lit=ch=='t'?"true":ch=='f'?"false":ch=='n'?"null":NULL;if(!lit||(size_t)(c->end-c->p)<strlen(lit)||memcmp(c->p,lit,strlen(lit)))goto bad;c->p+=strlen(lit);n->type=ch=='n'?TTJSONNull:TTJSONBoolean;if(n->type==TTJSONBoolean){n->text_length=strlen(lit);n->text=strdup(lit);if(!n->text)goto bad;}}
done:c->depth--;return n;
bad:tt_json_free(n);c->depth--;return NULL;
}
TTJSON *tt_json_parse(const char *s,size_t len){if(!s)return NULL;Cursor c={s,s+len,0};TTJSON *n=value(&c);ws(&c);if(c.p!=c.end){tt_json_free(n);return NULL;}return n;}
const TTJSON *tt_json_get_n(const TTJSON *n,const char *key,size_t length){if(!n||n->type!=TTJSONObject||!key)return NULL;for(size_t i=0;i<n->count;i++)if(n->key_lengths[i]==length&&!memcmp(n->keys[i],key,length))return n->children[i];return NULL;}
const TTJSON *tt_json_get(const TTJSON *n,const char *key){return key?tt_json_get_n(n,key,strlen(key)):NULL;}
