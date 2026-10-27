#!/usr/bin/env python3
"""Capture Safari on macOS. Requires Safari's Allow Remote Automation setting."""

import argparse
import json
import pathlib
import re
import subprocess
import time
import urllib.error
import urllib.request

from capture_server import capture_server


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manual", action="store_true", help="open a local capture page in ordinary Safari")
    args = parser.parse_args()

    def webdriver(path, data=None, method=None):
        request = urllib.request.Request(
            "http://127.0.0.1:4445" + path,
            data=None if data is None else json.dumps(data).encode(),
            headers={"Content-Type": "application/json"}, method=method,
        )
        try:
            with urllib.request.urlopen(request, timeout=30) as response:
                return json.load(response)["value"]
        except urllib.error.HTTPError as error:
            raise RuntimeError(error.read().decode()) from error

    with capture_server() as (server, received):
        driver = None if args.manual else subprocess.Popen(["/usr/bin/safaridriver", "--port", "4445"])
        session = None
        try:
            if args.manual:
                subprocess.run(["open", "-a", "Safari", f"http://localhost:{server.server_port}/uagent-capture"], check=True)
            for _ in range(0 if args.manual else 50):
                try:
                    webdriver("/status")
                    break
                except urllib.error.URLError:
                    time.sleep(0.1)
            if not args.manual:
                session = webdriver("/session", {"capabilities": {"alwaysMatch": {"browserName": "safari"}}})["sessionId"]
                webdriver(f"/session/{session}/url", {"url": f"http://localhost:{server.server_port}/uagent-capture"})
            headers = received.get(timeout=30)
            version = int(re.search(r"Version/(\d+)", dict(headers)["user-agent"])[1])
            output = pathlib.Path(f"tests/captures/safari-{version}-macos.json")
            output.write_text(json.dumps({
                "browser": "Safari", "version": version, "platform": "MacOs",
                "source": "Safari direct navigation to local capture server" + (" (manual)" if args.manual else " (safaridriver)"),
                "headers": headers,
            }, indent=2) + "\n")
            print(output)
        finally:
            if session:
                webdriver(f"/session/{session}", method="DELETE")
            if driver:
                driver.terminate()
                driver.wait(timeout=10)


if __name__ == "__main__":
    main()
