<img src="https://p16-tiktok-dm-sticker-sign-sg.ibyteimg.com/tos-alisg-i-dhq7zx4c1p-sg/d179a29e560642bba3707aa2ec9babd8~tplv-dhq7zx4c1p-full.awebp?rk3s=00edd399&x-expires=1792029789&x-signature=qrJJe26DGlzonOoaKdczYDcF0BE%3D" alt="RAHHHH">

### RAHHHH
**WAF fingerprinter + bypass suggester.**
Cloudflare · Akamai · AWS WAF · Imperva · F5 · ModSecurity — no exploits, markers only.

---

**Stack**

<p>
  <img src="https://cdn.simpleicons.org/rust" alt="Rust" width="20">
  <img src="https://cdn.simpleicons.org/go" alt="Go" width="20">
  <img src="https://cdn.simpleicons.org/python" alt="Python" width="20">
  <img src="https://cdn.simpleicons.org/yaml" alt="YAML" width="20">
</p>

---

**Author** — [fevberr](https://github.com/fevberr) *(super coolz guy btw)*

---

## Install

```powershell
git clone https://github.com/fevberr/waf-probe.git
cd waf-probe
cargo build --release
```

---

## Use

```powershell
.\target\release\waf-probe.exe scan https://example.com
.\target\release\waf-probe.exe test-bypass https://your-app.example/search --param q --yes-i-own-this
```

---

## Fingerprint DB

`signatures/00.yaml` — PRs welcome.

```yaml
- name: Cloudflare
  weight: 3
  headers:
    server: '(?i)^cloudflare$'
    cf-ray: '.+'
  body: '(?i)(cloudflare|Error 1020|Ray ID)'
```

---

## License

MIT © 2026 fevberr

```
MIT License

Copyright (c) 2026 fevberr

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## Usage (expanded)

Run `scan` from the repo root so the default `signatures/00.yaml` resolves.

**Options**

| Flag | Description |
| --- | --- |
| `--signatures <path>` | Use a different signature file (default: `signatures/00.yaml`) |
| `--json` | Emit machine-readable JSON instead of a text report |

**Examples**

```powershell
# Basic scan
.\target\release\waf-probe.exe scan https://example.com

# JSON output for scripting
.\target\release\waf-probe.exe scan https://example.com --json

# Custom signature file
.\target\release\waf-probe.exe scan https://example.com --signatures .\signatures\00.yaml
```

**Sample output**

```
waf-probe scan: https://example.com
HTTP status: 200 OK

Matched 1 signature(s):

  [HIT] Cloudflare (weight 3)
        - header server: cloudflare
        - header cf-ray: a3b9b210dde7dc77-GUA
        - header cf-cache-status: HIT
```

---

## Changelog

### 0.1.0 - 2026-09-15
- Initial working release.
- `scan <url>` command: fetches a URL and matches the response against WAF
  signatures in `signatures/00.yaml` (headers, cookies, body).
- Signatures: Cloudflare, Akamai, AWS WAF/CloudFront, Imperva/Incapsula,
  F5 BIG-IP ASM, ModSecurity/CRS, Sucuri, Barracuda, Fortinet FortiWeb,
  Wallarm, Azure Front Door, Google Cloud Armor, Fastly, Cloudflare Turnstile.
- `--json` output for scripting.
- `--signatures <path>` to override the signature file.
- Weights per signature; hits sorted by weight.
