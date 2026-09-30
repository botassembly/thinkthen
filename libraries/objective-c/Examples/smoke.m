/* The replay smoke (ticket 0335): one decide through [TTClient create], which
 * reads the environment, with the question and text sdlc/scripts/smoke names. */
#include "ThinkThen.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int main(void) {
 const char *q=getenv("THINKTHEN_TEST_SMOKE_QUESTION"),*text=getenv("THINKTHEN_TEST_SMOKE_TEXT");
 TTClient *client=[TTClient create]; if(!client||!q||!text)return 1;
 TTFailure f={0};TTDecision answer={123,-1};char *owned=NULL;
 if([client decideBytes:q questionLength:strlen(q) text:text length:strlen(text) deadline:-1 token:nil answer:&answer facts:&owned failure:&f])return 2;
 free(owned);[client dealloc];
 printf("smoke: %s\n",answer.outcome==TTOutcomeYes?"true":answer.outcome==TTOutcomeNo?"false":"null");return 0;
}
