import Foundation
import CThinkThen
@main enum NativeConsumer {
    static func selected(_ r: NativeResult, _ verb: String) throws {
        for i in r.rows.indices {
            switch verb {
            case "decide": _ = try r.decide(i)
            case "choose": _ = try r.choose(i)
            case "tag": _ = try r.tag(i)
            case "score": _ = try r.score(i)
            case "filter": _ = try r.filter(i)
            case "rank": _ = try r.rank(i)
            case "find": _ = try r.find(i)
            case "annotate": _ = try r.annotate(i)
            case "recognize": _ = try r.recognize(i)
            case "relate": _ = try r.relate(i)
            default: throw NativeConversion.invalidDiscriminator
            }
        }
    }
    static func emit(_ r: NativeResult) throws { print(String(decoding:try JSONEncoder().encode(r),as:UTF8.self)) }
    static func input(_ value: Any, text: Bool) throws -> NativeInput {
        if text, let s = value as? String { return .text(s) }
        return .json(String(decoding:try JSONSerialization.data(withJSONObject:value,options:[.fragmentsAllowed]),as:UTF8.self))
    }
    static func main() {
        var cancellation: Thread? = nil
        let joined = DispatchSemaphore(value:0)
        do {
            guard CommandLine.arguments.count == 3 else { throw NativeConversion.invalidExtent }
            let v = try JSONSerialization.jsonObject(with:Data(contentsOf:URL(fileURLWithPath:CommandLine.arguments[1]))) as! [String:Any]
            let settings = CommandLine.arguments[2]
            let settingValues = try JSONSerialization.jsonObject(with:Data(settings.utf8)) as! [String:Any]
            guard (settingValues["base_url"] as! String).hasPrefix("http://127.0.0.1:") else { throw NativeConversion.invalidExtent }
            let e: Engine
            do { e = try Engine(settingsJSON:settings) }
            catch let f as DoorFailure { try nativeChecked(nil,f.code); throw f }
            defer { e.close() }
            let buffers = NativeBuffers(); let role = NativeRole(rawValue:(v["role"] as! NSNumber).uint32Value)!
            let verb = v["verb"] as! String
            var q: NativeQuestion?
            if v["find_none"] as? Bool == true {
                var spec = thinkthen_question_spec_v1(); spec.kind = 7; spec.none = 1
                spec.text = buffers.content(v["find_text_literal"] as? Bool == true ? .text(v["find_text"] as! String) : .json(v["find_text"] as! String))
                var author = thinkthen_question_author_v1()
                if let metadata = v["metadata"] as? [String:Any] { author.name = thinkthen_optional_string_v1(present:1,value:buffers.string(metadata["name"] as! String)); author.wording_version = thinkthen_optional_u64_v1(present:1,value:(metadata["wording_version"] as! NSNumber).uint64Value) }
                q = try e.question(spec,author:author)
            } else if let loader = v["loader"] as? String {
                let ref = v["reference"] as! String
                if loader == "load" || loader == "file" { q = try e.loadQuestion(path:ref) }
                else if loader == "load_named" || loader == "named" { q = try e.namedQuestion(role:role,name:ref) }
                else { q = try e.referenceQuestion(role:role,reference:ref) }
            } else if v["question_form"] as? String == "file" { q = try e.loadQuestion(path:"fixture-question.json") }
            else { q = try e.parseQuestion(role:role,json:v["question_json"] as! String) }
            let token = try CancelToken(); var controls = NativeControls(); controls.attempts = true
            if let op = v["operation"] as? [String:Any] {
                if op["injection"] as? String == "cancel_token" { token.cancel(); controls.cancel = token }
                if op["injection"] as? String == "expired_deadline" { controls.deadline = 0 }
            }
            var images: [NativeImage] = []
            for hex in v["image_data"] as! [String] {
                let chars = Array(hex.utf8); var bytes: [UInt8] = []
                for i in stride(from:0,to:chars.count,by:2) { bytes.append(UInt8(String(decoding:chars[i..<i+2],as:UTF8.self),radix:16)!) }
                let image = try e.image(bytes:bytes,media:(v["media_code"] as! NSNumber).uint32Value)
                let imageView = try image.view(); precondition(imageView.bytes == bytes); images.append(image)
            }
            if let c = v["shared_context"], !(c is NSNull) { controls.context = try input(c,text:c is String) }
            var source: NativeSource?
            if let paths = v["paths"] as? [String] {
                source = try e.fileSource(paths:paths,unit:NativeUnit(rawValue:(v["source_unit"] as! NSNumber).uint32Value)!,window:(v["window"] as? NSNumber)?.intValue ?? 0,imageReader:v["image_reader"] as? Bool == true)
            } else {
                let items = v["items"] as! [Any]; var records: [NativeRecord] = []
                for (i,item) in items.enumerated() {
                    var original: NativeInput? = v["image_only"] as? Bool == true ? nil : try input(item,text:v["text"] as? Bool == true)
                    if v["caption_files"] as? Bool == true { original = .text(try String(contentsOfFile:"caption-\(i).txt",encoding:.utf8)) }
                    var context: NativeInput? = nil
                    if v["context_present"] as? Bool == true { let c = v["context"]!; context = try input(c,text:c is String) }
                    var options: [thinkthen_choice_v1] = []
                    if let orders = v["candidate_orders"] as? [[String]] { for name in orders[i] { var option = thinkthen_choice_v1(); option.name = buffers.string(name); options.append(option) } }
                    records.append(NativeRecord(original:original,context:context,options:options,images:images))
                }
                source = try e.records(records)
            }
            if v["held_cancel"] as? Bool == true {
                controls.cancel = token
                cancellation = Thread { precondition(getchar() == 33); token.cancel(); FileHandle.standardOutput.write(Data("cancel-fired\n".utf8)); joined.signal() }
                cancellation!.start()
            }
            if v["incremental"] as? Bool == true {
                let batch: NativeLazyBatch
                switch verb {
                case "decide": batch = try e.decideBatch(q!,source:source!,controls:controls)
                case "choose": batch = try e.chooseBatch(q!,source:source!,controls:controls)
                case "tag": batch = try e.tagBatch(q!,source:source!,controls:controls)
                case "score": batch = try e.scoreBatch(q!,source:source!,controls:controls)
                case "filter": batch = try e.filterBatch(q!,source:source!,controls:controls)
                case "annotate": batch = try e.annotateBatch(q!,source:source!,controls:controls)
                default: throw NativeConversion.invalidDiscriminator
                }
                defer { batch.close() }; q = nil; source = nil; images = []
                while let row = try batch.next() { try selected(row,verb); try emit(row) }
                try emit(batch.facts())
            } else {
                let result: NativeResult
                switch verb {
                case "decide": result = try e.decide(q!,source:source!,controls:controls)
                case "choose": result = try e.choose(q!,source:source!,controls:controls)
                case "tag": result = try e.tag(q!,source:source!,controls:controls)
                case "score": result = try e.score(q!,source:source!,controls:controls)
                case "filter": result = try e.filter(q!,source:source!,controls:controls)
                case "rank": result = try e.rank(q!,source:source!,controls:controls)
                case "find": result = try e.find(q!,source:source!,controls:controls)
                case "annotate": result = try e.annotate(q!,source:source!,controls:controls)
                case "recognize": result = try e.recognize(q!,source:source!,controls:controls)
                case "relate": result = try e.relate(q!,source:source!,controls:controls)
                default: throw NativeConversion.invalidDiscriminator
                }
                q = nil; source = nil; images = []; e.close(); try selected(result,verb); try emit(result)
            }
        } catch let failure as NativeFailure {
            do { print(String(decoding:try JSONEncoder().encode(failure.snapshot),as:UTF8.self)) }
            catch { FileHandle.standardError.write(Data("snapshot encoding failed\n".utf8)); exit(1) }
        } catch { FileHandle.standardError.write(Data("native consumer failed: \(error)\n".utf8)); exit(1) }
        if cancellation != nil { joined.wait() }
    }
}
