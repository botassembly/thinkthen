#include "tt_requests_generated.h"
#include <assert.h>
#include <string.h>

int TT_TEST_VALUE_OWNERS(void) {
    tt_value *number=NULL,*empty=NULL,*false_value=NULL,*text=NULL,*object=NULL,*copy=NULL,*field=NULL;
    const char decimal[]="123456789012345678901234567890";
    const char bytes[]={'a','\0','b'};
    assert(TT_VALUE_NUMBER(decimal,sizeof(decimal)-1,&number)==0);
    assert(TT_VALUE_NULL(&empty)==0);
    assert(TT_VALUE_BOOL(0,&false_value)==0);
    assert(TT_VALUE_STRING(bytes,sizeof(bytes),&text)==0);
    const thinkthen_cobol_text names[]={{"count",5},{"empty",5},{"selected",8},{"text",4}};
    const tt_value *values[]={number,empty,false_value,text};
    assert(TT_VALUE_OBJECT(names,values,4,&object)==0);
    TT_VALUE_FREE(number); TT_VALUE_FREE(empty); TT_VALUE_FREE(false_value); TT_VALUE_FREE(text);
    const tt_value *items[]={object};
    assert(TT_VALUE_ARRAY(items,1,&copy)==0);
    TT_VALUE_FREE(object);
    assert(TT_VALUE_ELEMENT(copy,1,&object)==0);
    TT_VALUE_FREE(copy);
    char output[8192]; uint64_t length=0; uint32_t observed=0;
    assert(TT_VALUE_NAME(object,1,output,&length)==0 && length==5 && !memcmp(output,"count",5));
    assert(TT_VALUE_FIELD(object,"count",5,&field)==0);
    assert(TT_VALUE_TEXT(field,output,&length)==0 && length==sizeof(decimal)-1 && !memcmp(output,decimal,length));
    TT_VALUE_FREE(field);
    assert(TT_VALUE_FIELD(object,"selected",8,&field)==0);
    assert(TT_VALUE_BOOLEAN(field,&observed)==0 && observed==0);
    TT_VALUE_FREE(field);
    assert(TT_VALUE_HAS_FIELD(object,"empty",5,&observed)==0 && observed==1);
    assert(TT_VALUE_FIELD(object,"empty",5,&field)==0);
    assert(TT_VALUE_KIND(field,&observed)==0 && observed==TT_VALUE_NULL_KIND);
    TT_VALUE_FREE(field);
    assert(TT_VALUE_HAS_FIELD(object,"missing",7,&observed)==0 && observed==0);
    field=object;
    assert(TT_VALUE_FIELD(object,"missing",7,&field)!=0 && field==object);
    assert(TT_VALUE_FIELD(object,"text",4,&field)==0);
    TT_VALUE_FREE(object);
    assert(TT_VALUE_TEXT(field,output,&length)==0 && length==3 && !memcmp(output,bytes,3));
    TT_VALUE_FREE(field);
    char large[8193]; memset(large,'x',sizeof(large));
    assert(TT_VALUE_STRING(large,sizeof(large),&field)==0);
    memset(output,'z',sizeof(output)); length=17;
    assert(TT_VALUE_TEXT(field,output,&length)==THINKTHEN_COBOL_OVERFLOW && length==17);
    for(unsigned i=0;i<sizeof(output);i++) assert(output[i]=='z');
    TT_VALUE_FREE(field); TT_VALUE_FREE(NULL);
    return 0;
}
