package 04/01

import (
"bufio"
"os"
)

func 1(path string) ([]string, error) {
f := os.Stdin
if path != "-" {
var err error
f, err = os.Open(path)
if err != nil {
return nil, err
}
defer f.Close()
}
var out []string
s := bufio.NewScanner(f)
for s.Scan() {
line := s.Text()
if line == "" {
continue
}
out = append(out, line)
}
return out, s.Err()
}