#!/usr/bin/env python3
import sys
import json
import os
import glob
from datetime import timedelta

def format_duration(seconds):
    return str(timedelta(seconds=int(seconds)))

def format_size(size_bytes):
    if size_bytes is None:
        return "N/A"
    for unit in ['B', 'KB', 'MB', 'GB']:
        if size_bytes < 1024.0:
            return f"{size_bytes:.1f} {unit}"
        size_bytes /= 1024.0
    return f"{size_bytes:.1f} TB"

def main():
    metrics = {}
    
    # Read all metric files
    for filename in glob.glob("metrics/*.json"):
        try:
            with open(filename, 'r') as f:
                data = json.load(f)
                # Key by OS and Runtime
                key = (data.get('os', 'unknown'), data.get('runtime', 'unknown'))
                if key not in metrics:
                    metrics[key] = {}
                metrics[key].update(data)
        except Exception as e:
            print(f"Warning: Failed to read {filename}: {e}", file=sys.stderr)

    # Generate Markdown
    print("# 🚀 Build Summary")
    print("")
    
    # Table Header
    print("| OS | Runtime | Status | Tests | Build Size | Duration | Cache |")
    print("|:---|:--------|:------:|:------|:-----------|:---------|:-----:|")
    
    # Sort keys: OS first, then Runtime (stable < beta < nightly)
    def sort_key(k):
        os_order = {'ubuntu-latest': 0, 'macos-latest': 1, 'windows-latest': 2}
        rust_order = {'stable': 0, 'beta': 1, 'nightly': 2}
        return (os_order.get(k[0], 99), rust_order.get(k[1], 99))

    for os_name, runtime in sorted(metrics.keys(), key=sort_key):
        data = metrics[(os_name, runtime)]
        
        # Status Icon
        status = "✅" if data.get('status') == 'success' else "❌"
        
        # Tests
        total = data.get('tests_total', 0)
        passed = data.get('tests_passed', 0)
        failed = data.get('tests_failed', 0)
        skipped = data.get('tests_skipped', 0)
        
        if total == 0:
            tests_display = "N/A"
        else:
            if failed > 0:
                tests_display = f"❌ {passed}/{total} ({failed} failed)"
            else:
                tests_display = f"✅ {passed}/{total}"
                if skipped > 0:
                    tests_display += f" ({skipped} skip)"

        # Size
        size = format_size(data.get('size_bytes'))
        
        # Duration
        duration = format_duration(data.get('duration', 0))
        
        # Cache
        cache = "✅" if data.get('cache_hit') else "❌"
        
        print(f"| {os_name} | {runtime} | {status} | {tests_display} | {size} | {duration} | {cache} |")

    print("")
    print("### 🔍 Details")
    print(f"- **Total Jobs**: {len(metrics)}")
    
if __name__ == '__main__':
    main()
