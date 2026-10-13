#pragma once
#include <thinkthen/thinkthen.h>
#include "results_generated.hpp"
#include "inputs_generated.hpp"
#include <memory>
namespace tt {
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
    bool input_closed_=false;
    explicit Call(thinkthen_session* p):session_(p) {}
    friend class Client;
    void require_live() const { if(!session_) throw std::logic_error("C++ call is closed"); }
public:
    Call(Call&&)=default;
    Call& operator=(Call&&)=default;
    Call(const Call&)=delete;
    Call& operator=(const Call&)=delete;
    void cancel() noexcept { input_closed_=true; if(session_) thinkthen_session_cancel(session_.get()); }
    void close() noexcept { input_closed_=true; session_.reset(); }
    bool input_closed() const noexcept { return input_closed_ || !session_; }
    void finish() { require_live(); detail::check_session(thinkthen_session_finish(session_.get(),nullptr,0)); input_closed_=true; }
    bool try_push(const inputs::RequestSessionDescriptor& item) {
        require_live(); if(input_closed_) return false; auto bytes=item.document().dump(); uint32_t state=THINKTHEN_SESSION_CLOSED_V1;
        detail::check_session(thinkthen_session_try_push(session_.get(),bytes.data(),bytes.size(),&state));
        input_closed_=state==THINKTHEN_SESSION_CLOSED_V1;
        return state==THINKTHEN_SESSION_ACCEPTED_V1;
    }
    Poll read(bool blocking=false) {
        require_live(); uint32_t state=THINKTHEN_SESSION_PENDING_V1; thinkthen_session_result* raw=nullptr;
        detail::check_session(blocking ? thinkthen_session_next_packet(session_.get(),THINKTHEN_SESSION_WAIT_SETTLEMENT_V1,&state,&raw) : thinkthen_session_try_read(session_.get(),&state,&raw));
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
    Poll poll() { return read(); }
    bool push(const inputs::RequestSessionDescriptor& item) {
        require_live(); if(input_closed_) return false; auto bytes=item.document().dump(); uint32_t state=THINKTHEN_SESSION_CLOSED_V1;
        detail::check_session(thinkthen_session_push(session_.get(),bytes.data(),bytes.size(),&state));
        input_closed_=state==THINKTHEN_SESSION_CLOSED_V1;
        return state==THINKTHEN_SESSION_ACCEPTED_V1;
    }
    // Wait consumes nothing. Keep this owner live and join waiters before close.
    void wait(bool input=false) {
        require_live(); uint32_t ready=0;
        detail::check_session(thinkthen_session_wait(session_.get(),input ? THINKTHEN_SESSION_WAIT_INPUT_V1 : THINKTHEN_SESSION_WAIT_SETTLEMENT_V1,UINT32_MAX,&ready));
    }
    // Explicit blocking convenience; a caller can run this on its chosen worker.
    std::vector<results::SessionPacket> collect() {
        std::vector<results::SessionPacket> values;
        try {
            for(;;) {
                auto next=read(true); if(next.state==PollState::end) return values;
                if(next.packet) values.push_back(std::move(*next.packet));
            }
        } catch(SessionFailure& error) { error.packets=std::move(values); throw; }
    }
};
struct UsageStatus {
    const UsagePersistenceState state;
    const std::optional<std::string> advice;
};
template<auto Reader> class AnswerCall : public Call {
public:
    explicit AnswerCall(Call&& call):Call(std::move(call)) {}
    auto answer() { if(!input_closed()) finish(); return Reader(collect()); }
};
}
#include "calls_generated.hpp"
namespace tt {
class Client : public NamedCalls<Client> {
    friend class NamedCalls<Client>;
    std::unique_ptr<thinkthen_engine,detail::EngineDeleter> engine_;
    Call start(const std::string& function,const inputs::RequestQuestion& question,const inputs::RequestInput& input,const inputs::RequestOptions& options) const {
        if(!engine_) throw std::logic_error("C++ client is closed");
        auto q=question.document().dump(), in=input.document().dump(), opts=options.document().dump();
        thinkthen_named_request_v1 request{{function.data(),function.size()},{q.data(),q.size()},{in.data(),in.size()},{opts.data(),opts.size()}};
        thinkthen_session* raw=nullptr;
        detail::check_session(thinkthen_session_new_named_with_surface(engine_.get(),&request,{"cpp",3},&raw));
        return Call(raw);
    }
public:
    Client():engine_(thinkthen_engine_new()) { check_engine(); }
    explicit Client(const inputs::EngineSettings& settings):Client(settings.document().object()) {}
    explicit Client(const Json::Object& settings):engine_(thinkthen_engine_new_with(Json(settings).dump().c_str())) { check_engine(); }
    Client(Client&&)=default;
    Client& operator=(Client&&)=default;
    Client(const Client&)=delete;
    Client& operator=(const Client&)=delete;
    void close() noexcept { engine_.reset(); }
private:
    thinkthen_engine* native_engine() const {
        if(!engine_) throw std::logic_error("C++ client is closed");
        return engine_.get();
    }
    void check_engine() {
        if(!engine_) throw NativeFailure(thinkthen_error_code(nullptr),thinkthen_error_message(nullptr));
    }
};
}
