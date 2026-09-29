#include "ThinkThen.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int main(void) {
 TTClient *client=[TTClient create]; if(!client)return 1;
 TTFailure f={0};TTDecision answer={123,-1};
 if([client decideBytes:"Is it?" questionLength:6 text:"consumer-objc" length:13 deadline:-1 token:nil answer:&answer failure:&f]||answer.outcome!=TTOutcomeYes)return 2;
 tt_failure_clear(&f);
 char *raw=[client jsonBytes:"{\"decide\":\"Is it?\",\"evidence\":\"consumer-json\"}" length:strlen("{\"decide\":\"Is it?\",\"evidence\":\"consumer-json\"}") deadline:-1 token:nil failure:&f];
 if(!raw)return 3;TTJSON *root=tt_json_parse(raw,strlen(raw));const TTJSON *v=tt_json_call_value(root);const TTJSON *facts=tt_json_get(root,"facts");if(!v||v->type!=TTJSONBoolean||strcmp(v->text,"true")||!facts||facts->count!=7||strcmp(tt_json_get(facts,"records")->text,"1")||strcmp(tt_json_get(facts,"requests_sent")->text,"1")||strcmp(tt_json_get(facts,"cache_answers")->text,"0")||strcmp(tt_json_get(facts,"model")->text,"jev-1.13.0"))return 4;tt_json_free(root);free(raw);tt_failure_clear(&f);[client dealloc];puts("INSTALLED_OBJC_CONSUMER_PASS");return 0;
}
