#include <assert.h>
#include <stdlib.h>
#include <string.h>
#include "ThinkThen.h"
#include "TTJSON.h"

int main(void) {
    TTClient *client = [TTClient create];
    TTFailure failure = {0};

    const char *singer_song =
        "{\"version\": 1, \"relate\": {\"relations\": [{"
        "\"name\": \"sings\", \"source\": \"singer\", "
        "\"target\": \"song\"}]}}";
    const char *beatles[] = {
        "{\"name\": \"Paul McCartney\", "
        "\"kind\": \"singer\"}",
        "{\"name\": \"Ringo Starr\", \"kind\": \"singer\"}",
        "{\"name\": \"Yesterday\", \"kind\": \"song\"}",
        "{\"name\": \"Octopus's Garden\", "
        "\"kind\": \"song\"}",
    };
    size_t lengths[4];
    for (int i = 0; i < 4; i++) {
        lengths[i] = strlen(beatles[i]);
    }
    char *sings = NULL;
    size_t sings_length = 0;
    char *facts = NULL;
    TTErrorKind sings_error =
        [client relate:singer_song
                 texts:beatles
               lengths:lengths
                 count:4
                result:&sings
                  size:&sings_length
              deadline:-1
                 token:nil
                 facts:&facts
               failure:&failure];
    assert(sings_error == TTErrorNone);
    TTJSON *found = tt_json_parse(sings, sings_length);
    const TTJSON *edges = tt_json_get(found, "edges");
    const char *singers[] = {
        "Paul McCartney", "Ringo Starr",
    };
    const char *songs[] = {"Yesterday", "Octopus's Garden"};
    assert(edges->count == 2);
    for (size_t i = 0; i < 2; i++) {
        const TTJSON *edge = edges->children[i];
        const TTJSON *singer = tt_json_get(
            tt_json_get(edge, "source"), "name");
        const TTJSON *song = tt_json_get(
            tt_json_get(edge, "target"), "name");
        assert(strcmp(singer->text, singers[i]) == 0);
        assert(strcmp(song->text, songs[i]) == 0);
    }
    tt_json_free(found);
    free(sings);
    free(facts);

    tt_failure_clear(&failure);
    [client dealloc];
    return 0;
}
