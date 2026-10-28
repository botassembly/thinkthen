/* Feature: C streams preserve every position under shared loopback faults.
 * Each worker owns its call and rows; only the client is shared. */
#include <pthread.h>
#include <stdlib.h>

typedef struct {
    thinkthen_client *client;
    size_t count;
    const char *state, *code;
    double probability;
    int cancel;
    thinkthen_call *call;
    size_t failed, offset;
    uint64_t identifier_width_mask;
} stress_work;

static void *stress_drive(void *raw) {
    stress_work *work = raw;
    thinkthen_error error = {.struct_size = sizeof error};
    size_t sent = 0, received = 0, failed = 0;
    int32_t status;
    do {
        thinkthen_row row = {.struct_size = sizeof row};
        status = thinkthen_call_next(work->call, &row, &error);
        if (status == THINKTHEN_NEEDS_INPUT) {
            if (sent == work->count) thinkthen_call_end_input(work->call);
            else {
                char record[96];
                int n = snprintf(record, sizeof record, "{\"id\":\"record-%zu\",\"body\":\"record %zu\"}", work->offset + sent, work->offset + sent);
                assert(n > 0 && (size_t)n < sizeof record);
                int32_t admitted = thinkthen_call_send_json(work->call, (const uint8_t *)record, (size_t)n, &error);
                if (admitted == THINKTHEN_OK) ++sent;
                else {
                    /* Cancellation can arrive after the input slot was reserved. */
                    assert(work->cancel && admitted == THINKTHEN_ERROR);
                    assert(strcmp(thinkthen_code_name(error.code), "local.cancelled") == 0);
                    thinkthen_error_free(&error);
                }
            }
        } else if (status == THINKTHEN_OK) {
            assert(row.position == ++received);
            work->identifier_width_mask |= UINT64_C(1) << (row.id.len - 2);
            char id[64];
            int n = snprintf(id, sizeof id, "\"record-%zu\"", work->offset + received - 1);
            assert(row.id.len == (size_t)n && memcmp(row.id.data, id, (size_t)n) == 0);
            if (row.status == THINKTHEN_STATUS_FAILED) {
                ++failed;
                assert(strcmp(thinkthen_code_name(row.error_code), work->code) == 0);
            } else {
                assert(row.status == THINKTHEN_STATUS_ANSWERED);
                assert(row.answer.len == 4 && memcmp(row.answer.data, "true", 4) == 0);
                assert(row.probability == work->probability);
            }
        }
        thinkthen_row_free(&row);
    } while (status == THINKTHEN_OK || status == THINKTHEN_NEEDS_INPUT);
    if (work->cancel) {
        assert(status == THINKTHEN_ERROR && received == 0);
        assert(strcmp(thinkthen_code_name(error.code), "local.cancelled") == 0);
    } else {
        assert(status == THINKTHEN_END && received == work->count);
        work->failed = failed;
    }
    thinkthen_error_free(&error);
    return NULL;
}
static int stress_main(int argc, char **argv) {
    assert(argc == 10);
    thinkthen_client *client = NULL;
    thinkthen_error error = {.struct_size = sizeof error};
    assert(thinkthen_client_open_json((const uint8_t *)argv[2], strlen(argv[2]), &client, &error) == THINKTHEN_OK);
    int cancel = strcmp(argv[1], "stress-cancel") == 0;
    size_t workers = cancel ? 1 : strtoull(argv[7], NULL, 10);
    assert(workers > 0);
    stress_work *work = calloc(workers, sizeof *work);
    pthread_t *threads = calloc(workers, sizeof *threads);
    assert(work != NULL && threads != NULL);
    size_t failed = 0;
    uint64_t widths = 0;
    size_t rounds = strtoull(argv[8], NULL, 10);
    for (size_t round = 0; round < rounds; ++round) {
        for (size_t i = 0; i < workers; ++i) {
            work[i] = (stress_work){client, strtoull(argv[3], NULL, 10), argv[4], argv[5], strtod(argv[6], NULL), cancel, NULL, 0, strtoull(argv[9], NULL, 10) + (round * workers + i) * strtoull(argv[3], NULL, 10), 0};
            thinkthen_question *q = NULL;
            assert(thinkthen_question_new(THINKTHEN_FUNCTION_DECIDE, (const uint8_t *)"Ready?", 6, &q, &error) == THINKTHEN_OK);
            assert(thinkthen_question_set_evidence(q, (const uint8_t *)"/body", 5, &error) == THINKTHEN_OK);
            assert(thinkthen_call_open_question(client, q, NULL, THINKTHEN_OFFSET_CODE_POINTS, &work[i].call, &error) == THINKTHEN_OK);
            thinkthen_question_free(q);
            assert(pthread_create(&threads[i], NULL, stress_drive, &work[i]) == 0);
        }
        if (cancel) {
            assert(getchar() == 'c');
            thinkthen_call_cancel(work[0].call);
        }
        for (size_t i = 0; i < workers; ++i) {
            assert(pthread_join(threads[i], NULL) == 0);
            failed += work[i].failed;
            widths |= work[i].identifier_width_mask;
            thinkthen_call_free(work[i].call);
        }
        const char *hook = getenv("THINKTHEN_STRESS_CHECKPOINT_COMMAND");
        if (hook != NULL) assert(system(hook) == 0);
    }
    thinkthen_client_free(client);
    thinkthen_error_free(&error);
    assert(cancel || (strcmp(argv[4], "failed") == 0 ? failed > 0 : failed == 0));
    printf("{\"identifier_width_mask\":%llu,\"calls\":%zu,\"rows\":%zu,\"failed\":%zu,\"cancelled\":%s}\n", (unsigned long long)widths, workers * rounds, cancel ? 0 : workers * rounds * work[0].count, failed, cancel ? "true" : "false");
    free(threads);
    free(work);
    return 0;
}
