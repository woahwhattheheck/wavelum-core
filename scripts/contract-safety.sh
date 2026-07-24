#!/bin/bash
# Contract Safety Verification Script
# Checks for common Soroban contract vulnerabilities and unsafe patterns

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

FAILED=0
WARNINGS=0

echo "=========================================="
echo "  Soroban Contract Safety Verification"
echo "=========================================="
echo ""

# Test 1: Check for unwrap/expect in production code
echo -e "${YELLOW}[1/5]${NC} Scanning for unwrap()/expect() in production code..."

# Find all Rust files in contracts (exclude test files)
UNWRAP_ISSUES=$(find contracts -name "*.rs" ! -path "*/tests/*" ! -path "*/test_*" -type f -exec grep -l "\.unwrap()\|\.expect(" {} \; 2>/dev/null || true)

if [ -n "$UNWRAP_ISSUES" ]; then
    echo -e "${RED}✗ Found unwrap()/expect() calls in production code:${NC}"
    echo "$UNWRAP_ISSUES" | while read -r file; do
        LINES=$(grep -n "\.unwrap()\|\.expect(" "$file" | grep -v "test" || true)
        if [ -n "$LINES" ]; then
            echo -e "${RED}  $file:${NC}"
            echo "$LINES" | sed 's/^/    /'
        fi
    done
    FAILED=$((FAILED + 1))
else
    echo -e "${GREEN}✓ No unwrap()/expect() found in production code${NC}"
fi

echo ""

# Test 2: Run clippy with strict settings
echo -e "${YELLOW}[2/5]${NC} Running Clippy with strict safety settings..."

if cargo clippy --all-targets --all-features -- -D warnings -W clippy::all -W clippy::pedantic 2>&1 | tee /tmp/clippy-output.txt; then
    echo -e "${GREEN}✓ Clippy checks passed${NC}"
else
    echo -e "${RED}✗ Clippy found issues${NC}"
    FAILED=$((FAILED + 1))
fi

echo ""

# Test 3: Check for missing authorization in public functions
echo -e "${YELLOW}[3/5]${NC} Scanning for public functions without authorization checks..."

MISSING_AUTH=$(grep -r "pub fn" contracts --include="*.rs" \
    | grep -v "test" \
    | grep -v "\/\/" \
    | grep -v "#\[contractimpl\]" \
    | grep -v "impl" || true)

if [ -n "$MISSING_AUTH" ]; then
    # This is a warning, as some public functions may be helpers
    echo -e "${YELLOW}⚠ Review these public functions for access control:${NC}"
    echo "$MISSING_AUTH" | head -20 | sed 's/^/  /'
    WARNINGS=$((WARNINGS + 1))
else
    echo -e "${GREEN}✓ No suspicious public functions found${NC}"
fi

echo ""

# Test 4: Run formal reentrancy verification
echo -e "${YELLOW}[4/5]${NC} Running formal reentrancy verification tests..."

if cargo test -p vesting_contracts formal_reentrancy -- --nocapture --test-threads=1 2>&1; then
    echo -e "${GREEN}✓ Formal reentrancy tests passed${NC}"
else
    echo -e "${RED}✗ Formal reentrancy tests failed${NC}"
    FAILED=$((FAILED + 1))
fi

echo ""

# Test 5: Check code formatting
echo -e "${YELLOW}[5/5]${NC} Checking code formatting..."

if cargo fmt --all -- --check 2>&1; then
    echo -e "${GREEN}✓ Code formatting check passed${NC}"
else
    echo -e "${RED}✗ Code formatting issues found${NC}"
    FAILED=$((FAILED + 1))
fi

echo ""
echo "=========================================="

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✓ All safety checks passed!${NC}"
    exit 0
else
    echo -e "${RED}✗ $FAILED safety check(s) failed${NC}"
    exit 1
fi
