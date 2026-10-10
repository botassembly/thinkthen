#pragma once

#include "duckdb/main/extension/extension_loader.hpp"
#include "duckdb/parser/parsed_data/create_scalar_function_info.hpp"
#include "duckdb/parser/parsed_data/create_table_function_info.hpp"

namespace duckdb {

template <class Info>
void DescribeRegistration(ExtensionLoader &loader, Info info, const string &purpose) {
    info.on_conflict = OnCreateConflict::ALTER_ON_CONFLICT;
    for (const auto &function : info.functions.functions) {
        FunctionDescription description;
        description.parameter_types = function.arguments;
        description.description = purpose;
        info.descriptions.push_back(std::move(description));
    }
    loader.RegisterFunction(std::move(info));
}

template <class Function>
void RegisterDescribedScalar(ExtensionLoader &loader, Function function, const string &purpose) {
    DescribeRegistration(loader, CreateScalarFunctionInfo(std::move(function)), purpose);
}

template <class Function>
void RegisterDescribedTable(ExtensionLoader &loader, Function function, const string &purpose) {
    DescribeRegistration(loader, CreateTableFunctionInfo(std::move(function)), purpose);
}

} // namespace duckdb
