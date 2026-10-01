#include <assert.h>
#include <stdlib.h>
#include <string.h>
#include "ThinkThen.h"
#include "TTJSON.h"

int main(void) {
    TTClient *client = [TTClient create];
    TTFailure failure = {0};

    const char *which_team =
        "{\"choose\": \"Which team owns this?\", "
        "\"options\": {"
        "\"billing\": \"Invoices, fees, and refunds.\", "
        "\"shipping\": \"Parcels and delivery.\", "
        "\"account\": \"Logins and passwords.\"}, "
        "\"evidence\": "
        "\"My parcel went to the wrong address.\"}";
    char *owner = [client json:which_team
                      deadline:-1
                         token:nil
                       failure:&failure];
    assert(owner);
    TTJSON *envelope =
        tt_json_parse(owner, strlen(owner));
    const TTJSON *team = tt_json_get(envelope, "value");
    assert(strcmp(team->text, "shipping") == 0);
    tt_json_free(envelope);
    free(owner);

    tt_failure_clear(&failure);
    [client dealloc];
    return 0;
}
