#!/usr/bin/env python3
import sys
import json
import os
import time
from datetime import datetime

def parse_cargo_test_output(input_file):
    tests = []
    passed = 0
    failed = 0
    ignored = 0
    
    try:
        with open(input_file, 'r') as f:
            for line in f:
                try:
                    event = json.loads(line)
                    if event.get('type') == 'test' and event.get('event') in ['ok', 'failed', 'ignored']:
                        tests.append(event)
                        if event['event'] == 'ok':
                            passed += 1
                        elif event['event'] == 'failed':
                            failed += 1
                        elif event['event'] == 'ignored':
                            ignored += 1
                except json.JSONDecodeError:
                    continue
    except FileNotFoundError:
        print(f"Error: Could not find input file {input_file}", file=sys.stderr)
        return None

    return {
        'tests': tests,
        'passed': passed,
        'failed': failed,
        'ignored': ignored,
        'total': passed + failed + ignored
    }

def generate_junit_xml(results, output_file):
    with open(output_file, 'w') as f:
        f.write('<?xml version="1.0" encoding="UTF-8"?>\n')
        f.write('<testsuites>\n')
        f.write(f'  <testsuite name="cargo-tests" tests="{results["total"]}" failures="{results["failed"]}" skipped="{results["ignored"]}">\n')
        
        for test in results['tests']:
            name = test.get('name', 'unknown')
            time_ms = test.get('exec_time', 0)
            # JUnit expects seconds
            time_sec = float(time_ms) if time_ms else 0.0
            
            if test['event'] == 'ok':
                f.write(f'    <testcase name="{name}" time="{time_sec}"/>\n')
            elif test['event'] == 'failed':
                f.write(f'    <testcase name="{name}" time="{time_sec}">\n')
                f.write('      <failure/>\n')
                f.write('    </testcase>\n')
            elif test['event'] == 'ignored':
                f.write(f'    <testcase name="{name}" time="{time_sec}">\n')
                f.write('      <skipped/>\n')
                f.write('    </testcase>\n')
                
        f.write('  </testsuite>\n')
        f.write('</testsuites>\n')

def generate_metrics_json(results, output_file):
    # Get environment variables
    os_name = os.environ.get('MATRIX_OS', 'unknown')
    rust_version = os.environ.get('MATRIX_RUST', 'unknown')
    job_status = os.environ.get('JOB_STATUS', 'unknown')
    
    metrics = {
        "os": os_name,
        "runtime": rust_version,
        "status": job_status,
        "tests_total": results['total'],
        "tests_passed": results['passed'],
        "tests_failed": results['failed'],
        "tests_skipped": results['ignored'],
        "duration": int(os.environ.get('DURATION', 0)),
        "cache_hit": os.environ.get('CACHE_HIT') == 'true',
        "timestamp": datetime.utcnow().isoformat()
    }
    
    with open(output_file, 'w') as f:
        json.dump(metrics, f, indent=2)

def main():
    if len(sys.argv) < 2:
        print("Usage: process_tests.py <input_json_file>")
        sys.exit(1)
        
    input_file = sys.argv[1]
    results = parse_cargo_test_output(input_file)
    
    if results is None:
        sys.exit(1)
        
    # Generate outputs
    os.makedirs('test-results', exist_ok=True)
    generate_junit_xml(results, 'test-results/results.xml')
    
    # Generate metrics if requested
    if os.environ.get('GENERATE_METRICS') == 'true':
        generate_metrics_json(results, 'test-metrics.json')
    
    # Print summary to stdout
    print(f"Tests: {results['total']} | Passed: {results['passed']} | Failed: {results['failed']} | Ignored: {results['ignored']}")
    
    # Exit with failure if any tests failed
    if results['failed'] > 0:
        sys.exit(1)

if __name__ == '__main__':
    main()
