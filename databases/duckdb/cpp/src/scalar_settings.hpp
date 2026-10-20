#pragma once

#include "bridge.hpp"
#include "duckdb/main/client_context.hpp"

#include <optional>
#include <string>

namespace duckdb {

struct SessionSettings {
    struct Setting { string name; string value; bool literal; };
    vector<Setting> values;
    std::optional<string> batch;
    std::optional<string> model;
    int64_t total = INT64_MIN;
    int32_t cache_allowed = 1;
    int32_t record_allowed = 1;
    int32_t replay_allowed = 1;
    struct BridgeValues {
        vector<ThinkThenSetting> values;
        ThinkThenSettings settings;
        operator ThinkThenSettings() const {
            auto result = settings;
            result.settings = values.data();
            result.count = values.size();
            return result;
        }
    };
    BridgeValues Bridge() const {
        BridgeValues result;
        for (const auto &value : values) {
            result.values.push_back({reinterpret_cast<const uint8_t *>(value.name.data()), value.name.size(),
                reinterpret_cast<const uint8_t *>(value.value.data()), value.value.size(), value.literal ? 1 : 0});
        }
        const auto bytes = [](const std::optional<string> &value) {
            return value ? reinterpret_cast<const uint8_t *>(value->data()) : nullptr;
        };
        result.settings = {nullptr, 0, bytes(batch), batch ? batch->size() : 0, total,
            bytes(model), model ? model->size() : 0, cache_allowed, record_allowed, replay_allowed};
        return result;
    }
};

void ValidateRequestTotal(ClientContext &context);
SessionSettings Settings(ClientContext &context);

} // namespace duckdb
