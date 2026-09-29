/* MIT License; see ../LICENSE. C-only, dependency-free JSON grammar adapter.
 * A parsed object is never mutated: map descriptions cross unchanged.
 * Functions accept byte length and never read past the caller's storage. */
#include "TTJSON.h"
#include <stdio.h>
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
