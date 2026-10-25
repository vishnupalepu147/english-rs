"""Build the playground: compile englishc to wasm, run the crate examples
through rustc for real, and write everything to playground/dist/.

Run from anywhere with: python3 -I playground/build.py
"""

import base64
import json
import pathlib
import re
import shutil
import subprocess

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parent
DIST = HERE / "dist"
WORK = ROOT / "target" / "playground-examples"


def run(cmd, cwd, env=None):
    return subprocess.run(cmd, cwd=cwd, capture_output=True, text=True, env=env)


def build_wasm():
    result = run(["cargo", "build", "--release", "--target", "wasm32-unknown-unknown", "-p", "englishc", "--lib"], ROOT)
    if result.returncode != 0:
        raise SystemExit(result.stderr)
    wasm = ROOT / "target" / "wasm32-unknown-unknown" / "release" / "englishc.wasm"
    encoded = base64.b64encode(wasm.read_bytes()).decode()
    (DIST / "englishc.wasm.js").write_text(f"window.ENGLISHC_WASM = \"{encoded}\";\n")
    return wasm.stat().st_size


def tidy(stderr):
    """Keep rustc's words; drop cargo chatter, local paths and long lists."""
    lines = []
    skipping = False
    for line in stderr.splitlines():
        if re.match(r"^\s+(Compiling|Locking|Adding|Blocking)\b", line):
            continue
        if line.startswith(("For more information", "error: could not compile")):
            continue
        if "the following other types implement trait" in line:
            lines.append("⋮ (trimmed)")
            skipping = True
            continue
        if skipping:
            if line.startswith(("error", "warning")):
                skipping = False
            else:
                continue
        lines.append(line.replace(str(ROOT) + "/", ""))
    return "\n".join(lines).strip("\n")


def build_examples():
    crate = WORK / "demo"
    (crate / "src").mkdir(parents=True, exist_ok=True)
    (crate / "Cargo.toml").write_text(
        '[package]\nname = "demo"\nversion = "0.1.0"\nedition = "2024"\n\n'
        f'[dependencies]\nenglish = {{ path = "{ROOT / "crates" / "english"}" }}\n\n[workspace]\n'
    )
    out = []
    for path in sorted((HERE / "examples").glob("*.rs")):
        code = path.read_text()
        (crate / "src" / "main.rs").write_text(code)
        result = run(["cargo", "run", "--color", "never", "-q"], crate)
        text = tidy(result.stderr)
        if result.returncode == 0:
            text = (text + "\n" if text else "") + "$ cargo run -q\n" + result.stdout.rstrip()
        out.append({"name": path.stem.split("-", 1)[1], "code": code.rstrip(), "out": text, "ok": result.returncode == 0})
    (DIST / "examples.js").write_text("window.EXAMPLES_OUT = " + json.dumps(out, indent=1) + ";\n")
    return out


def main():
    DIST.mkdir(exist_ok=True)
    size = build_wasm()
    examples = build_examples()
    shutil.copy(HERE / "index.html", DIST / "index.html")
    shutil.copy(ROOT / "assets" / "banner.svg", DIST / "banner.svg")
    print(f"englishc.wasm: {size} bytes; examples: " + ", ".join(f"{e['name']} ({'ok' if e['ok'] else 'error'})" for e in examples))


main()
