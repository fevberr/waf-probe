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
cd waf-probe\core
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

## Legal

Authorized security testing only. `test-bypass` requires `--yes-i-own-this`.

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
