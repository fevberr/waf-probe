<img src="https://p16-tiktok-dm-sticker-sign-sg.ibyteimg.com/tos-alisg-i-dhq7zx4c1p-sg/d179a29e560642bba3707aa2ec9babd8~tplv-dhq7zx4c1p-full.awebp?rk3s=00edd399&x-expires=1792029789&x-signature=qrJJe26DGlzonOoaKdczYDcF0BE%3D" alt="RAHHHH">

# waf-probe

**Point it at a URL. It tells you which WAF is in front of it.**

`waf-probe` sends one request, inspects the response headers, cookies, and
body, and matches them against a database of WAF fingerprints. It prints
which WAF it thinks you are behind and the evidence for that call.

No exploits, no payloads, no attacks 
—
 just identification. It is a
fingerprinting tool, not a scanner.

Supported WAFs: Cloudflare, Akamai, AWS WAF / CloudFront, Imperva / Incapsula,
F5 BIG-IP ASM, ModSecurity / CRS, Sucuri, Barracuda, Fortinet FortiWeb,
Wallarm, Azure Front Door, Google Cloud Armor, Fastly, Cloudflare Turnstile.

---

## Quick start

```powershell
git clone https://github.com/fevberr/waf-probe.git
cd waf-probe
cargo build --release
.\target\release\waf-probe.exe scan https://example.com
```

That is it. One command to build, one to run.

---

## What the output looks like

```
waf-probe scan: https://example.com
HTTP status: 200 OK

Matched 1 signature(s):

  [HIT] Cloudflare (weight 3)
        - header server: cloudflare
        - header cf-ray: a3b9b210dde7dc77-GUA
        - header cf-cache-status: HIT
```

Each hit lists the WAF name, a weight (how confident the match is), and the
exact headers, cookies, or body text that triggered it. If nothing matches:

```
No known WAF signature matched.
```

---

## Usage

```
waf-probe scan <url> [options]
```

| Option | Description |
| --- | --- |
| `--signatures <path>` | Use a different signature file (default: `signatures/00.yaml`) |
| `--json` | Print machine-readable JSON instead of a text report |

**Examples**

```powershell
# Basic scan
.\target\release\waf-probe.exe scan https://example.com

# JSON for scripting
.\target\release\waf-probe.exe scan https://example.com --json

# Custom signature database
.\target\release\waf-probe.exe scan https://example.com --signatures .\signatures\00.yaml
```

Run from the repo root so the default `signatures/00.yaml` is found.

---

## How detection works

Every signature in `signatures/00.yaml` has four optional parts:

| Field | What it checks |
| --- | --- |
| `headers` | Response header name -> regex on its value |
| `cookies` | Cookie name regex -> regex on its value |
| `body` | Regex on the response body |
| `weight` | Confidence score (higher = stronger signal) |

A signature matches if **any** of its header, cookie, or body rules match.
Hits are sorted by weight, so the most confident match is first.

---

## Adding a signature

Open `signatures/00.yaml` and add an entry. Example:

```yaml
- name: My WAF
  weight: 3
  headers:
    server: '(?i)^my-waf$'
    x-my-waf-id: '.+'
  cookies:
    'my_waf_.*': '.+'
  body: '(?i)(blocked by My WAF)'
```

Only `name` is required; `weight` defaults to 1. Pull requests welcome.

---

## FAQ

**Does it attack the target?**
No. It sends a single normal GET request and reads the response.

**Why do I need to run it from the repo root?**
The default signature path is `signatures/00.yaml`, relative to the current
directory. Use `--signatures <path>` to point elsewhere.

**Why did nothing match?**
Either there is no WAF in front of the site, or it hides its headers. Add a
signature if you spot a pattern.

**Is it fast?**
One HTTP request plus regex matching. Milliseconds.

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

## Changelog

### 0.1.0 - 2026-09-15
- Initial working release.
- `scan <url>` command: fetches a URL and matches the response against WAF
  signatures in `signatures/00.yaml` (headers, cookies, body).
- `--json` output for scripting.
- `--signatures <path>` to override the signature file.
- Weights per signature; hits sorted by weight.
