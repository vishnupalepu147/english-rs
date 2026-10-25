"""Generates assets/banner.svg. Run: python3 -I assets/banner.py assets/banner.svg /tmp/banner-fixed.svg

The terminal text is real englishc output for "They was right about Rust."
(without the note line), followed by real `englishc --fix` output.
"""
import html
W, H = 1280, 460
mono = "ui-monospace, SFMono-Regular, 'JetBrains Mono', Menlo, Consolas, 'DejaVu Sans Mono', monospace"
serif = "Georgia, 'Times New Roman', 'DejaVu Serif', serif"
sans = "-apple-system, 'Segoe UI', Inter, Roboto, 'Helvetica Neue', Arial, sans-serif"

def esc(s): return html.escape(s, quote=False)

# A line is a list of (text, class) runs.
A = [
    [("$ ", "mut"), ("englishc essay.txt", "fg")],
    [("   ", ""), ("Compiling", "ok"), (" essay.txt (english 2026)", "fg")],
    [("error[E0001]", "err"), (": mismatched verb form", "bold")],
    [(" ", ""), ("-->", "gut"), (" essay.txt:1:6", "fg")],
    [("  |", "gut")],
    [("1 |", "gut"), (" They was right about Rust.", "fg")],
    [("  |", "gut"), (" ----", "gut"), (" ^^^ expected `were`, found `was`", "err")],
    [("  |", "gut"), (" |", "gut")],
    [("  |", "gut"), (" `They` is plural", "gut")],
    [("  |", "gut")],
    [("help", "help"), (": use `were`", "fg")],
    [("  |", "gut")],
    [("1 -", "gut"), (" They ", "fg"), ("was", "del"), (" right about Rust.", "fg")],
    [("1 +", "gut"), (" They ", "fg"), ("were", "add"), (" right about Rust.", "fg")],
]
B = [
    [("$ ", "mut"), ("englishc --fix essay.txt", "fg")],
    [("       ", ""), ("Fixed", "ok"), (" essay.txt (1 fix)", "fg")],
    [("   ", ""), ("Compiling", "ok"), (" essay.txt (english 2026)", "fg")],
    [("    ", ""), ("Finished", "ok"), (" `proofread` profile [grammatical + spell-checked] target(s) in 0.00s", "fg")],
    [],
    [("$ ", "mut"), ("cat essay.txt", "fg")],
    [("They ", "fg"), ("were", "add"), (" right about Rust.", "fg")],
    [("$ ", "mut")],
]

TX, TY, TW, TH = 590, 44, 650, 372
x0, y0, lh = TX + 22, TY + 66, 20

def lines(rows, cls):
    out = [f'<g class="{cls}">']
    for i, runs in enumerate(rows):
        if not runs:
            continue
        spans = "".join(f'<tspan class="{c}">{esc(t)}</tspan>' if c else esc(t) for t, c in runs)
        out.append(f'<text x="{x0}" y="{y0 + i * lh}" xml:space="preserve">{spans}</text>')
    out.append("</g>")
    return "\n    ".join(out)

# Wavy underline under "English,".
sx, sy, step = 64, 218, 9
wave = f"M{sx} {sy} " + " ".join(f"q {step/2} {-5 if k % 2 == 0 else 5} {step} 0" for k in range(34))
wave_len = 34 * step

cursor_y = y0 + 7 * lh

svg = f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" role="img" aria-labelledby="title desc">
  <title id="title">English, rewritten in Rust</title>
  <desc id="desc">The words "English, rewritten in Rust" beside a terminal where englishc reports error E0001 on "They was right about Rust." and then fixes it to "They were right about Rust."</desc>
  <defs>
    <linearGradient id="bg" x1="0" y1="0" x2="1" y2="1">
      <stop offset="0" stop-color="#0d0e12"/>
      <stop offset="1" stop-color="#18100d"/>
    </linearGradient>
    <radialGradient id="glow" cx="0.5" cy="0.5" r="0.5">
      <stop offset="0" stop-color="#f26b3a" stop-opacity="0.22"/>
      <stop offset="1" stop-color="#f26b3a" stop-opacity="0"/>
    </radialGradient>
    <pattern id="dots" width="24" height="24" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#ffffff" fill-opacity="0.06"/>
    </pattern>
    <clipPath id="card"><rect width="{W}" height="{H}" rx="20"/></clipPath>
  </defs>
  <style>
    text {{ font-family: {mono}; font-size: 12.5px; fill: #d7dae0; }}
    .fg {{ fill: #d7dae0; }} .mut {{ fill: #7d8590; }} .bold {{ fill: #f0f3f6; font-weight: 700; }}
    .err {{ fill: #ff6b6b; font-weight: 700; }} .gut {{ fill: #6cb6ff; font-weight: 700; }}
    .ok {{ fill: #7ee787; font-weight: 700; }} .help {{ fill: #56d4dd; font-weight: 700; }}
    .add {{ fill: #7ee787; font-weight: 700; }} .del {{ fill: #ff8a8a; }}
    .title {{ font-family: {serif}; font-size: 86px; fill: #f4efe9; letter-spacing: -1px; }}
    .rust {{ font-family: {mono}; font-size: 44px; font-weight: 700; fill: #f26b3a; letter-spacing: -1px; }}
    .kicker {{ font-size: 15px; fill: #f26b3a; fill-opacity: 0.85; }}
    .tag {{ font-family: {sans}; font-size: 19px; fill: #b3aca4; }}
    .pill {{ font-size: 13px; fill: #d7dae0; }}
    .bar {{ font-family: {sans}; font-size: 12.5px; fill: #7d8590; }}
    .a {{ animation: a 11s infinite; }}
    .b {{ opacity: 0; animation: b 11s infinite; }}
    .squiggle {{ stroke-dasharray: {wave_len + 40}; stroke-dashoffset: {wave_len + 40}; animation: draw 11s infinite; }}
    .straight {{ opacity: 0; animation: b 11s infinite; }}
    .cursor {{ animation: blink 1.1s steps(1) infinite; }}
    @keyframes a {{ 0%, 52% {{ opacity: 1; }} 56%, 96% {{ opacity: 0; }} 100% {{ opacity: 1; }} }}
    @keyframes b {{ 0%, 52% {{ opacity: 0; }} 56%, 96% {{ opacity: 1; }} 100% {{ opacity: 0; }} }}
    @keyframes draw {{ 0%, 4% {{ stroke-dashoffset: {wave_len + 40}; opacity: 1; }} 14%, 52% {{ stroke-dashoffset: 0; opacity: 1; }} 56%, 100% {{ stroke-dashoffset: 0; opacity: 0; }} }}
    @keyframes blink {{ 0%, 50% {{ opacity: 1; }} 51%, 100% {{ opacity: 0; }} }}
    @media (prefers-reduced-motion: reduce) {{
      .a, .b, .squiggle, .straight, .cursor {{ animation: none; }}
      .squiggle {{ stroke-dashoffset: 0; }}
    }}
  </style>
  <g clip-path="url(#card)">
    <rect width="{W}" height="{H}" fill="url(#bg)"/>
    <rect width="{W}" height="{H}" fill="url(#dots)"/>
    <ellipse cx="{TX + TW / 2}" cy="{TY + TH / 2}" rx="520" ry="300" fill="url(#glow)"/>
    <text x="-40" y="600" style="font-family: {serif}; font-size: 640px; fill: #ffffff; fill-opacity: 0.028;">Æ</text>
  </g>
  <rect x="0.5" y="0.5" width="{W - 1}" height="{H - 1}" rx="20" fill="none" stroke="#ffffff" stroke-opacity="0.08"/>

  <text class="kicker" x="62" y="100">#![forbid(bad_grammar)]</text>
  <text class="title" x="58" y="196">English,</text>
  <path class="squiggle" d="{wave}" fill="none" stroke="#ff6b6b" stroke-width="3" stroke-linecap="round"/>
  <path class="straight" d="M{sx} {sy} h {wave_len}" fill="none" stroke="#7ee787" stroke-width="3" stroke-linecap="round"/>
  <text class="rust" x="60" y="276">rewritten in Rust</text>
  <text class="tag" x="62" y="326">Grammar is a trait bound. Pronouns are borrows.</text>
  <text class="tag" x="62" y="354">And a compiler gives prose rustc-grade errors.</text>
  {{pills}}

  <rect x="{TX}" y="{TY}" width="{TW}" height="{TH}" rx="14" fill="#0a0b0e" fill-opacity="0.92" stroke="#ffffff" stroke-opacity="0.1"/>
  <line x1="{TX}" y1="{TY + 36}" x2="{TX + TW}" y2="{TY + 36}" stroke="#ffffff" stroke-opacity="0.07"/>
  <circle cx="{TX + 20}" cy="{TY + 18}" r="6" fill="#ff5f57"/>
  <circle cx="{TX + 40}" cy="{TY + 18}" r="6" fill="#febc2e"/>
  <circle cx="{TX + 60}" cy="{TY + 18}" r="6" fill="#28c840"/>
  <text class="bar" x="{TX + TW / 2}" y="{TY + 22}" text-anchor="middle">~/essay — englishc</text>
    {lines(A, "a")}
    {lines(B, "b")}
  <g class="b"><rect class="cursor" x="{x0 + 2 * 7.5}" y="{cursor_y - 12}" width="8" height="15" fill="#d7dae0"/></g>
</svg>
'''

pills = []
px = 62
for label in ["0 dependencies", "13 error codes", "stable Rust"]:
    w = len(label) * 7.6 + 26
    pills.append(f'<rect x="{px}" y="384" width="{w:.0f}" height="30" rx="15" fill="#15171c" stroke="#ffffff" stroke-opacity="0.12"/>'
                 f'<text class="pill" x="{px + w / 2:.0f}" y="403.5" text-anchor="middle">{label}</text>')
    px += w + 10
svg = svg.replace("{pills}", "\n  ".join(pills))
import sys
open(sys.argv[1], "w").write(svg)
# A static copy showing the fixed state, for checking the layout.
static_b = svg.replace(".a {{", "").replace("animation: a 11s infinite;", "opacity: 0;").replace(".b { opacity: 0; animation: b 11s infinite; }", ".b { opacity: 1; }").replace(".straight { opacity: 0; animation: b 11s infinite; }", ".straight { opacity: 1; }")
open(sys.argv[2], "w").write(static_b)
