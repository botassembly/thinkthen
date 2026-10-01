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
        "\"records\": ["
        "\"Please refund the extra fee on my invoice.\", "
        "\"My parcel went to the wrong address.\", "
        "\"I cannot reset my password.\"]}";
    char *owners = [client json:which_team
                       deadline:-1
                          token:nil
                        failure:&failure];
    assert(owners);
    TTJSON *envelope =
        tt_json_parse(owners, strlen(owners));
    const TTJSON *teams = tt_json_get(envelope, "value");
    assert(teams->count == 3);
    TTJSON **team = teams->children;
    assert(strcmp(team[0]->text, "billing") == 0);
    assert(strcmp(team[1]->text, "shipping") == 0);
    assert(strcmp(team[2]->text, "account") == 0);
    tt_json_free(envelope);
    free(owners);

    tt_failure_clear(&failure);
    [client dealloc];
    return 0;
}
