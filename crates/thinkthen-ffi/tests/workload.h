/* Volume calls through the same public ABI and sanitizer executable. */
typedef struct {
    thinkthen_client *client;
    thinkthen_call *call;
    size_t worker, records, rounds, rows, failed;
    int cancel, answered, refused;
    double probability;
    const char *code;
    pthread_mutex_t *lock;
    pthread_cond_t *start;
    int *ready;
} workload_worker;

static thinkthen_call *workload_call(thinkthen_client *client, size_t worker, int cancel) {
    thinkthen_call *call = NULL;
    thinkthen_error error = {.struct_size = sizeof error};
    const char request[] = "{\"question\":{\"function\":\"decide\",\"question\":\"Ready?\",\"threshold\":0.5,\"evidence\":\"/body\"}}";
    char held[512];
    if (cancel) {
        int len = snprintf(held, sizeof held,
            "{\"question\":{\"function\":\"decide\",\"question\":\"Ready?\"},"
            "\"records\":[{\"id\":\"held-%zu\",\"body\":\"held\"}]}", worker);
        assert(len > 0 && (size_t)len < sizeof held);
    }
    const char *input = cancel ? held : request;
    assert(thinkthen_call_open_json(client, (const uint8_t *)input, strlen(input),
                                   THINKTHEN_OFFSET_UTF8, &call, &error) == THINKTHEN_OK);
    return call;
}

static void workload_record(workload_worker *worker, thinkthen_call *call,
                            size_t round, size_t index) {
    char record[256];
    int len = snprintf(record, sizeof record,
        "{\"id\":\"call-%zu-%zu-record-%zu\",\"body\":\"evidence %zu\"}",
        worker->worker, round, index, index);
    assert(len > 0 && (size_t)len < sizeof record);
    thinkthen_error error = {.struct_size = sizeof error};
    assert(thinkthen_call_send_json(call, (const uint8_t *)record, (size_t)len, &error) == THINKTHEN_OK);
    /* The ABI must own input bytes when send returns. */
    memset(record, 'x', (size_t)len);
}

static void workload_row(workload_worker *worker, thinkthen_row *row, size_t round, size_t index) {
    char id[128];
    int len = snprintf(id, sizeof id, "\"call-%zu-%zu-record-%zu\"", worker->worker, round, index);
    assert(len > 0 && (size_t)len < sizeof id);
    assert(row->position == index + 1 && row->id.len == (size_t)len);
    assert(memcmp(row->id.data, id, (size_t)len) == 0);
    if (row->status == THINKTHEN_STATUS_FAILED) {
        assert(!worker->answered);
        assert(strcmp(thinkthen_code_name(row->error_code), worker->code) == 0);
        ++worker->failed;
    } else {
        assert(!worker->refused && row->status == THINKTHEN_STATUS_ANSWERED);
        assert(row->probability == worker->probability);
        /* Faults may affect only an initial batch; remaining rows must answer. */
        assert(row->answer.len == 4 && memcmp(row->answer.data, "true", 4) == 0);
    }
    ++worker->rows;
    thinkthen_row_free(row);
    assert(row->id.data == NULL && row->answer.data == NULL && row->error.data == NULL);
}

static void *run_workload(void *opaque) {
    workload_worker *worker = opaque;
    assert(pthread_mutex_lock(worker->lock) == 0);
    while (!*worker->ready) assert(pthread_cond_wait(worker->start, worker->lock) == 0);
    assert(pthread_mutex_unlock(worker->lock) == 0);
    for (size_t round = 0; round < worker->rounds; ++round) {
        thinkthen_call *call = worker->cancel ? worker->call : workload_call(worker->client, worker->worker, 0);
        thinkthen_error error = {.struct_size = sizeof error};
        size_t sent = 0, received = 0;
        if (worker->cancel) {
            thinkthen_call_end_input(call);
        }
        for (;;) {
            thinkthen_row row = {.struct_size = sizeof row};
            int32_t status = thinkthen_call_next(call, &row, &error);
            if (status == THINKTHEN_NEEDS_INPUT) {
                assert(!worker->cancel);
                if (sent < worker->records) workload_record(worker, call, round, sent++);
                else thinkthen_call_end_input(call);
            } else if (status == THINKTHEN_OK) {
                assert(!worker->cancel && received < worker->records);
                workload_row(worker, &row, round, received++);
            } else if (status == THINKTHEN_ERROR) {
                assert(worker->cancel && error.code == THINKTHEN_CODE_LOCAL_CANCELLED);
                thinkthen_error_free(&error);
                assert(thinkthen_call_next(call, &row, &error) == THINKTHEN_END);
                break;
            } else {
                assert(status == THINKTHEN_END && !worker->cancel);
                assert(sent == worker->records && received == sent);
                break;
            }
        }
        if (!worker->cancel) thinkthen_call_free(call);
    }
    return NULL;
}

static void workload(void) {
    char *settings = line(), *parameters = line();
    assert(settings != NULL && parameters != NULL);
    size_t records, concurrency, rounds;
    int cancel, answered, refused;
    double probability;
    char code[128];
    assert(sscanf(parameters, "%zu %zu %zu %d %d %d %lf %127s", &records, &concurrency,
                  &rounds, &cancel, &answered, &refused, &probability, code) == 8);
    assert(records > 0 && concurrency > 0 && rounds > 0 && (!cancel || rounds == 1));
    free(parameters);
    thinkthen_client *client = NULL;
    thinkthen_error error = {.struct_size = sizeof error};
    assert(thinkthen_client_open_json((const uint8_t *)settings, strlen(settings), &client, &error) == THINKTHEN_OK);
    free(settings);
    pthread_t *threads = calloc(concurrency, sizeof *threads);
    workload_worker *workers = calloc(concurrency, sizeof *workers);
    assert(threads != NULL && workers != NULL);
    pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;
    pthread_cond_t start = PTHREAD_COND_INITIALIZER;
    int ready = 0;
    for (size_t index = 0; index < concurrency; ++index) {
        workers[index] = (workload_worker){.client = client, .worker = index,
            .records = records, .rounds = rounds, .cancel = cancel, .answered = answered,
            .refused = refused, .probability = probability, .code = code, .lock = &lock, .start = &start, .ready = &ready};
        if (cancel) workers[index].call = workload_call(client, index, 1);
        assert(pthread_create(&threads[index], NULL, run_workload, &workers[index]) == 0);
    }
    assert(pthread_mutex_lock(&lock) == 0);
    ready = 1;
    assert(pthread_cond_broadcast(&start) == 0);
    assert(pthread_mutex_unlock(&lock) == 0);
    if (cancel) {
        /* The shared runner sends this only after a held HTTP request arrives. */
        assert(getchar() == 'x');
        for (size_t index = 0; index < concurrency; ++index) thinkthen_call_cancel(workers[index].call);
    }
    size_t rows = 0, failed = 0;
    for (size_t index = 0; index < concurrency; ++index) {
        assert(pthread_join(threads[index], NULL) == 0);
        if (cancel) thinkthen_call_free(workers[index].call);
        rows += workers[index].rows;
        failed += workers[index].failed;
    }
    assert(pthread_cond_destroy(&start) == 0);
    assert(pthread_mutex_destroy(&lock) == 0);
    thinkthen_client_free(client);
    free(workers);
    free(threads);
    printf("{\"calls\":%zu,\"rows\":%zu,\"failed\":%zu,\"cancelled\":%s}\n",
           concurrency * rounds, rows, failed, cancel ? "true" : "false");
}
