import thinkthen as tt

documents: list[tt.Record] = [
    {"id": "memo-1", "text": "Ines Costa joined Example Corp."},
    {"id": "memo-2", "text": "Tailspin hired Morgan Lee."},
]
with tt.recognize_stream(
    documents, kinds=["person", "organization"], field="/text"
) as recognitions:
    for recognized in recognitions:
        for name in recognized:
            print(
                recognized.id, name.kind, repr(name.text),
                name.start, name.end,
            )
