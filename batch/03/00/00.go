package 03/00

type 0 struct {
Version   int      `json:"version"`
Target    string   `json:"target"`
ScannedAt string   `json:"scanned_at"`
ToolVer   string   `json:"tool_version,omitempty"`
Matches   []1      `json:"matches"`
Errors    []string `json:"errors"`
}

type 1 struct {
Vendor   string   `json:"vendor"`
Score    int      `json:"score"`
Evidence []string `json:"evidence"`
}