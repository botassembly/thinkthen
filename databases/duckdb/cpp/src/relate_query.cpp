#include "relate_query.hpp"
#include "bridge.hpp"
#include "scalar_owner.hpp"
#include "duckdb/catalog/catalog.hpp"
#include "duckdb/parser/sql_statement.hpp"

#include <condition_variable>
#include <map>
#include <thread>

namespace duckdb {
namespace {

std::mutex registry_lock;
std::map<DatabaseInstance *, std::weak_ptr<RelateDatabase>> registry;

string Failed(const string &message) {
	const auto at = message.find("thinkthen ");
	return at == string::npos ? "thinkthen usage: the relate query failed: " + message : message.substr(at);
}

string Error(const string &message) {
	return "thinkthen usage: " + message;
}

string Boundary(ClientContext &caller, const string &raw) {
	const auto message = Failed(raw);
	const string marker = "thinkthen usage: the relate query failed: Catalog Error: Table with name ";
	if (message.rfind(marker, 0) != 0) { return message; }
	const auto end = message.find(" does not exist", marker.size());
	if (end == string::npos) { return message; }
	auto name = message.substr(marker.size(), end - marker.size());
	while (!name.empty() && (name.front() == '"' || name.front() == ' ')) { name.erase(name.begin()); }
	while (!name.empty() && (name.back() == '"' || name.back() == ' ')) { name.pop_back(); }
	if (name.empty()) { return message; }
	try {
		EntryLookupInfo lookup(CatalogType::TABLE_ENTRY, name);
		if (Catalog::GetEntry(caller, "temp", "main", lookup, OnEntryNotFound::RETURN_NULL)) {
			return "thinkthen local: the relate query names the temporary table " + name +
			       ", and relate runs on a separate connection, so it cannot see temporary tables; materialize it (CREATE TABLE ... AS SELECT) or run the query directly";
		}
	} catch (const Exception &) {}
	const auto newline = message.find('\n');
	const auto first = message.substr(0, newline);
	const auto later = newline == string::npos ? string() : message.substr(newline);
	return first + "; relate reads only committed tables on its separate connection; if you created this table in an open transaction, commit it before retrying" + later;
}

unique_ptr<MaterializedQueryResult> Query(Connection &connection, ClientContext &caller, const string &sql) {
	auto result = connection.Query(sql);
	if (!result || result->HasError()) {
		throw OrdinaryError("%s", Boundary(caller, result ? result->GetError() : "the query returned no result").c_str());
	}
	return result;
}

struct ResetQuery {
	Connection &connection;
	~ResetQuery() {
		try { connection.Query("ROLLBACK"); } catch (...) {}
		try { connection.Query("RESET search_path"); } catch (...) {}
	}
};

class Limit {
public:
	Limit(Connection &connection, const std::optional<std::chrono::steady_clock::time_point> &at) {
		if (!at) { return; }
		thread = std::thread([this, &connection, at] {
			std::unique_lock<std::mutex> held(lock);
			if (!done.wait_until(held, *at, [this] { return finished; })) {
				fired.store(true, std::memory_order_release);
				connection.Interrupt();
			}
		});
	}
	~Limit() {
		{
			std::lock_guard<std::mutex> held(lock);
			finished = true;
		}
		done.notify_one();
		if (thread.joinable()) { thread.join(); }
	}
	bool Fired() const { return fired.load(std::memory_order_acquire); }
private:
	std::mutex lock;
	std::condition_variable done;
	std::thread thread;
	std::atomic<bool> fired {false};
	bool finished = false;
};

RelateFound QueryRows(Connection &connection, ClientContext &caller, const string &sql, uint64_t holding,
                     const std::optional<string> &search_path) {
	vector<unique_ptr<SQLStatement>> statements;
	try { statements = connection.ExtractStatements(sql); }
	catch (const Exception &error) {
		throw OrdinaryError("%s", Error("the relate query did not parse: " + string(error.what())).c_str());
	}
	if (statements.size() != 1) {
		throw OrdinaryError("thinkthen usage: the relate query is one SQL statement and this one holds %llu; relate reads records, it does not run scripts", static_cast<unsigned long long>(statements.size()));
	}
	const string refused = "thinkthen usage: the relate query must be a SELECT; relate reads records, it does not write files, attach databases, change settings, or load extensions";
	if (statements[0]->type != StatementType::SELECT_STATEMENT) {
		throw OrdinaryError("%s", refused.c_str());
	}
	const auto wrapper = "SELECT * FROM (" + sql + ") AS thinkthen_kind";
	auto prepared = connection.Prepare(wrapper);
	if (prepared->HasError()) {
		const auto message = prepared->GetError();
		throw OrdinaryError("%s", (message.rfind("Parser Error", 0) == 0 ? refused : Boundary(caller, message)).c_str());
	}
	const auto capped = "SELECT COLUMNS(*)::VARCHAR FROM (" + sql + ") AS thinkthen_capped LIMIT 256";
	auto started = connection.Query("BEGIN TRANSACTION READ ONLY");
	if (started->HasError()) {
		throw OrdinaryError("thinkthen usage: the relate query could not start its read-only transaction: %s", started->GetError().c_str());
	}
	ResetQuery reset {connection};
	if (search_path && !search_path->empty()) {
		string quoted;
		for (char ch : *search_path) { quoted += ch; if (ch == '\'') { quoted += '\''; } }
		auto path = connection.Query("SET search_path = '" + quoted + "'");
		if (path->HasError()) {
			throw OrdinaryError("thinkthen usage: the relate query could not run under the calling session's search path %s: %s", search_path->c_str(), path->GetError().c_str());
		}
	}
	auto plan = Query(connection, caller, "EXPLAIN (FORMAT JSON) " + capped);
	if (plan->RowCount() == 0 || plan->ColumnCount() < 2 || plan->GetValue(1, 0).IsNull()) {
		throw OrdinaryError("thinkthen defect: the relate plan came back empty");
	}
	const auto plan_text = plan->GetValue(1, 0).GetValue<string>();
	RustReply checked(thinkthen_cpp_relate_plan(reinterpret_cast<const uint8_t *>(plan_text.data()), plan_text.size(), holding));
	Checked(checked.value);
	auto found = Query(connection, caller, capped);
	RelateFound rows;
	rows.columns = found->ColumnCount();
	for (idx_t row = 0; row < found->RowCount(); ++row) {
		vector<std::optional<string>> values;
		for (idx_t column = 0; column < rows.columns; ++column) {
			auto value = found->GetValue(column, row);
			values.push_back(value.IsNull() ? std::nullopt : std::optional<string>(value.GetValue<string>()));
		}
		rows.rows.push_back(std::move(values));
	}
	return rows;
}

} // namespace

std::shared_ptr<RelateDatabase> HoldRelateDatabase(ClientContext &context) {
	std::lock_guard<std::mutex> held(registry_lock);
	auto key = context.db.get();
	if (auto present = registry[key].lock()) { return present; }
	auto created = std::make_shared<RelateDatabase>(context.db);
	registry[key] = created;
	return created;
}

void InterruptBusyRelate() noexcept {
	try {
		std::lock_guard<std::mutex> held(registry_lock);
		for (auto at = registry.begin(); at != registry.end();) {
			if (auto entry = at->second.lock()) {
				if (entry->busy.load(std::memory_order_acquire)) { entry->connection.Interrupt(); }
				++at;
			} else { at = registry.erase(at); }
		}
	} catch (...) {}
}

bool RelateBusyFor(ClientContext &context) noexcept {
	try {
		std::lock_guard<std::mutex> held(registry_lock);
		auto found = registry.find(context.db.get());
		if (found == registry.end()) { return false; }
		auto entry = found->second.lock();
		return entry && entry->busy.load(std::memory_order_acquire);
	} catch (...) { return true; }
}

RelateFound ReadRelateRows(RelateDatabase &held, ClientContext &caller, const string &sql,
                          uint64_t seconds, uint64_t holding, const std::optional<string> &search_path) {
	if (QueryInterrupted(caller)) {
		throw OrdinaryError("thinkthen cancelled: the call was cancelled");
	}
	if (held.connection.context == caller.shared_from_this()) {
		throw OrdinaryError("thinkthen usage: the relate query calls thinkthen_relate while its own query is running; nested relate cannot run, because the outer query waits on the connection the inner one needs");
	}
	using Clock = std::chrono::steady_clock;
	const auto now = Clock::now();
	const auto room = std::chrono::duration_cast<std::chrono::seconds>(Clock::time_point::max() - now).count();
	const auto at = seconds && seconds <= static_cast<uint64_t>(room)
	                    ? std::optional<Clock::time_point>(now + std::chrono::seconds(seconds))
	                    : std::nullopt;
	std::unique_lock<std::mutex> gate(held.gate, std::defer_lock);
	while (!gate.try_lock()) {
		if (QueryInterrupted(caller)) { throw OrdinaryError("thinkthen cancelled: the call was cancelled"); }
		if (at && Clock::now() >= *at) {
			throw OrdinaryError("thinkthen deadline: the relate query waited past its %llu-second limit in the queue behind another relate on this database and did not run; retry after that relate ends or raise SET thinkthen_relate_seconds (0 turns the limit off)", static_cast<unsigned long long>(seconds));
		}
		std::this_thread::sleep_for(std::chrono::milliseconds(5));
	}
	auto lease = std::make_unique<RelateLease>(held, std::move(gate));
	Limit timer(held.connection, at);
	try {
		auto found = QueryRows(held.connection, caller, sql, holding, search_path);
		if (QueryInterrupted(caller)) { throw OrdinaryError("thinkthen cancelled: the call was cancelled"); }
		if (at) {
			found.remaining_ms = std::chrono::duration_cast<std::chrono::milliseconds>(
			    *at - Clock::now()).count();
			if (found.remaining_ms < 0) { found.remaining_ms = 0; }
		}
		found.lease = std::move(lease);
		return found;
	} catch (const Exception &) {
		if (QueryInterrupted(caller)) { throw OrdinaryError("thinkthen cancelled: the call was cancelled"); }
		if (timer.Fired()) {
			throw OrdinaryError("thinkthen deadline: the relate query ran past its %llu-second limit and was stopped; filter the rows first or raise SET thinkthen_relate_seconds (0 turns the limit off)", static_cast<unsigned long long>(seconds));
		}
		throw;
	}
}

} // namespace duckdb

extern "C" void thinkthen_cpp_interrupt_busy() noexcept {
	duckdb::InterruptBusyRelate();
}
