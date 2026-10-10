import ThinkThen

@main struct PackageConsumer {
    static func main() async throws {
        if CommandLine.arguments.count > 1 {
            let constructors: [() throws -> Void] = [
                { let client = try Client(); client.close() },
                { let engine = try Engine(); engine.close() },
                { let engine = try Engine(settingsJSON: "{}"); engine.close() },
                { _ = try CancelToken() }
            ]
            for construct in constructors {
                do { try construct(); fatalError("Missing asset succeeded") }
                catch let error as SessionBoundaryFailure {
                    guard case .local = error.kind else { fatalError("Wrong failure kind") }
                    precondition(error.message == "ThinkThen native package could not be loaded")
                }
            }
            print("PACKAGE_MISSING_ASSET_PASS")
        } else {
            try await withThrowingTaskGroup(of: Void.self) { tasks in
                for _ in 0..<8 { tasks.addTask { let client = try Client(); client.close() } }
                try await tasks.waitForAll()
            }
            let engine = try Engine(settingsJSON: "{\"cache\":false}")
            _ = try engine.usagePersistence()
            engine.close()
            let token = try CancelToken(); token.cancel()
            print("PACKAGE_LOAD_PASS")
        }
    }
}
