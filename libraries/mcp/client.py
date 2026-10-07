"""Dependency-free named consumer of the ThinkThen local MCP subprocess.

A client owns only the process it launches. It never runs a shell or starts a
process per tool call. Call cancel from another thread while a tool is pending.
"""
import json
import math
import queue
import subprocess
import threading

VERSION = '2025-11-25'
MAX_MESSAGE = 192 * 1024 * 1024


def _json(text):
    def members(pairs):
        value = {}
        for name, item in pairs:
            if name in value:
                raise ValueError('duplicate JSON member')
            value[name] = item
        return value
    def constant(_):
        raise ValueError('nonfinite JSON number')
    def number(text):
        value = float(text)
        if not math.isfinite(value):
            raise ValueError('nonfinite JSON number')
        return value
    return json.loads(text, object_pairs_hook=members, parse_constant=constant, parse_float=number)


def _same(left, right):
    if type(left) is not type(right):
        return False
    if isinstance(left, dict):
        return list(left) == list(right) and all(_same(left[name], right[name]) for name in left)
    if isinstance(left, list):
        return len(left) == len(right) and all(_same(a, b) for a, b in zip(left, right))
    return left == right


class ProtocolError(Exception):
    """A safe diagnostic that never includes a raw protocol payload."""


class ToolError(Exception):
    """A failed native call; result retains the native safe error and facts."""
    def __init__(self, result):
        super().__init__('ThinkThen tool call failed')
        self.result = result


class CancelledError(Exception):
    """The caller cancelled its pending MCP request."""


class Client:
    """One stdio session with ten explicit judging methods."""
    def __init__(self, writer, reader, process=None):
        self.writer = writer
        self.reader = reader
        self.process = process
        self._binary = process is not None
        self._next_id = 0
        self._write_lock = threading.Lock()
        self._call_lock = threading.Lock()
        self._state_lock = threading.Lock()
        self._active_method = None
        self._responses = queue.Queue(1)
        self._closed = threading.Event()
        self._cancelled = threading.Event()
        self._reader_thread = None
        self._active_id = None
        self._cancelled_through = 0

    @classmethod
    def launch(cls, command=('thinkthen', 'mcp'), *, env=None, cwd=None):
        process = subprocess.Popen(command, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                   stderr=subprocess.DEVNULL, env=env, cwd=cwd)
        client = cls(process.stdin, process.stdout, process)
        try:
            client.initialize()
        except BaseException:
            client.close()
            raise
        return client

    def _send(self, message):
        try:
            encoded = (json.dumps(message, ensure_ascii=False, separators=(',', ':'),
                                  allow_nan=False) + '\n').encode('utf-8')
        except (TypeError, ValueError, UnicodeError):
            raise ProtocolError('MCP request is not valid JSON') from None
        if len(encoded) > 16 * 1024 * 1024:
            raise ProtocolError('MCP request exceeds 16 MiB')
        try:
            with self._write_lock:
                self.writer.write(encoded if self._binary else encoded.decode('utf-8'))
                self.writer.flush()
        except (OSError, ValueError):
            raise ProtocolError('MCP input closed') from None

    def _read(self):
        while not self._closed.is_set():
            try:
                line = self.reader.readline(MAX_MESSAGE + 1)
                ending = b'\n' if isinstance(line, bytes) else '\n'
                size = len(line) if isinstance(line, bytes) else len(line.encode('utf-8'))
                if not line.endswith(ending) or size > MAX_MESSAGE:
                    raise ProtocolError('MCP output closed or exceeded its bound')
                response = _json(line)
            except (OSError, UnicodeError, ValueError, RecursionError):
                response = ProtocolError('invalid MCP output')
            except ProtocolError as error:
                response = error
            while not self._closed.is_set():
                try:
                    self._responses.put(response, timeout=0.05)
                    break
                except queue.Full:
                    continue
            if isinstance(response, ProtocolError):
                return

    @property
    def pending_id(self):
        """ID to cancel from another thread, or None when no request is pending."""
        with self._state_lock:
            return self._active_id

    def _request(self, method, params=None):
        with self._call_lock:
            self._next_id += 1
            ident = self._next_id
            with self._state_lock:
                self._cancelled.clear()
                self._active_id = ident
                self._active_method = method
            message = {'jsonrpc': '2.0', 'id': ident, 'method': method}
            if params is not None:
                message['params'] = params
            try:
                self._send(message)
                if self._reader_thread is None:
                    self._reader_thread = threading.Thread(target=self._read, daemon=True)
                    self._reader_thread.start()
                return self._receive(ident)
            finally:
                with self._state_lock:
                    self._active_id = None
                    self._active_method = None

    def _receive(self, ident):
        while not self._closed.is_set():
            if self._cancelled.is_set():
                raise CancelledError('MCP request cancelled')
            try:
                response = self._responses.get(timeout=0.05)
            except queue.Empty:
                continue
            if isinstance(response, ProtocolError):
                raise response
            if (not isinstance(response, dict) or response.get('jsonrpc') != '2.0'
                    or type(response.get('id')) is not int
                    or ('error' in response) == ('result' in response)):
                raise ProtocolError('unexpected MCP response')
            if response['id'] <= self._cancelled_through:
                continue  # Late cancelled responses carry no current answer.
            if response['id'] != ident:
                raise ProtocolError('unexpected MCP response')
            if 'error' in response:
                raise ProtocolError('MCP request refused')
            return response['result']
        raise ProtocolError('MCP session closed')

    def initialize(self):
        result = self._request('initialize', {'protocolVersion': VERSION, 'capabilities': {},
                                            'clientInfo': {'name': 'thinkthen-consumer', 'version': '0.2'}})
        if (not isinstance(result, dict) or result.get('protocolVersion') != VERSION
                or not isinstance(result.get('capabilities'), dict)
                or not isinstance(result['capabilities'].get('tools'), dict)
                or not isinstance(result.get('serverInfo'), dict)
                or not isinstance(result['serverInfo'].get('name'), str)
                or not isinstance(result['serverInfo'].get('version'), str)):
            raise ProtocolError('unsupported MCP initialization')
        self._send({'jsonrpc': '2.0', 'method': 'notifications/initialized'})
        return result

    def ping(self):
        return self._request('ping')

    def tools(self):
        result = self._request('tools/list')
        if not isinstance(result, dict) or not isinstance(result.get('tools'), list):
            raise ProtocolError('invalid MCP tool catalog')
        return result['tools']

    def cancel(self, request_id):
        if type(request_id) not in (str, int):
            raise ProtocolError('invalid cancellation ID')
        with self._state_lock:
            if request_id == self._active_id and self._active_method == 'initialize':
                raise ProtocolError('initialization cannot be cancelled')
            self._send({'jsonrpc': '2.0', 'method': 'notifications/cancelled',
                        'params': {'requestId': request_id}})
            if request_id == self._active_id:
                self._cancelled_through = request_id
                self._cancelled.set()

    def _tool(self, name, arguments):
        result = self._request('tools/call', {'name': name, 'arguments': arguments})
        try:
            structured = result['structuredContent']
            content = result['content']
            if (not isinstance(structured, dict) or len(content) != 1
                    or content[0]['type'] != 'text'
                    or not _same(_json(content[0]['text']), structured)
                    or type(result.get('isError')) is not bool):
                raise ProtocolError('invalid MCP tool result')
        except (KeyError, TypeError, ValueError, RecursionError):
            raise ProtocolError('invalid MCP tool result') from None
        if result['isError']:
            raise ToolError(structured)
        return structured

    def decide(self, **arguments):
        return self._tool('decide', arguments)

    def choose(self, **arguments):
        return self._tool('choose', arguments)

    def tag(self, **arguments):
        return self._tool('tag', arguments)

    def score(self, **arguments):
        return self._tool('score', arguments)

    def filter(self, **arguments):
        return self._tool('filter', arguments)

    def rank(self, **arguments):
        return self._tool('rank', arguments)

    def find(self, **arguments):
        return self._tool('find', arguments)

    def annotate(self, **arguments):
        return self._tool('annotate', arguments)

    def recognize(self, **arguments):
        return self._tool('recognize', arguments)

    def relate(self, **arguments):
        return self._tool('relate', arguments)

    def close(self):
        self._closed.set()
        if self.process is None:
            if self._reader_thread is not None:
                self._reader_thread.join(timeout=1)
            return
        process, self.process = self.process, None
        try:
            process.stdin.close()
        except (OSError, ValueError):
            pass
        # The server receives EOF and must join its native calls. If it cannot
        # retire, terminate only this owned subprocess, never a process group.
        try:
            process.wait(timeout=10)
        except subprocess.TimeoutExpired:
            process.terminate()
            try:
                process.wait(timeout=2)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait()
        finally:
            process.stdout.close()
            if self._reader_thread is not None:
                self._reader_thread.join(timeout=1)

    def __enter__(self):
        return self

    def __exit__(self, *_):
        self.close()
