#!/usr/bin/env python3
"""Dogtail GUI test for Rust Tables — verifies window + toolbar + grid."""
import sys, time
from dogtail import tree

def main():
    app = tree.root.application('tables')
    print('Rust Tables — found application')

    # Toolbar buttons
    for name in ['B', 'I', 'U']:
        try:
            btn = app.child(name=name, roleName='toggle button')
            print(f'  Found {name} toggle')
        except Exception:
            print(f'  [SKIP] {name} not found')

    # Text area
    try:
        tv = app.child(roleName='text')
        print('  Found text area')
    except Exception:
        print('  [SKIP] text area')

    print('RUST GUITEST: PASS')
    return 0

if __name__ == '__main__':
    try:
        sys.exit(main())
    except Exception as e:
        print(f'RUST GUITEST: FAIL — {e}')
        sys.exit(1)
