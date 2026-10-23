#include "duckdb.hpp"
#include "yyjson.hpp"
#include "duckdb/parser/parsed_data/create_scalar_function_info.hpp"
#include <cstdlib>
#include <iostream>
#include <memory>
#include <thread>
#include <mutex>
#include <set>
using namespace duckdb;
using namespace duckdb_yyjson;
static void Print(MaterializedQueryResult &result) {
    auto doc = yyjson_mut_doc_new(nullptr);
    auto root = yyjson_mut_obj(doc);
    yyjson_mut_doc_set_root(doc, root);
    if (result.HasError()) yyjson_mut_obj_add_strcpy(doc, root, "error", result.GetError().c_str());
    else {
        auto rows = yyjson_mut_arr(doc);
        yyjson_mut_obj_add_val(doc, root, "rows", rows);
        for (idx_t i = 0; i < result.RowCount(); ++i) {
            auto row = yyjson_mut_obj(doc);
            yyjson_mut_arr_add_val(rows, row);
            for (idx_t j = 0; j < result.ColumnCount(); ++j) {
                auto value = result.GetValue(j, i);
                auto item = value.IsNull() ? yyjson_mut_null(doc) : yyjson_mut_strcpy(doc, value.ToString().c_str());
                yyjson_mut_obj_add_val(doc, row, result.names[j].c_str(), item);
            }
        }
    }
    auto text = yyjson_mut_write(doc, 0, nullptr);
    std::cout << text << std::endl;
    std::free(text); yyjson_mut_doc_free(doc);
}
int main(int argc, char **argv) {
    try {
        if (argc < 2) return 2;
        if (string(argv[1]) == "--platform") { std::cout << DuckDB::Platform(); return 0; }
        DBConfig config;
        config.SetOptionByName("allow_unsigned_extensions", Value::BOOLEAN(true));
        DuckDB database(nullptr, &config);
        Connection connection(database);
        auto load = connection.Query("LOAD '" + string(argv[1]) + "'");
        if (load->HasError()) { Print(*load); return 1; }
        if (argc > 2 && string(argv[2]) == "--cold") {
            auto setting = connection.Query("SET threads=4");
            if (setting->HasError()) { Print(*setting); return 1; }
            auto result = connection.Query("SELECT * FROM thinkthen_decide((SELECT 'evidence' AS record), 'Ready?') "
                "UNION ALL SELECT * FROM thinkthen_decide((SELECT 'evidence' AS record), 'Ready?')");
            Print(*result);
            return result->HasError() ? 1 : 0;
        }
        if (argc > 2 && string(argv[2]) == "--abandon") {
            auto setting = connection.Query("SET thinkthen_settings='{\"adapter\":\"stub\"}'");
            if (setting->HasError()) { Print(*setting); return 1; }
            auto result = connection.SendQuery("SELECT * FROM thinkthen_decide((SELECT range::VARCHAR AS record FROM range(8193)), 'Ready?')");
            if (result->HasError() || result->type != QueryResultType::STREAM_RESULT) return 1;
            auto chunk = result->Fetch();
            if (!chunk || chunk->size() == 0 || chunk->size() >= 8193) return 1;
            result.reset();
            // The connection and database now close with unread call output.
            return 0;
        }
        if (argc > 2 && string(argv[2]) == "--parallel") {
            std::mutex mutex;
            std::set<std::thread::id> threads;
            ScalarFunction mark("record_thread", {LogicalType::BIGINT}, LogicalType::BIGINT,
                [&](DataChunk &input, ExpressionState &, Vector &output) {
                    { std::lock_guard<std::mutex> lock(mutex); threads.insert(std::this_thread::get_id()); }
                    output.Reference(input.data[0]);
                });
            CreateScalarFunctionInfo info(mark); connection.context->RegisterFunction(info);
            for (auto sql : {"SET threads=4", "SET thinkthen_settings='{\"adapter\":\"stub\"}'",
                            "CREATE TABLE evidence AS SELECT range AS i FROM range(500000)"}) {
                auto result = connection.Query(sql);
                if (result->HasError()) { Print(*result); return 1; }
            }
            if (argc != 4) return 2;
            auto result = connection.Query(argv[3]);
            Print(*result);
            if (result->HasError() || threads.size() < 2) return 1;
            std::cerr << "DuckDB parallel source threads=" << threads.size() << '\n';
            return 0;
        }
        if (argc > 2) {
            std::thread work([&] { auto result = connection.Query("SELECT * FROM thinkthen_decide((SELECT 'evidence' AS record), 'Ready?')"); Print(*result); });
            if (std::cin.get() != 'c') return 2;
            connection.Interrupt(); work.join(); return 0;
        }
        Connection second(database);
        string query;
        while (std::getline(std::cin, query)) {
            std::unique_ptr<yyjson_doc, decltype(&yyjson_doc_free)> message(yyjson_read(query.data(), query.size(), 0), yyjson_doc_free);
            if (!message || !yyjson_is_str(yyjson_doc_get_root(message.get()))) return 2;
            auto value = yyjson_doc_get_root(message.get());
            query.assign(yyjson_get_str(value), yyjson_get_len(value));
            auto result = query.rfind("2:", 0) == 0 ? second.Query(query.substr(2)) : connection.Query(query);
            Print(*result);
        }
        return 0;
    } catch (const std::exception &error) { std::cerr << error.what() << '\n'; return 1; }
}
