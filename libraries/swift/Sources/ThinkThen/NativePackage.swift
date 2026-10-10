import Foundation
import CThinkThen

#if os(Linux) && SWIFT_PACKAGE
private enum NativePackage {
    static let loaded: Result<Void, SessionBoundaryFailure> = {
        #if arch(x86_64)
        let triple = "x86_64-unknown-linux-gnu"
        #elseif arch(arm64)
        let triple = "aarch64-unknown-linux-gnu"
        #else
        #error("ThinkThen has no native asset for this Swift host")
        #endif
        let asset = Bundle.module.url(forResource: "libthinkthen", withExtension: "so", subdirectory: "Native/" + triple)
        guard let asset, asset.path.withCString({ thinkthen_swift_load($0) }) == 0 else {
            return .failure(SessionBoundaryFailure(code: Int32(THINKTHEN_ELOCAL), message: "ThinkThen native package could not be loaded"))
        }
        return .success(())
    }()
}

#endif
func requireNativePackage() throws {
    #if os(Linux) && SWIFT_PACKAGE
    try NativePackage.loaded.get()
    #endif
}
