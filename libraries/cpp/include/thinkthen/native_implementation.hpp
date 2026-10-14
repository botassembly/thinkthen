#define THINKTHEN_CPP_IMPLEMENTATION
#include "client.hpp"
namespace tt {
Failure::Failure(int code,const std::string& message):std::runtime_error(message),kind(static_cast<NativeFailureKind>(code)) {}
Failure::Failure(const char* message,const std::vector<results::SessionPacket>& values):std::runtime_error(message),kind(NativeFailureKind::defect),packets(values) {}
Failure::Failure(results::SessionPacketTerminal value):std::runtime_error(value.failure().value->error().value->message().value.value()),kind(failure_kind(value.failure().value->error().value->kind().value->value())),retryable(*value.failure().value->error().value->retryable().value),failure(*value.failure().value),terminal(std::move(value)) {}
Data Data::parse(const std::string& text) {
    thinkthen_value* raw=nullptr;
    if(thinkthen_value_parse(text.data(),text.size(),&raw)) throw std::invalid_argument("invalid original value");
    std::unique_ptr<thinkthen_value,decltype(&thinkthen_value_free)> owner(raw,&thinkthen_value_free);
    const thinkthen_complete_json_v1* view=nullptr;
    if(thinkthen_value_view(owner.get(),&view)) throw std::invalid_argument("invalid original value owner");
    return results::detail::native_value(view);
}
Data Data::read(std::istream& input) { return parse(std::string(std::istreambuf_iterator<char>(input),{})); }
namespace detail {
Data parse_original(const std::string& text) { return Data::parse(text); }
void fail(int code,const thinkthen_error_view_v1* view,const char* message) {
    ::tt::Failure error(code,message);
    if(view) { error.retryable=view->retryable!=0; if(view->facts) error.facts.emplace(results::detail::Loader::input<results::ImmediateFacts>(results::detail::native_value(view->facts))); }
    throw error;
}
void check(int code) { if(code) fail(code,thinkthen_session_error_view(),thinkthen_session_error_message()); }
struct EngineDeleter { void operator()(thinkthen_engine* p) const noexcept { thinkthen_engine_free(p); } };
struct SessionDeleter { void operator()(thinkthen_session* p) const noexcept { thinkthen_session_cancel(p); thinkthen_session_free(p); } };
struct PacketDeleter { void operator()(thinkthen_session_result* p) const noexcept { thinkthen_session_result_free(p); } };
}
struct Call::State {
    std::unique_ptr<thinkthen_session,detail::SessionDeleter> session;
    bool closed=false;
    explicit State(thinkthen_session* raw):session(raw) {}
};
struct Client::State {
    std::unique_ptr<thinkthen_engine,detail::EngineDeleter> engine;
    explicit State(const char* settings=nullptr):engine(settings ? thinkthen_engine_new_with(settings) : thinkthen_engine_new()) {
        if(!engine) detail::fail(thinkthen_error_code(nullptr),thinkthen_error_view(nullptr),thinkthen_error_message(nullptr));
    }
    thinkthen_engine* live() const { if(!engine) throw std::logic_error("C++ client is closed"); return engine.get(); }
};
Call::Call(std::unique_ptr<State> state):state_(std::move(state)) {}
Call::Call(Call&&) noexcept=default;
Call& Call::operator=(Call&&) noexcept=default;
Call::~Call()=default;
void Call::cancel() noexcept { if(state_) { state_->closed=true; thinkthen_session_cancel(state_->session.get()); } }
void Call::close() noexcept { state_.reset(); }
bool Call::input_closed() const noexcept { return !state_ || state_->closed; }
void Call::finish() {
    if(!state_) throw std::logic_error("C++ call is closed");
    detail::check(thinkthen_session_finish(state_->session.get(),nullptr,0)); state_->closed=true;
}
bool Call::admit(const inputs::RequestSessionDescriptor& item,bool blocking) {
    if(!state_) throw std::logic_error("C++ call is closed");
    if(state_->closed) return false;
    auto bytes=::tt::detail::serialize(::tt::results::detail::document(item)); uint32_t status=THINKTHEN_SESSION_CLOSED_V1;
    detail::check(blocking ? thinkthen_session_push(state_->session.get(),bytes.data(),bytes.size(),&status) : thinkthen_session_try_push(state_->session.get(),bytes.data(),bytes.size(),&status));
    state_->closed=status==THINKTHEN_SESSION_CLOSED_V1; return status==THINKTHEN_SESSION_ACCEPTED_V1;
}
Poll Call::read(bool blocking,bool settlement) {
    if(!state_) throw std::logic_error("C++ call is closed");
    uint32_t status=THINKTHEN_SESSION_PENDING_V1; thinkthen_session_result* raw=nullptr;
    uint32_t interest=settlement ? THINKTHEN_SESSION_WAIT_SETTLEMENT_V1 : THINKTHEN_SESSION_WAIT_RESULT_V1;
    detail::check(blocking ? thinkthen_session_next_packet(state_->session.get(),interest,&status,&raw) : thinkthen_session_try_read(state_->session.get(),&status,&raw));
    std::unique_ptr<thinkthen_session_result,detail::PacketDeleter> packet(raw);
    if(status==THINKTHEN_SESSION_PENDING_V1) return {PollState::pending,{}};
    if(status==THINKTHEN_SESSION_END_V1) return {PollState::end,{}};
    const thinkthen_complete_session_packet_v1* view=nullptr;
    detail::check(thinkthen_session_result_view(packet.get(),&view));
    auto value=results::detail::Loader::input<results::SessionPacket>(results::detail::native_value(view));
    if(auto terminal=value.as_SessionPacketTerminal()) if(terminal->failure().state==results::PresenceState::value) throw ::tt::Failure(std::move(*terminal));
    return {PollState::result,std::move(value)};
}
void Call::wait(bool input) {
    if(!state_) throw std::logic_error("C++ call is closed");
    uint32_t ready=0;
    detail::check(thinkthen_session_wait(state_->session.get(),input ? THINKTHEN_SESSION_WAIT_INPUT_V1 : THINKTHEN_SESSION_WAIT_SETTLEMENT_V1,UINT32_MAX,&ready));
}
std::vector<results::SessionPacket> Call::collect() {
    std::vector<results::SessionPacket> values;
    try { for(;;) { auto next=read(true,true); if(next.state==PollState::end) return values; if(next.packet) values.push_back(std::move(*next.packet)); } }
    catch(::tt::Failure& error) { error.packets=std::move(values); throw; }
}
Client::Client():state_(std::make_unique<State>()) {}
Client::Client(const inputs::EngineSettings& settings):state_(std::make_unique<State>(::tt::detail::serialize(::tt::results::detail::document(settings)).c_str())) {}
Client::Client(Client&&) noexcept=default;
Client& Client::operator=(Client&&) noexcept=default;
Client::~Client()=default;
void Client::close() noexcept { state_.reset(); }
Call Client::start(const std::string& function,const inputs::RequestQuestion& question,const inputs::RequestInput& input,const inputs::RequestOptions& options) const {
    if(!state_) throw std::logic_error("C++ client is closed");
    auto q=::tt::detail::serialize(::tt::results::detail::document(question)), in=::tt::detail::serialize(::tt::results::detail::document(input)), opts=::tt::detail::serialize(::tt::results::detail::document(options));
    thinkthen_named_request_v1 request{{function.data(),function.size()},{q.data(),q.size()},{in.data(),in.size()},{opts.data(),opts.size()}};
    auto state=std::make_unique<Call::State>(nullptr); thinkthen_session* raw=nullptr;
    detail::check(thinkthen_session_new_named_with_surface(state_->live(),&request,{"cpp",3},&raw));
    state->session.reset(raw); return Call(std::move(state));
}
results::Plan Client::plan_impl(const inputs::RequestCall& call) const {
    if(!state_) throw std::logic_error("C++ client is closed");
    const auto& value=::tt::results::detail::document(call); auto function=value.at("function").get<std::string>();
    auto question=::tt::detail::serialize(value.at("question")), input=::tt::detail::serialize(value.at("input"));
    auto options=value.contains("options") ? ::tt::detail::serialize(value.at("options")) : std::string("{}");
    thinkthen_named_request_v1 request{{function.data(),function.size()},{question.data(),question.size()},{input.data(),input.size()},{options.data(),options.size()}};
    thinkthen_value* raw=nullptr; detail::check(thinkthen_request_plan_named_value(state_->live(),&request,&raw));
    std::unique_ptr<thinkthen_value,decltype(&thinkthen_value_free)> owned(raw,&thinkthen_value_free);
    const thinkthen_complete_json_v1* view=nullptr; detail::check(thinkthen_value_view(owned.get(),&view));
    return results::detail::Loader::input<results::Plan>(results::detail::native_value(view));
}
UsageStatus Client::usage_impl(bool finish) const {
    if(!state_) throw std::logic_error("C++ client is closed");
    thinkthen_complete_usage_persistence_v1 status{}; thinkthen_complete_utf8_v1 advice{};
    detail::check(finish ? thinkthen_engine_finish_usage_status_v1(state_->live(),&status,&advice) : thinkthen_engine_usage_persistence_v1(state_->live(),&status,&advice));
    return {static_cast<UsagePersistenceState>(status.kind),advice.data ? std::optional<std::string>(std::string(advice.data,advice.len)) : std::nullopt};
}
}
