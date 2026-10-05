import ThinkThen

@main
struct Backends {
    static func main() throws {
        let settings = [
            #"{"backend":"typesafe"}"#,
            #"{"backend":"liquid"}"#,
            #"{"backend":"ollama","base_url":"http://localhost:11535/v1"}"#,
        ]
        for setting in settings {
            let tt = try Engine(settingsJSON: setting)
            let question = "Does the customer ask for a refund?"
            let brokenIsRefund = try tt.decide(
                question,
                "Please refund my order. It arrived broken."
            )
            let thanksIsRefund = try tt.decide(
                question,
                "Thanks for the quick help yesterday!"
            )
            precondition(brokenIsRefund.value.outcome == .yes)
            precondition(thanksIsRefund.value.outcome == .no)
            tt.close()
        }
    }
}
