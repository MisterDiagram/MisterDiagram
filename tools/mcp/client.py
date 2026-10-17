"""Small newline-delimited MCP stdio client. Contributor-original, MIT OR Apache-2.0."""
import json
from pathlib import Path
import queue
import subprocess
import threading

MAX_MESSAGE = 4 * 1024 * 1024


class Client:
    def __init__(self, binary):
        self.process = subprocess.Popen([str(binary), 'mcp'], stdin=subprocess.PIPE,
                                        stdout=subprocess.PIPE, text=True, encoding='utf-8')
        self.replies = queue.Queue(maxsize=2)
        self.next_id = 0
        threading.Thread(target=self._read, daemon=True).start()
        try:
            reply = self.request('initialize', {'protocolVersion': '2025-06-18', 'capabilities': {},
                                 'clientInfo': {'name': 'misterdiagram-python', 'version': '1'}})
            if reply['serverInfo']['name'] != 'misterdiagram':
                raise RuntimeError('unexpected MCP server')
            self._send({'jsonrpc': '2.0', 'method': 'notifications/initialized'})
        except BaseException:
            self.close()
            raise

    def _read(self):
        try:
            while True:
                line = self.process.stdout.readline(MAX_MESSAGE + 1)
                if not line:
                    raise RuntimeError('MCP server closed stdout')
                if len(line.encode('utf-8')) > MAX_MESSAGE:
                    raise RuntimeError('MCP response exceeds 4 MiB')
                self.replies.put(json.loads(line))
        except Exception as error:
            self.replies.put(error)

    def _send(self, value):
        line = json.dumps(value, separators=(',', ':'), ensure_ascii=True) + '\n'
        if len(line) > MAX_MESSAGE:
            raise ValueError('MCP request exceeds 4 MiB')
        self.process.stdin.write(line)
        self.process.stdin.flush()

    def request(self, method, params):
        self.next_id += 1
        self._send({'jsonrpc': '2.0', 'id': self.next_id, 'method': method, 'params': params})
        try:
            reply = self.replies.get(timeout=120)
        except queue.Empty as error:
            raise TimeoutError('MCP response timeout') from error
        if isinstance(reply, Exception):
            raise reply
        if reply.get('id') != self.next_id or 'error' in reply:
            raise RuntimeError(f'MCP protocol error: {reply}')
        return reply['result']

    def tool(self, name, **arguments):
        result = self.request('tools/call', {'name': name, 'arguments': arguments})
        text = result['content'][0]['text']
        if result.get('isError'):
            raise RuntimeError(text)
        return json.loads(text)

    def command(self, command, **params):
        return self.tool('run_command', command=command, params=params)

    def close(self):
        if self.process.stdin:
            self.process.stdin.close()
        try:
            code = self.process.wait(timeout=15)
        except subprocess.TimeoutExpired:
            self.process.kill()
            self.process.wait()
            raise TimeoutError('MCP server did not exit after EOF')
        finally:
            self.process.stdout.close()
        if code:
            raise RuntimeError(f'MCP server exited {code}')

    def __enter__(self):
        return self

    def __exit__(self, *args):
        self.close()


def default_binary():
    return Path(__file__).resolve().parents[2] / 'target' / 'debug' / ('misterdiagram.exe' if __import__('os').name == 'nt' else 'misterdiagram')
