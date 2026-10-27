# Browser captures

The initial captures are Chrome 156, Edge 156, Firefox 154 and Safari 26 on
macOS. Tests compare every profile header against these captures. Safari retains
its captured minor/patch suffix. Desktop and mobile platform variants originate
from the Spire draft; only the checked-in browser/platform captures have been
verified against real browsers. Mobile profiles describe phones. iOS Chrome,
Edge and Firefox and Android Edge are not modeled.

To refresh installed browsers from the repository root:

```sh
python3 tools/capture.py chrome --binary /path/to/chrome
python3 tools/capture.py edge --binary /path/to/msedge
python3 tools/capture.py firefox --binary /path/to/firefox
python3 tools/capture_safari.py
python3 tools/update.py
cargo test -p uagents
```

The Python tools use only the standard library. Chrome, Edge and Firefox get
disposable profiles and navigate through a local proxy. Use Xvfb on Linux for
headed Chromium, whose User-Agent differs from `HeadlessChrome`. Safari requires
macOS and Allow Remote Automation; `capture_safari.py --manual` opens a local
capture page in normal Safari instead. Close that page after recording.

The browser-data workflow captures stable desktop browsers weekly and opens a
review PR. It never runs during Cargo builds. Review fixture changes, regenerate
the data, and publish them in a patch release. Older captures remain as provenance;
tests pin the selected latest capture rather than claiming historical fidelity.
The repository must allow Actions to create pull requests. An optional
`BROWSER_DATA_TOKEN` secret can trigger normal PR workflows on these bot changes;
the capture workflow itself runs tests before opening its PR with the default token.

MIT License; see the repository LICENSE.
