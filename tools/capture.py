#!/usr/bin/env python3
"""Capture a real browser's direct navigation through a local HTTP proxy.

Uses only Python's standard library. Run headed (under Xvfb on Linux): Chrome's
headless User-Agent identifies HeadlessChrome and is not a browser profile.
Each browser gets a disposable profile; existing browser sessions are untouched.
"""

import argparse
import json
import pathlib
import platform
import re
import subprocess
import tempfile

from capture_server import capture_server


def capture(browser, binary, output):
    with capture_server() as (server, received):
        port = server.server_port
        with tempfile.TemporaryDirectory(prefix="uagent-") as profile:
            if browser == "firefox":
                prefs = {
                    "network.proxy.type": 1,
                    "network.proxy.http": "127.0.0.1",
                    "network.proxy.http_port": port,
                    "network.proxy.no_proxies_on": "",
                    "network.proxy.allow_hijacking_localhost": True,
                    "browser.shell.checkDefaultBrowser": False,
                    "browser.startup.homepage_override.mstone": "ignore",
                    "intl.accept_languages": "en-US,en",
                }
                pathlib.Path(profile, "user.js").write_text("\n".join(
                    f"user_pref({json.dumps(k)}, {json.dumps(v)});"
                    for k, v in prefs.items()
                ))
                command = [binary, "--no-remote", "--profile", profile]
            else:
                defaults = pathlib.Path(profile, "Default")
                defaults.mkdir()
                (defaults / "Preferences").write_text(json.dumps({"intl": {"accept_languages": "en-US,en"}}))
                command = [
                    binary, f"--user-data-dir={profile}", "--no-first-run",
                    "--no-default-browser-check", "--disable-background-networking",
                    f"--proxy-server=http://127.0.0.1:{port}",
                    "--proxy-bypass-list=<-loopback>", "--lang=en-US",
                ]
            process = subprocess.Popen(
                [*command, "http://localhost/uagent-capture"],
                stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
            )
            try:
                headers = received.get(timeout=60)
            finally:
                process.terminate()
                try:
                    process.wait(timeout=10)
                except subprocess.TimeoutExpired:
                    process.kill()
                    process.wait()
    ua = dict(headers)["user-agent"]
    token = {"chrome": "Chrome", "edge": "Edg", "firefox": "Firefox"}[browser]
    version = int(re.search(rf"{token}/(\d+)", ua)[1])
    if "HeadlessChrome" in ua:
        raise ValueError("capture a headed browser, under Xvfb in CI")
    capture_platform = {"Darwin": "MacOs", "Linux": "Linux", "Windows": "Windows"}[platform.system()]
    data = {
        "browser": browser.capitalize(), "version": version,
        "platform": capture_platform,
        "source": "Real browser, disposable profile, direct navigation through tools/capture.py proxy",
        "headers": headers,
    }
    output.mkdir(parents=True, exist_ok=True)
    path = output / f"{browser}-{version}-{capture_platform.lower()}.json"
    path.write_text(json.dumps(data, indent=2) + "\n")
    print(path)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("browser", choices=["chrome", "edge", "firefox"])
    parser.add_argument("--binary", required=True)
    parser.add_argument("--output", type=pathlib.Path, default=pathlib.Path("tests/captures"))
    args = parser.parse_args()
    capture(args.browser, args.binary, args.output)
