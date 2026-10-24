#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <thinkthen_c.h>

int main(void) {
    json_t *settings = json_pack("{s:s}", "adapter", "stub");
    json_t *question = json_pack("{s:s,s:s}",
                                "type", "inline", "value", "Ready?");
    tt_client *client = NULL;
    tt_call *call = NULL;
    tt_row *row = NULL;
    tt_error *error = NULL;
    int result = EXIT_FAILURE;
    bool sent = false;
    if (!settings || !question) goto cleanup;
    client = tt_client_new(settings, &error);
    if (!client) goto cleanup;
    call = tt_decide(client, question, NULL, &error);
    if (!call) goto cleanup;
    for (;;) {
        switch (tt_next(call, &row, &error)) {
        case TT_INPUT:
            if (sent) {
                tt_end_input(call);
            } else {
                const char text[] = "evidence";
                if (!tt_send_text(call, (tt_text){text, sizeof text - 1}, &error))
                    goto cleanup;
                sent = true;
            }
            break;
        case TT_ROW:
            if (tt_row_status(row) == TT_ANSWERED) {
                bool answer;
                if (!tt_row_verdict(row, &answer)) goto cleanup;
                puts(answer ? "yes" : "no");
            } else if (tt_row_status(row) == TT_UNSURE)
                puts("not sure");
            else {
                tt_text message = tt_error_message(tt_row_error(row));
                fwrite(message.data, 1, message.len, stderr);
                fputc('\n', stderr);
            }
            tt_row_free(row);
            row = NULL;
            break;
        case TT_DONE:
            result = EXIT_SUCCESS;
            goto cleanup;
        case TT_ERROR:
            goto cleanup;
        }
    }
cleanup:
    if (error) {
        tt_text message = tt_error_message(error);
        fwrite(message.data, 1, message.len, stderr);
        fputc('\n', stderr);
    }
    tt_row_free(row);
    tt_error_free(error);
    tt_call_free(call);
    tt_client_free(client);
    json_decref(question);
    json_decref(settings);
    return result;
}
