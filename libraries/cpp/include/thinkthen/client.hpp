#pragma once
#include <thinkthen/thinkthen.h>
#include "results_generated.hpp"
#include "inputs_generated.hpp"
#include <memory>
#include <thread>
#include <chrono>
namespace tt {
enum class NativeFailureKind { usage=THINKTHEN_EUSAGE, backend=THINKTHEN_EBACKEND,
    deadline=THINKTHEN_EDEADLINE, local=THINKTHEN_ELOCAL,
    cancelled=THINKTHEN_ECANCELLED, defect=THINKTHEN_EDEFECT };
class NativeFailure : public std::runtime_error {
public:
    NativeFailureKind kind;
    NativeFailure(int code, const char* message):std::runtime_error(message),kind(static_cast<NativeFailureKind>(code)) {}
};
class SessionFailure : public std::runtime_error {
public:
    results::CallError failure;
    results::SessionPacketTerminal terminal;
    std::vector<results::SessionPacket> packets;
    explicit SessionFailure(results::SessionPacketTerminal value):std::runtime_error(value.failure().value->error().value->message().value.value()),failure(*value.failure().value),terminal(std::move(value)) {}
};
namespace detail {
inline void check_session(int code) { if(code) throw NativeFailure(code,thinkthen_session_error_message()); }
struct EngineDeleter { void operator()(thinkthen_engine* p) const noexcept { thinkthen_engine_free(p); } };
struct SessionDeleter { void operator()(thinkthen_session* p) const noexcept { thinkthen_session_cancel(p); thinkthen_session_free(p); } };
struct PacketDeleter { void operator()(thinkthen_session_result* p) const noexcept { thinkthen_session_result_free(p); } };
}
enum class PollState { pending, result, end };
struct Poll {
    PollState state;
    std::optional<results::SessionPacket> packet;
};
// One host thread owns a Call. Move it to a worker or poll it from an event loop.
// Cancellation and destruction do not join native work. Packets own copied values.
class Call {
    std::unique_ptr<thinkthen_session,detail::SessionDeleter> session_;
    explicit Call(thinkthen_session* p):session_(p) {}
    friend class Client;
    void require_live() const { if(!session_) throw std::logic_error("C++ call is closed"); }
public:
    Call(Call&&)=default;
    Call& operator=(Call&&)=default;
    Call(const Call&)=delete;
    Call& operator=(const Call&)=delete;
    void cancel() noexcept { if(session_) thinkthen_session_cancel(session_.get()); }
    void close() noexcept { session_.reset(); }
    void finish() { require_live(); detail::check_session(thinkthen_session_finish(session_.get(),nullptr,0)); }
    bool try_push(const inputs::RequestSessionDescriptor& item) {
        require_live(); auto bytes=item.document().dump(); uint32_t state=THINKTHEN_SESSION_CLOSED_V1;
        detail::check_session(thinkthen_session_try_push(session_.get(),bytes.data(),bytes.size(),&state));
        if(state==THINKTHEN_SESSION_CLOSED_V1) throw std::logic_error("native input is closed");
        return state==THINKTHEN_SESSION_ACCEPTED_V1;
    }
    Poll poll() {
        require_live(); uint32_t state=THINKTHEN_SESSION_PENDING_V1; thinkthen_session_result* raw=nullptr;
        detail::check_session(thinkthen_session_try_read(session_.get(),&state,&raw));
        std::unique_ptr<thinkthen_session_result,detail::PacketDeleter> packet(raw);
        if(state==THINKTHEN_SESSION_PENDING_V1) return {PollState::pending,{}};
        if(state==THINKTHEN_SESSION_END_V1) return {PollState::end,{}};
        const char* bytes=nullptr; size_t size=0;
        detail::check_session(thinkthen_session_result_json(packet.get(),&bytes,&size));
        results::SessionPacket value(Json::parse(std::string(bytes,size)));
        if(auto terminal=value.as_SessionPacketTerminal()) {
            auto failure=terminal->failure();
            if(failure.state==results::PresenceState::value) throw SessionFailure(std::move(*terminal));
        }
        return {PollState::result,std::move(value)};
    }
    // Explicit blocking convenience; a caller can run this on its chosen worker.
    std::vector<results::SessionPacket> collect() {
        std::vector<results::SessionPacket> values;
        try {
            for(;;) {
                auto next=poll(); if(next.state==PollState::end) return values;
                if(next.packet) values.push_back(std::move(*next.packet));
                else std::this_thread::sleep_for(std::chrono::milliseconds(1));
            }
        } catch(SessionFailure& error) { error.packets=std::move(values); throw; }
    }
};
struct UsageStatus {
    const UsagePersistenceState state;
    const std::optional<std::string> advice;
};
class Client {
    std::unique_ptr<thinkthen_engine,detail::EngineDeleter> engine_;
    Call start(const char* function,const inputs::RequestQuestion& question,const inputs::RequestInput& input,const inputs::RequestOptions& options) const {
        if(!engine_) throw std::logic_error("C++ client is closed");
        auto bytes=Json{{"schema",results::request_version},{"call",Json{{"function",function},{"question",question.document()},{"input",input.document()},{"options",options.document()}}}}.dump();
        thinkthen_session* raw=nullptr;
        detail::check_session(thinkthen_session_new_with_surface(engine_.get(),bytes.data(),bytes.size(),"cpp",3,&raw));
        return Call(raw);
    }
public:
    Client():engine_(thinkthen_engine_new()) { check_engine(); }
    explicit Client(const Json::Object& settings):engine_(thinkthen_engine_new_with(Json(settings).dump().c_str())) { check_engine(); }
    Client(Client&&)=default;
    Client& operator=(Client&&)=default;
    Client(const Client&)=delete;
    Client& operator=(const Client&)=delete;
    void close() noexcept { engine_.reset(); }
    UsageStatus usage_persistence() const { return usage_status(false); }
    UsageStatus finish_usage_status() const { return usage_status(true); }
    Call decide(const inputs::RequestQuestion& question,const inputs::RequestInput& input,const inputs::RequestOptions& options={}) const { return start("decide",question,input,options); }
    Call choose(const inputs::RequestQuestion& question,const inputs::RequestInput& input,const inputs::RequestOptions& options={}) const { return start("choose",question,input,options); }
    Call tag(const inputs::RequestQuestion& question,const inputs::RequestInput& input,const inputs::RequestOptions& options={}) const { return start("tag",question,input,options); }
    Call score(const inputs::RequestQuestion& question,const inputs::RequestInput& input,const inputs::RequestOptions& options={}) const { return start("score",question,input,options); }
    Call filter(const inputs::RequestQuestion& question,const inputs::RequestInput& input,const inputs::RequestOptions& options={}) const { return start("filter",question,input,options); }
    Call rank(const inputs::RequestQuestion& question,const inputs::RequestInput& input,const inputs::RequestOptions& options={}) const { return start("rank",question,input,options); }
    Call find(const inputs::RequestQuestion& question,const inputs::RequestInput& input,const inputs::RequestOptions& options={}) const { return start("find",question,input,options); }
    Call annotate(const inputs::RequestQuestion& question,const inputs::RequestInput& input,const inputs::RequestOptions& options={}) const { return start("annotate",question,input,options); }
    Call recognize(const inputs::RequestQuestion& question,const inputs::RequestInput& input,const inputs::RequestOptions& options={}) const { return start("recognize",question,input,options); }
    Call relate(const inputs::RequestQuestion& question,const inputs::RequestInput& input,const inputs::RequestOptions& options={}) const { return start("relate",question,input,options); }
private:
    UsageStatus usage_status(bool finish) const {
        if(!engine_) throw std::logic_error("C++ client is closed");
        thinkthen_complete_usage_persistence_v1 state{}; thinkthen_complete_utf8_v1 advice{};
        int code=finish ? thinkthen_engine_finish_usage_status_v1(engine_.get(),&state,&advice) : thinkthen_engine_usage_persistence_v1(engine_.get(),&state,&advice);
        detail::check_session(code);
        return {static_cast<UsagePersistenceState>(state.kind), advice.data ? std::optional<std::string>(std::string(advice.data,advice.len)) : std::nullopt};
    }
    void check_engine() {
        if(!engine_) throw NativeFailure(thinkthen_error_code(nullptr),thinkthen_error_message(nullptr));
    }
};
}
