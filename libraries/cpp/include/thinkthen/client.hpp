#pragma once
#include "results_generated.hpp"
#include "inputs_generated.hpp"
#include <memory>
namespace tt {
enum class PollState { pending, result, end };
struct Poll {
    PollState state;
    std::optional<results::SessionPacket> packet;
};
class Call {
    struct State;
    std::unique_ptr<State> state_;
    explicit Call(std::unique_ptr<State> state);
    friend class Client;
    bool admit(const inputs::RequestSessionDescriptor& item,bool blocking);
    Poll read(bool blocking,bool settlement);
protected:
    const std::vector<results::SessionPacket>& completed();
public:
    Call(Call&&) noexcept;
    Call& operator=(Call&&) noexcept;
    ~Call();
    Call(const Call&)=delete;
    Call& operator=(const Call&)=delete;
    void cancel() noexcept;
    void close() noexcept;
    bool input_closed() const noexcept;
    void finish();
    bool try_push(const inputs::RequestSessionDescriptor& item) { return admit(item,false); }
    bool push(const inputs::RequestSessionDescriptor& item) { return admit(item,true); }
    Poll poll() { return read(false,false); }
    Poll next() { return read(true,false); }
    void wait(bool input=false);
    std::vector<results::SessionPacket> collect();
};
struct UsageStatus {
    const UsagePersistenceState state;
    const std::optional<std::string> advice;
};
template<auto Project> class ResultCall : public Call {
public:
    explicit ResultCall(Call&& call):Call(std::move(call)) {}
    std::vector<results::SessionPacket> collect() { return Call::collect(); }
    auto result() { if(!input_closed()) finish(); return Project(completed()); }
};
}
#include "calls_generated.hpp"
namespace tt {
class Client : public NamedCalls<Client> {
    friend class NamedCalls<Client>;
    struct State;
    std::unique_ptr<State> state_;
    Call start(const std::string& function,const inputs::RequestQuestion& question,const inputs::RequestInput& input,const inputs::RequestOptions& options) const;
    results::Plan plan_impl(const inputs::RequestCall& call) const;
    UsageStatus usage_impl(bool finish) const;
public:
    Client();
    explicit Client(const inputs::EngineSettings& settings);
    Client(Client&&) noexcept;
    Client& operator=(Client&&) noexcept;
    ~Client();
    Client(const Client&)=delete;
    Client& operator=(const Client&)=delete;
    void close() noexcept;
};
}
