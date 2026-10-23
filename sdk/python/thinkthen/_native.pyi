from collections.abc import Iterator
from typing import Any

class Error(Exception): ...

class NativeCancel:
    def __init__(self) -> None: ...
    def cancel(self) -> None: ...

class NativeQuestion:
    @property
    def function(self) -> str: ...

class NativeToolbox:
    def __init__(self, definition: str) -> None: ...
    def question(self, name: str | None) -> NativeQuestion: ...

class NativeStream:
    def receive(self) -> tuple[str, str | None, bool, Any, int] | None: ...
    def close(self) -> None: ...

class NativeEngine:
    def __init__(self, settings_json: str) -> None: ...
    def start(
        self,
        function: str,
        request_json: str,
        options_json: str,
        source: Iterator[tuple[bool, str, Any]],
        cancel: NativeCancel,
        keep_originals: bool,
        loaded: NativeQuestion | None = None,
        toolbox: NativeToolbox | None = None,
    ) -> NativeStream: ...
