/* Borrowed immediate diagnostics retain their slot, root and nested backing. */
#include <thinkthen.h>
#include <assert.h>
#include <stdatomic.h>
#include <stdint.h>
#include <string.h>
#include "platform.h"
static atomic_uint ready;
static atomic_bool release_workers;
static THREAD_RESULT worker(void *argument) {
    thinkthen_engine *engine=argument;
    thinkthen_question *out=(thinkthen_question *)(uintptr_t)7;
    assert(thinkthen_error_view(engine)==NULL);
    assert(thinkthen_question_parse(engine,0,(thinkthen_string_v1){"{}",2},&out)==THINKTHEN_EUSAGE);
    assert(out==(thinkthen_question *)(uintptr_t)7);
    const thinkthen_error_view_v1 *view=thinkthen_error_view(engine);
    assert(view && view->code==THINKTHEN_EUSAGE && !view->retryable && !view->facts);
    assert(thinkthen_error_view(NULL)==NULL && thinkthen_session_error_view()==NULL);
    atomic_fetch_add(&ready,1);
    while(!atomic_load(&release_workers)) {}
    assert(thinkthen_error_view(engine)==view);
    return THREAD_DONE;
}
int main(void) {
    binary_streams();
    assert(thinkthen_error_view(NULL)==NULL && thinkthen_session_error_view()==NULL);
    assert(!thinkthen_engine_new_with("{\"unknown\":true}"));
    const thinkthen_error_view_v1 *constructor=thinkthen_error_view(NULL);
    assert(constructor && constructor->code==THINKTHEN_EUSAGE && !constructor->retryable && !constructor->facts);
    uint32_t output=99;
    assert(thinkthen_session_wait(NULL,THINKTHEN_SESSION_WAIT_RESULT_V1,0,&output)==THINKTHEN_EUSAGE && output==99);
    const thinkthen_error_view_v1 *session=thinkthen_session_error_view();
    assert(session && session->code==THINKTHEN_EUSAGE && !session->retryable && !session->facts);
    assert(thinkthen_error_view(NULL)==constructor);
    thinkthen_engine *engine=thinkthen_engine_new_with("{}");
    assert(engine && thinkthen_error_view(NULL)==NULL && thinkthen_error_view(engine)==NULL);
    assert(thinkthen_session_error_view()==session);
    thinkthen_question *question=(thinkthen_question *)(uintptr_t)7;
    assert(thinkthen_question_parse(engine,0,(thinkthen_string_v1){"{}",2},&question)==THINKTHEN_EUSAGE);
    assert(question==(thinkthen_question *)(uintptr_t)7);
    const thinkthen_error_view_v1 *view=thinkthen_error_view(engine);
    assert(view && view->code==thinkthen_error_code(engine));
    assert(view->retryable==thinkthen_error_retryable(engine) && !view->facts);
    const char *message=thinkthen_error_message(engine);
    assert(view->message.len==strlen(message) && !memcmp(view->message.data,message,view->message.len));
    THREAD_TYPE threads[8];
    for(unsigned i=0;i<8;i++) assert(!fixture_start(&threads[i],worker,engine));
    while(atomic_load(&ready)!=8) {}
    assert(thinkthen_error_view(engine)==view && !memcmp(view->message.data,message,view->message.len));
    atomic_store(&release_workers,1);
    for(unsigned i=0;i<8;i++) assert(!fixture_join(threads[i]));
    assert(thinkthen_error_view(engine)==view && thinkthen_session_error_view()==session);
    const char json[]="{\"decide\":\"Does it pass?\"}";
    assert(thinkthen_question_parse(engine,THINKTHEN_LOAD_ATOMIC_V1,(thinkthen_string_v1){json,sizeof(json)-1},&question)==THINKTHEN_OK);
    thinkthen_question_author_v1 author={0};
    assert(thinkthen_question_author(question,&author)==THINKTHEN_OK);
    assert(thinkthen_error_view(engine)==view && thinkthen_session_error_view()==session);
    assert(!thinkthen_engine_new_with("{\"unknown\":false}"));
    assert(thinkthen_error_view(engine)==view && thinkthen_session_error_view()==session);
    thinkthen_question_free(question); thinkthen_engine_free(engine);
    assert(thinkthen_session_error_view()==session);
    return 0;
}
