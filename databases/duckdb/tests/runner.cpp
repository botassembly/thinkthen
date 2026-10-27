#include "duckdb.hpp"
#include "duckdb/main/stream_query_result.hpp"
#include "yyjson.hpp"
#include "duckdb/parser/parsed_data/create_scalar_function_info.hpp"
#include <cstdlib>
#include <iostream>
#include <memory>
#include <thread>
#include <mutex>
#include <set>
#ifndef _WIN32
#include <sys/resource.h>
#endif
using namespace duckdb;
using namespace duckdb_yyjson;
template <class Read>
static void PrintRows(const vector<string> &names, idx_t count, Read read, const string &error = "") {
    auto doc = yyjson_mut_doc_new(nullptr);
    auto root = yyjson_mut_obj(doc);
    yyjson_mut_doc_set_root(doc, root);
    if (!error.empty()) yyjson_mut_obj_add_strcpy(doc, root, "error", error.c_str());
    else {
        auto rows = yyjson_mut_arr(doc);
        yyjson_mut_obj_add_val(doc, root, "rows", rows);
        for (idx_t i = 0; i < count; ++i) {
            auto row = yyjson_mut_obj(doc);
            yyjson_mut_arr_add_val(rows, row);
            for (idx_t j = 0; j < names.size(); ++j) {
                auto value = read(j, i);
                auto item = value.IsNull() ? yyjson_mut_null(doc) : yyjson_mut_strcpy(doc, value.ToString().c_str());
                yyjson_mut_obj_add_val(doc, row, names[j].c_str(), item);
            }
        }
    }
    auto text = yyjson_mut_write(doc, 0, nullptr);
    std::cout << text << std::endl;
    std::free(text); yyjson_mut_doc_free(doc);
}
static void Print(MaterializedQueryResult &result) {
    PrintRows(result.names, result.HasError() ? 0 : result.RowCount(),
              [&](idx_t column, idx_t row) { return result.GetValue(column, row); },
              result.HasError() ? result.GetError() : "");
}
static void CloseStream(StreamQueryResult *result) {
    result->Close();
    delete result;
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
#ifndef _WIN32
        if (argc > 2 && string(argv[2]) == "--rank-memory") {
            connection.Query("SET thinkthen_backend='stub'");
            connection.Query("SET threads=1");
            auto peak = []() {
                rusage usage{};
                if (getrusage(RUSAGE_SELF, &usage)) throw std::runtime_error("cannot read resident memory");
#ifdef __APPLE__
                return static_cast<uint64_t>(usage.ru_maxrss);
#else
                return static_cast<uint64_t>(usage.ru_maxrss) * 1024;
#endif
            };
            auto before = peak();
            string source = "SELECT range AS document_id,range::VARCHAR AS record";
            string mismatch = "status!='answered' OR rank IS NULL OR document_id+1!=position";
            for (int column = 0; column < 18; ++column) {
                auto name = "column_" + std::to_string(column);
                source += ",range+" + std::to_string(column) + " AS " + name;
                mismatch += " OR " + name + "!=document_id+" + std::to_string(column);
            }
            auto result = connection.Query("SELECT count(*),count(*) FILTER (WHERE " + mismatch + ") FROM thinkthen_rank((" +
                source + " FROM range(100000)), 'Order?', levels := ['low','medium','high'])");
            if (result->HasError()) { Print(*result); return 1; }
            std::cout << "{\"rows\":" << result->GetValue(0, 0).ToString() << ",\"misplaced\":" <<
                result->GetValue(1, 0).ToString() << ",\"growth_bytes\":" << peak() - before << "}" << std::endl;
            return 0;
        }
#endif
        if (argc > 2 && string(argv[2]) == "--cold") {
            connection.Query("SET thinkthen_backend='stub'");
            auto setting = connection.Query("SET threads=4");
            if (setting->HasError()) { Print(*setting); return 1; }
            auto result = connection.Query("SELECT * FROM thinkthen_decide((SELECT 'evidence' AS record), 'Ready?') "
                "UNION ALL SELECT * FROM thinkthen_decide((SELECT 'evidence' AS record), 'Ready?')");
            Print(*result);
            return result->HasError() ? 1 : 0;
        }
        if (argc > 2 && string(argv[2]) == "--abandon") {
            auto setting = connection.Query("SET thinkthen_backend='stub'");
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
            for (auto sql : {"SET threads=4", "SET thinkthen_backend='stub'",
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
            std::thread work([&] { auto result = connection.Query(argc > 3 ? argv[3] : "SELECT * FROM thinkthen_decide((SELECT 'evidence' AS record), 'Ready?')"); Print(*result); });
            if (std::cin.get() != 'c') return 2;
            connection.Interrupt(); work.join();
            auto recovered = connection.Query("SELECT 1 AS alive");
            return recovered->HasError() ? 1 : 0;
        }
        Connection second(database);
        string query;
        while (std::getline(std::cin, query)) {
            std::unique_ptr<yyjson_doc, decltype(&yyjson_doc_free)> message(yyjson_read(query.data(), query.size(), 0), yyjson_doc_free);
            if (!message) return 2;
            auto first = yyjson_obj_get(yyjson_doc_get_root(message.get()), "first");
            if (first) {
                auto result = connection.SendQuery(yyjson_get_str(first));
                if (result->HasError()) throw std::runtime_error(result->GetError());
                if (result->type != QueryResultType::STREAM_RESULT) throw std::runtime_error("expected a streamed result");
                std::unique_ptr<StreamQueryResult, decltype(&CloseStream)> stream(
                    static_cast<StreamQueryResult *>(result.release()), CloseStream);
                auto names = stream->names;
                auto chunk = stream->Fetch();
                stream.reset();
                auto recovered = connection.Query("SELECT 1 AS alive");
                if (recovered->HasError()) throw std::runtime_error(recovered->GetError());
                PrintRows(names, chunk ? chunk->size() : 0,
                          [&](idx_t column, idx_t row) { return chunk->GetValue(column, row); });
                continue;
            }
            auto concurrent = yyjson_obj_get(yyjson_doc_get_root(message.get()), "concurrent");
            if (concurrent) {
                if (yyjson_arr_size(concurrent) != 2) return 2;
                auto first = yyjson_arr_get(concurrent, 0), other = yyjson_arr_get(concurrent, 1);
                if (!yyjson_is_str(first) || !yyjson_is_str(other)) return 2;
                unique_ptr<MaterializedQueryResult> left, right;
                std::thread worker([&] { left = connection.Query(yyjson_get_str(first)); });
                right = second.Query(yyjson_get_str(other));
                worker.join();
                Print(*left); Print(*right);
                continue;
            }
            if (!yyjson_is_str(yyjson_doc_get_root(message.get()))) return 2;
            auto value = yyjson_doc_get_root(message.get());
            query.assign(yyjson_get_str(value), yyjson_get_len(value));
            auto result = query.rfind("2:", 0) == 0 ? second.Query(query.substr(2)) : connection.Query(query);
            Print(*result);
        }
        return 0;
    } catch (const std::exception &error) { std::cerr << error.what() << '\n'; return 1; }
}
