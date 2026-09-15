package 04/00

import (
"context"
"encoding/json"
"fmt"
"os/exec"
"time"

"github.com/you/waf-probe/batch/03/00"
)

func 0(ctx context.Context, target string, timeout float64, corePath string) (*03/00.0, error) {
cctx, cancel := context.WithTimeout(ctx, time.Duration(timeout*1.5*float64(time.Second)))
defer cancel()
cmd := exec.CommandContext(cctx, corePath, "scan", "--json", "--timeout",
fmt.Sprintf("%.1f", timeout), target)
out, err := cmd.Output()
if err != nil {
return nil, fmt.Errorf("core: %w", err)
}
var rep 03/00.0
if err := json.Unmarshal(out, &rep); err != nil {
return nil, err
}
return &rep, nil
}