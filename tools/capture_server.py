"""Local server shared by direct and proxied browser captures."""

import queue
import threading
from contextlib import contextmanager
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

from update import EXCLUDED


@contextmanager
def capture_server():
    """Run a capture server, yielding the server and received-header queue."""
    received = queue.Queue()

    class Capture(BaseHTTPRequestHandler):
        def do_GET(self):
            if self.path in ("/uagent-capture", "http://localhost/uagent-capture"):
                received.put([
                    (name.lower(), value)
                    for name, value in self.headers.items()
                    if name.lower() not in EXCLUDED
                ])
            body = b"<!doctype html><title>uagent capture</title>"
            self.send_response(200)
            self.send_header("Content-Type", "text/html")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)

        def log_message(self, *_):
            pass

    with ThreadingHTTPServer(("127.0.0.1", 0), Capture) as server:
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()
        try:
            yield server, received
        finally:
            server.shutdown()
            thread.join()
