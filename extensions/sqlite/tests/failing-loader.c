#include <sqlite3ext.h>
#include <dlfcn.h>
#include <stdlib.h>

/* Interpose the host boundary without adding a fault switch to the product. */
SQLITE_EXTENSION_INIT1
static unsigned registrations;
static int fail_module(sqlite3 *db, const char *name, const sqlite3_module *module,
                       void *context, void (*destroy)(void *)) {
    if (module && ++registrations == 2) {
        destroy(context);
        return SQLITE_ERROR;
    }
    return sqlite3_api->create_module_v2(db, name, module, context, destroy);
}
static int fail_function(sqlite3 *db, const char *name, int argc, int flags, void *context,
                         void (*function)(sqlite3_context *,int,sqlite3_value **),
                         void (*step)(sqlite3_context *,int,sqlite3_value **),
                         void (*final)(sqlite3_context *),void (*destroy)(void *)) {
    (void)db; (void)name; (void)argc; (void)flags; (void)function; (void)step; (void)final;
    destroy(context);
    return SQLITE_ERROR;
}
__attribute__((visibility("default")))
int sqlite3_thinkthensqlite_init(sqlite3 *db,char **error,const sqlite3_api_routines *api) {
    SQLITE_EXTENSION_INIT2(api);
    void *library=dlopen(getenv("THINKTHEN_FAILURE_EXTENSION"),RTLD_NOW|RTLD_LOCAL);
    if(!library) return SQLITE_ERROR;
    int (*initialize)(sqlite3 *,char **,const sqlite3_api_routines *)=
        (int (*)(sqlite3 *,char **,const sqlite3_api_routines *))dlsym(library,"sqlite3_thinkthensqlite_init");
    sqlite3_api_routines wrapped=*api;
    registrations=0;
    if(getenv("THINKTHEN_FAIL_FUNCTION")) wrapped.create_function_v2=fail_function;
    else wrapped.create_module_v2=fail_module;
    int result=initialize?initialize(db,error,&wrapped):SQLITE_ERROR;
    dlclose(library);
    return result;
}
