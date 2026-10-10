import Foundation

extension InputRequestQuestion {
    public static func text(_ text: String) -> Self { .text(InputRequestQuestionText(text: text)) }
    public static func file(_ path: String) -> Self { .file(InputRequestQuestionFile(path: path)) }
    public static func name(_ name: String) -> Self { .name(InputRequestQuestionName(name: name)) }
    public static func reference(_ reference: String) -> Self { .reference(InputRequestQuestionReference(reference: reference)) }
}

extension InputRequestInput {
    public static func text(_ text: String, images: Presence<[InputRequestImage]> = .absent) -> Self {
        .text(InputRequestInputText(images: images, text: text))
    }
    public static func files(_ paths: [String], reading: Presence<InputRequestReader> = .absent) -> Self {
        .source(InputRequestInputSource(source: InputRequestSource(paths: paths, reading: reading)))
    }
}

extension InputRequestImage {
    public static func bytes(_ bytes: Data, media: InputImageMedia) -> Self {
        .bytes(InputRequestImageBytes(bytes: bytes.base64EncodedString(), media: media))
    }
    public static func file(_ path: String, media: Presence<InputImageMedia> = .absent) -> Self {
        .file(InputRequestImageFile(media: media, path: path))
    }
}

extension InputRequestItem {
    public static func text(_ text: String) -> Self { InputRequestItem(original: .value(.text(InputRequestOriginalText(text: text)))) }
}
