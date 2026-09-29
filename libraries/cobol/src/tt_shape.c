/* MIT License; see ../LICENSE. C-only, dependency-free JSON grammar adapter.
 * A parsed object is never mutated: map descriptions cross unchanged.
 * Functions accept byte length and never read past the caller's storage. */
#include "TTJSON.h"
#include <errno.h>
#include <math.h>
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
static int count(const TTJSON *n) {
    if (!n || n->type!=TTJSONNumber || !n->text || !n->text_length) return 0;
    for(size_t i=0;i<n->text_length;i++) if(n->text[i]<'0'||n->text[i]>'9') return 0;
    errno=0;char *end=NULL;strtoull(n->text,&end,10);
    return !errno && end && !*end;
}
/* A COBOL caller owns copied JSON bytes; reject invalid required/optional facts first. */
int tt_cobol_facts(const char *json,size_t length) {
    TTJSON *n=tt_json_parse(json,length);
    int ok=n && n->type==TTJSONObject && n->count>=4 && n->count<=7;
    if(ok){
        ok=count(tt_json_get(n,"records")) && count(tt_json_get(n,"requests_sent")) &&
           count(tt_json_get(n,"cache_answers"));
        const TTJSON *seconds=tt_json_get(n,"seconds");
        if(!seconds||seconds->type!=TTJSONNumber||!seconds->text)ok=0;
        else{errno=0;char *end=NULL;double value=strtod(seconds->text,&end);
            if(errno||!end||*end||!isfinite(value)||value<0)ok=0;}
        const TTJSON *input=tt_json_get(n,"input_tokens"),*output=tt_json_get(n,"output_tokens"),
                     *model=tt_json_get(n,"model");
        if(input&&!count(input))ok=0;
        if(output&&!count(output))ok=0;
        if(model&&model->type!=TTJSONString)ok=0;
        for(size_t i=0;i<n->count;i++){
            const char *key=n->keys[i];size_t len=n->key_lengths[i];
            if(!((len==7&&!memcmp(key,"records",7))||(len==13&&!memcmp(key,"requests_sent",13))||
                 (len==13&&!memcmp(key,"cache_answers",13))||(len==7&&!memcmp(key,"seconds",7))||
                 (len==12&&!memcmp(key,"input_tokens",12))||(len==13&&!memcmp(key,"output_tokens",13))||
                 (len==5&&!memcmp(key,"model",5))))ok=0;
        }
    }
    tt_json_free(n);return ok?0:1;
}
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
/* 1=unresolved null; 2=failed marker; 3=resolved; 0=invalid.
 * Parser enforces structural keys before categorizing a field. */
int tt_cobol_field(const char *json,size_t length) {
    TTJSON *n=tt_json_parse(json,length);if(!n)return 0;
    int result=0;
    if(n->type==TTJSONNull)result=1;
    else if(n->type==TTJSONObject){
        TTJSON *row=tt_json_parse(json,length);
        if(row){ /* reuse the annotate structural validator on a one-field row */
            TTJSON wrapper={.type=TTJSONObject,.count=1};
            char *key="field";size_t key_length=5;TTJSON *child=row;wrapper.keys=&key;wrapper.key_lengths=&key_length;wrapper.children=&child;
            if(tt_json_answer_shape(&wrapper,"annotate"))result=2;
            tt_json_free(row);
        }
    } else {
        TTJSON wrapper={.type=TTJSONObject,.count=1};char *key="field";size_t key_length=5;TTJSON *child=n;
        wrapper.keys=&key;wrapper.key_lengths=&key_length;wrapper.children=&child;
        if(tt_json_answer_shape(&wrapper,"annotate"))result=3;
    }
    tt_json_free(n);return result;
}
int tt_cobol_shape(const char *json,size_t length,const char *kind) {
    TTJSON *n=tt_json_parse(json,length);if(!n)return 0;
    int ok=tt_json_answer_shape(n,kind);tt_json_free(n);return ok;
}
