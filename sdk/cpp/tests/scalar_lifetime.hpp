// Feature: scalar calls release their cursor before returning or throwing.
#pragma once
#include <thinkthen.h>
#include <cassert>
#include <iostream>

namespace scalar_lifetime {
inline const char* operation = nullptr;
inline const thinkthen_client* client = nullptr;
inline thinkthen_call* cursor = nullptr;
inline unsigned opens = 0, frees = 0;
inline void begin(const char* name) {
    assert(!operation);
    operation = name; client = nullptr; cursor = nullptr; opens = frees = 0;
}
inline int32_t open(const thinkthen_client* owner, const thinkthen_question* question,
    const thinkthen_text* name, const thinkthen_record_input* records, size_t count,
    size_t stride, thinkthen_offset offsets, thinkthen_call** out, thinkthen_error* error) {
    const auto result = thinkthen_call_open_records_question(owner, question, name, records,
        count, stride, offsets, out, error);
    if (operation && result == THINKTHEN_OK) {
        assert(++opens == 1);
        client = owner; cursor = *out;
        thinkthen_usage usage{}; usage.struct_size = sizeof usage;
        assert(thinkthen_client_usage_values(client, &usage, nullptr) == THINKTHEN_OK);
        std::cerr << operation << ": opened scalar cursor, active workers " << usage.active_calls << '\n';
    }
    return result;
}
inline void free(thinkthen_call* call) {
    if (operation && call == cursor && call) assert(++frees == 1);
    thinkthen_call_free(call);
}
inline void finish() {
    std::cerr << operation << ": scalar cursor opens " << opens << ", frees " << frees << '\n';
    // Check ownership before waiting: a leaked cursor must fail, not hang settlement.
    assert(opens == 1 && frees == 1 && client && cursor);
    thinkthen_usage usage{}; usage.struct_size = sizeof usage;
    assert(thinkthen_client_usage_values(client, &usage, nullptr) == THINKTHEN_OK);
    std::cerr << operation << ": after free, active workers " << usage.active_calls << '\n';
    // Free and ordinary usage are nonblocking. Only this native event waits for workers.
    assert(thinkthen_client_usage_values_settled(client, &usage, nullptr) == THINKTHEN_OK);
    assert(usage.active_calls == 0 && usage.active_attempts == 0);
    operation = nullptr;
}
}
#define thinkthen_call_open_records_question scalar_lifetime::open
#define thinkthen_call_free scalar_lifetime::free
