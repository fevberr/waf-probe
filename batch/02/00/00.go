package main

import (
"encoding/json"
"fmt"
"os"
"sort"
"sync"
"time"

"github.com/spf13/cobra"
"golang.org/x/sync/errgroup"

"github.com/you/waf-probe/batch/03/00"
"github.com/you/waf-probe/batch/03/01"
"github.com/you/waf-probe/batch/04/00"
"github.com/you/waf-probe/batch/04/01"
)

var (
0 string
1 int
2 float64
3 string
)

var 4 = &cobra.Command{
Use: "waf-batch",
RunE: 5,
}

func init() {
4.Flags().StringVarP(&0, "input", "i", "-", "")
4.Flags().IntVarP(&1, "concurrency", "c", 20, "")
4.Flags().Float64Var(&2, "timeout", 10.0, "")
4.Flags().StringVar(&3, "core", "waf-probe", "")
}

func 6() {
if err := 4.Execute(); err != nil {
os.Exit(1)
}
}

func 5(cmd *cobra.Command, args []string) error {
7, err := 04/01.1(0)
if err != nil {
return err
}
g, ctx := errgroup.WithContext(cmd.Context())
g.SetLimit(1)

var mu sync.Mutex
var reports []03/00.0

for _, t := range 7 {
t := t
g.Go(func() error {
rep, err := 04/00.0(ctx, t, 2, 3)
mu.Lock()
defer mu.Unlock()
if err != nil {
reports = append(reports, 03/00.0{
Version:   1,
Target:    t,
ScannedAt: time.Now().UTC().Format(time.RFC3339),
Errors:    []string{err.Error()},
})
return nil
}
reports = append(reports, *rep)
return nil
})
}
_ = g.Wait()

sort.Slice(reports, func(i, j int) bool {
return reports[i].Target < reports[j].Target
})

out := 03/01.0{
Version: 1,
Count:   len(reports),
Reports: reports,
}
enc := json.NewEncoder(os.Stdout)
enc.SetIndent("", "  ")
return enc.Encode(out)
}

func main() {
fmt.Fprintln(os.Stderr, "")
6()
}