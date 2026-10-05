/* MIT License; see ../LICENSE. C-only, dependency-free JSON grammar adapter.
 * A parsed object is never mutated: map descriptions cross unchanged.
 * Functions accept byte length and never read past the caller's storage. */
#include "TTJSON.h"
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
static int description(const TTJSON *n) {
    const TTJSON *a, *b, *c;
    if (n->type==TTJSONString) return 1;
    if (n->type!=TTJSONObject || n->count!=3) return 0;
    a=tt_json_get(n,"what");b=tt_json_get(n,"not_for");c=tt_json_get(n,"examples");
    if (!a||a->type!=TTJSONString||!b||b->type!=TTJSONString||!c||c->type!=TTJSONArray) return 0;
    for (size_t i=0;i<c->count;i++) if (c->children[i]->type!=TTJSONString) return 0;
    return 1;
}
/* Returns 0 on valid grammar, 1 on invalid; writes offending label, bounded.
 * List is entirely bare; map is entirely described; never synthesize values. */
int tt_cobol_labels(const char *json,size_t length,char *bad,size_t capacity) {
    TTJSON *n=tt_json_parse(json,length);
    if(bad&&capacity) bad[0]=0;
    if(!n) return 1;
    int ok=(n->type==TTJSONArray || n->type==TTJSONObject) && n->count>0;
    if(n->type==TTJSONArray){
        for(size_t i=0;i<n->count;i++) if(n->children[i]->type!=TTJSONString || !n->children[i]->text_length){
            ok=0;
            if(bad&&capacity){
                const TTJSON *item=n->children[i];
                const char *label=item->type==TTJSONString?item->text:
                    item->type==TTJSONObject&&item->count?item->keys[0]:"unnamed label";
                snprintf(bad,capacity,"%s",label);
            }
            break;
        }
    } else if(n->type==TTJSONObject){
        for(size_t i=0;i<n->count;i++) if(!n->key_lengths[i] || !description(n->children[i])){
            ok=0;if(bad&&capacity) snprintf(bad,capacity,"%s",n->keys[i]);break;
        }
    }
    tt_json_free(n);return ok?0:1;
}
/* One annotate answer member: 1 unresolved null; 2 failure, writing its kind
 * code 1 to 6 and its cause; 3 answered; 0 another object or bad JSON. */
int tt_cobol_field(const char *json,size_t length,int *kind,char *cause,size_t capacity) {
    static const char *kinds[]={"usage","backend","deadline","local","cancelled","defect"};
    TTJSON *n=tt_json_parse(json,length);if(!n)return 0;
    int result=n->type==TTJSONNull?1:n->type==TTJSONObject?0:3;
    const TTJSON *failed=tt_json_get(n,"failed"),*k=tt_json_get(failed,"kind"),*c=tt_json_get(failed,"cause");
    if(k&&k->type==TTJSONString&&c&&c->type==TTJSONString)
        for(int i=0;i<6;i++) if(k->text_length==strlen(kinds[i])&&!memcmp(k->text,kinds[i],k->text_length)){
            *kind=i+1;snprintf(cause,capacity,"%.*s",(int)c->text_length,c->text);result=2;
        }
    tt_json_free(n);return result;
}
/* Copy one member's JSON text: an object member by name, or an array element
 * by its 1-based decimal index. 0 found; 1 missing; 2 bad JSON or too long.
 * Members the caller does not name are never read. */
int tt_cobol_member(const char *json,size_t length,const char *name,size_t name_length,
                    char *out,size_t capacity,size_t *out_length) {
    TTJSON *n=tt_json_parse(json,length);if(!n)return 2;
    const TTJSON *member=NULL;
    if(n->type==TTJSONObject) member=tt_json_get_n(n,name,name_length);
    else if(n->type==TTJSONArray&&name_length&&name_length<10){
        size_t index=0;for(size_t i=0;i<name_length;i++){if(name[i]<'0'||name[i]>'9'){index=0;break;}index=index*10+(size_t)(name[i]-'0');}
        if(index&&index<=n->count) member=n->children[index-1];
    }
    int result=!member?1:member->source_length>capacity?2:0;
    if(!result){memcpy(out,member->source,member->source_length);*out_length=member->source_length;}
    tt_json_free(n);return result;
}
/* Append text as one JSON string; quote, backslash and control bytes escape. */
static int quoted(char *out,size_t capacity,size_t *at,const char *text,size_t n) {
    if(*at+n*6+2>capacity) return 0;
    out[(*at)++]='"';
    for(size_t i=0;i<n;i++){
        unsigned char ch=(unsigned char)text[i];
        if(ch=='"'||ch=='\\'){out[(*at)++]='\\';out[(*at)++]=(char)ch;}
        else if(ch<0x20){char hex[7];snprintf(hex,sizeof hex,"\\u%04x",ch);memcpy(out+*at,hex,6);*at+=6;}
        else out[(*at)++]=(char)ch;
    }
    out[(*at)++]='"';return 1;
}
static int raw(char *out,size_t capacity,size_t *at,const char *text,size_t n) {
    if(*at+n>capacity) return 0;
    memcpy(out+*at,text,n);*at+=n;return 1;
}
static int json_object(const char *text,size_t length) {
    TTJSON *n=tt_json_parse(text,length);int object=n&&n->type==TTJSONObject;tt_json_free(n);return object;
}
/* Build one closed thinkthen.plan-input/1 object. A question that starts with
 * "{" is a question object, spliced as written; settings may be empty. Each
 * text row is an 8-byte length and 256 bytes. 0 built; 1 usage; 2 too long. */
int tt_cobol_plan_input(const char *verb,size_t verb_length,const char *question,size_t question_length,
                        const char *rows,uint32_t count,const char *settings,size_t settings_length,
                        char *out,size_t capacity,size_t *out_length) {
    size_t at=0,skip=0;
    while(skip<question_length&&(question[skip]==' '||question[skip]=='\t'||question[skip]=='\n'||question[skip]=='\r')) skip++;
    int object=skip<question_length&&question[skip]=='{';
    if((object&&!json_object(question,question_length))||(settings_length&&!json_object(settings,settings_length))) return 1;
    int ok=raw(out,capacity,&at,"{\"verb\":",8)&&quoted(out,capacity,&at,verb,verb_length)&&raw(out,capacity,&at,",\"question\":",12)&&
           (object?raw(out,capacity,&at,question,question_length):quoted(out,capacity,&at,question,question_length))&&
           raw(out,capacity,&at,",\"input\":[",10);
    for(uint32_t i=0;ok&&i<count;i++){
        uint64_t n;memcpy(&n,rows+(size_t)i*264,8);
        if(n>256) return 1;
        ok=(!i||raw(out,capacity,&at,",",1))&&quoted(out,capacity,&at,rows+(size_t)i*264+8,(size_t)n);
    }
    ok=ok&&raw(out,capacity,&at,"]",1)&&(!settings_length||(raw(out,capacity,&at,",\"settings\":",12)&&raw(out,capacity,&at,settings,settings_length)))&&
       raw(out,capacity,&at,"}",1);
    if(!ok) return 2;
    *out_length=at;return 0;
}
/* Compose a source envelope without reading any file or changing question grammar. */
int tt_cobol_files_input(const char *question,size_t qlen,const char *source,size_t slen,
                        char *out,size_t capacity,size_t *length) {
    TTJSON *q=tt_json_parse(question,qlen),*s=tt_json_parse(source,slen);
    if(!q||!s||q->type!=TTJSONObject||s->type!=TTJSONObject||tt_json_get(q,"source")) {
        tt_json_free(q);tt_json_free(s);return 1;
    }
    size_t used=q->source_length-1, need=used+(q->count?1:0)+9+s->source_length+1;
    if(need>capacity){tt_json_free(q);tt_json_free(s);return 2;}
    memcpy(out,q->source,used);
    if(q->count)out[used++]=',';
    memcpy(out+used,"\"source\":",9);used+=9;
    memcpy(out+used,s->source,s->source_length);used+=s->source_length;
    out[used++]='}';*length=used;
    tt_json_free(q);tt_json_free(s);return 0;
}
