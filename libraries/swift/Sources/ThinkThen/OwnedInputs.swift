import Foundation

extension Question {
    public static func text(_ text: String) -> Self { .text(QuestionText(text: text)) }
    public static func file(_ path: String) -> Self { .file(QuestionFile(path: path)) }
    public static func name(_ name: String) -> Self { .name(QuestionName(name: name)) }
    public static func reference(_ reference: String) -> Self { .reference(QuestionReference(reference: reference)) }
}

extension Input {
    public static func text(_ text: String, images: Presence<[Image]> = .absent) -> Self {
        .text(InputText(images: images, text: text))
    }
    public static func files(_ paths: [String], reading: Presence<Reader> = .absent) -> Self {
        .source(InputSource(source: Source(paths: paths, reading: reading)))
    }
}

extension Image {
    public static func bytes(_ bytes: Data, media: ImageMedia) -> Self {
        .bytes(ImageBytes(bytes: bytes.base64EncodedString(), media: media))
    }
    public static func file(_ path: String, media: Presence<ImageMedia> = .absent) -> Self {
        .file(ImageFile(media: media, path: path))
    }
}

extension Item {
    public static func text(_ text: String) -> Self { Item(original: .value(.text(OriginalText(text: text)))) }
}
