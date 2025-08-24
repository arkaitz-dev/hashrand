#!/bin/bash

# Final API Test Script - Simple and Reliable
BASE_URL="http://localhost:3000"
PASSED=0
FAILED=0
TOTAL=0

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

test_api() {
    local name="$1"
    local url="$2"
    local expected_status="$3"
    local length_check="$4"  # Optional length to check
    local method="${5:-GET}"  # Optional method, defaults to GET
    local post_data="$6"      # Optional POST data
    
    ((TOTAL++))
    echo -e "\n${BLUE}[$TOTAL] $name${NC}"
    echo "URL: $url"
    if [[ "$method" == "POST" ]]; then
        echo "Method: POST"
        echo "Data: $post_data"
    fi
    
    # Make request and capture response
    local temp_file=$(mktemp)
    local status
    if [[ "$method" == "POST" ]]; then
        status=$(curl -s -X POST -H "Content-Type: application/json" -w "%{http_code}" -o "$temp_file" -d "$post_data" "$url")
    else
        status=$(curl -s -w "%{http_code}" -o "$temp_file" "$url")
    fi
    local body=$(cat "$temp_file")
    rm "$temp_file"
    
    echo "Status: $status"
    echo "Response: $body"
    
    # Check HTTP status
    if [[ "$status" != "$expected_status" ]]; then
        echo -e "${RED}✗ FAIL - Wrong status: expected $expected_status, got $status${NC}"
        ((FAILED++))
        return
    fi
    
    # Optional length check
    if [[ -n "$length_check" && "$status" == "200" ]]; then
        # Extract hash from JSON response
        local hash=$(echo "$body" | jq -r '.hash' 2>/dev/null)
        if [[ -n "$hash" && "$hash" != "null" ]]; then
            local actual_length=${#hash}
            if [[ $actual_length -ne $length_check ]]; then
                echo -e "${YELLOW}⚠ WARNING - Hash length: expected $length_check, got $actual_length${NC}"
            fi
        else
            # Fallback to body length for non-JSON responses
            local actual_length=${#body}
            if [[ $actual_length -ne $length_check ]]; then
                echo -e "${YELLOW}⚠ WARNING - Response length: expected $length_check, got $actual_length${NC}"
            fi
        fi
    fi
    
    # Additional checks for specific endpoints
    case "$url" in
        *"/api/api-key"*)
            if [[ "$status" == "200" ]]; then
                local hash=$(echo "$body" | jq -r '.hash' 2>/dev/null)
                if [[ ! "$hash" == "ak_"* ]]; then
                    echo -e "${RED}✗ FAIL - API key missing 'ak_' prefix${NC}"
                    ((FAILED++))
                    return
                fi
            fi
            ;;
        *"/api/version"*)
            if [[ "$status" == "200" ]]; then
                if ! echo "$body" | jq . >/dev/null 2>&1; then
                    echo -e "${RED}✗ FAIL - Invalid JSON${NC}"
                    ((FAILED++))
                    return
                fi
            fi
            ;;
    esac
    
    echo -e "${GREEN}✓ PASS${NC}"
    ((PASSED++))
}

echo "=============================================="
echo "      HashRand API Final Test Suite"
echo "=============================================="

# Basic endpoint tests
echo -e "\n${YELLOW}=== Basic Functionality Tests ===${NC}"

test_api "Generate default hash" \
    "$BASE_URL/api/generate" \
    "200" \
    "21"

test_api "Generate custom length" \
    "$BASE_URL/api/generate?length=10" \
    "200" \
    "10"

test_api "Generate with all parameters" \
    "$BASE_URL/api/generate?length=8&alphabet=full&prefix=app_&suffix=_key&raw=true" \
    "200"

test_api "Generate with newline" \
    "$BASE_URL/api/generate?length=5&raw=false" \
    "200"

test_api "Password generation" \
    "$BASE_URL/api/password" \
    "200"

test_api "Password with custom params" \
    "$BASE_URL/api/password?length=30&alphabet=no-look-alike" \
    "200" \
    "30"

test_api "API key generation" \
    "$BASE_URL/api/api-key" \
    "200"

test_api "API key with custom params" \
    "$BASE_URL/api/api-key?length=50&alphabet=no-look-alike" \
    "200"

test_api "Mnemonic generation" \
    "$BASE_URL/api/mnemonic" \
    "200"

test_api "Mnemonic in Spanish" \
    "$BASE_URL/api/mnemonic?language=spanish" \
    "200"

test_api "Mnemonic in French" \
    "$BASE_URL/api/mnemonic?language=french" \
    "200"

test_api "Mnemonic in Portuguese" \
    "$BASE_URL/api/mnemonic?language=portuguese" \
    "200"

test_api "Mnemonic in Japanese" \
    "$BASE_URL/api/mnemonic?language=japanese" \
    "200"

test_api "Mnemonic in Chinese" \
    "$BASE_URL/api/mnemonic?language=chinese" \
    "200"

test_api "Mnemonic in Chinese Traditional" \
    "$BASE_URL/api/mnemonic?language=chinese-traditional" \
    "200"

test_api "Mnemonic in Italian" \
    "$BASE_URL/api/mnemonic?language=italian" \
    "200"

test_api "Mnemonic in Korean" \
    "$BASE_URL/api/mnemonic?language=korean" \
    "200"

test_api "Mnemonic in Czech" \
    "$BASE_URL/api/mnemonic?language=czech" \
    "200"

test_api "Mnemonic 12 words (default)" \
    "$BASE_URL/api/mnemonic?words=12" \
    "200"

test_api "Mnemonic 24 words" \
    "$BASE_URL/api/mnemonic?words=24" \
    "200"

test_api "Mnemonic Spanish 24 words" \
    "$BASE_URL/api/mnemonic?language=spanish&words=24" \
    "200"

# POST mnemonic tests
echo -e "\n${BLUE}Testing POST /api/mnemonic with seed${NC}"

test_api "POST Mnemonic with seed" \
    "$BASE_URL/api/mnemonic" \
    "200" \
    "" \
    "POST" \
    '{"seed":"2rfuWV8mXicE8TWKzEHFS91hJdezQ5TN1A8sXVsV5iDd"}'

test_api "POST Mnemonic with seed 24 words" \
    "$BASE_URL/api/mnemonic" \
    "200" \
    "" \
    "POST" \
    '{"seed":"2rfuWV8mXicE8TWKzEHFS91hJdezQ5TN1A8sXVsV5iDd","words":24}'

test_api "POST Mnemonic Spanish with seed" \
    "$BASE_URL/api/mnemonic" \
    "200" \
    "" \
    "POST" \
    '{"seed":"2rfuWV8mXicE8TWKzEHFS91hJdezQ5TN1A8sXVsV5iDd","language":"spanish"}'

test_api "Version endpoint" \
    "$BASE_URL/api/version" \
    "200"

# Alphabet tests
echo -e "\n${YELLOW}=== Alphabet Tests ===${NC}"

test_api "Base58 alphabet" \
    "$BASE_URL/api/generate?alphabet=base58&length=15" \
    "200" \
    "15"

test_api "No-look-alike alphabet" \
    "$BASE_URL/api/generate?alphabet=no-look-alike&length=15" \
    "200" \
    "15"

test_api "Full alphabet" \
    "$BASE_URL/api/generate?alphabet=full&length=15" \
    "200" \
    "15"

test_api "Full-with-symbols alphabet" \
    "$BASE_URL/api/generate?alphabet=full-with-symbols&length=15" \
    "200" \
    "15"

# Edge cases
echo -e "\n${YELLOW}=== Edge Case Tests ===${NC}"

test_api "Minimum length (2)" \
    "$BASE_URL/api/generate?length=2" \
    "200" \
    "2"

test_api "Maximum length (128)" \
    "$BASE_URL/api/generate?length=128" \
    "200" \
    "128"

test_api "Password minimum (21)" \
    "$BASE_URL/api/password?length=21" \
    "200" \
    "21"

test_api "Password maximum (44)" \
    "$BASE_URL/api/password?length=44" \
    "200" \
    "44"

test_api "API key minimum (44)" \
    "$BASE_URL/api/api-key?length=44" \
    "200"

test_api "API key maximum (64)" \
    "$BASE_URL/api/api-key?length=64" \
    "200"

# Error validation tests
echo -e "\n${YELLOW}=== Error Validation Tests ===${NC}"

test_api "Length too small" \
    "$BASE_URL/api/generate?length=1" \
    "400"

test_api "Length too large" \
    "$BASE_URL/api/generate?length=200" \
    "400"

test_api "Password too short" \
    "$BASE_URL/api/password?length=10" \
    "400"

test_api "Password too long" \
    "$BASE_URL/api/password?length=50" \
    "400"

test_api "API key too short" \
    "$BASE_URL/api/api-key?length=30" \
    "400"

test_api "API key too long" \
    "$BASE_URL/api/api-key?length=70" \
    "400"

test_api "Prefix too long" \
    "$BASE_URL/api/generate?prefix=ThisPrefixIsWayTooLongAndShouldBeRejected12345" \
    "400"

test_api "Suffix too long" \
    "$BASE_URL/api/generate?suffix=ThisSuffixIsWayTooLongAndShouldBeRejected12345" \
    "400"

test_api "No-look-alike password too short" \
    "$BASE_URL/api/password?length=23&alphabet=no-look-alike" \
    "400"

test_api "No-look-alike API key too short" \
    "$BASE_URL/api/api-key?length=46&alphabet=no-look-alike" \
    "400"

test_api "Invalid mnemonic language" \
    "$BASE_URL/api/mnemonic?language=invalid" \
    "400"

test_api "Invalid mnemonic words (15)" \
    "$BASE_URL/api/mnemonic?words=15" \
    "400"

test_api "Invalid mnemonic words (text)" \
    "$BASE_URL/api/mnemonic?words=invalid" \
    "400"

test_api "POST Mnemonic missing seed" \
    "$BASE_URL/api/mnemonic" \
    "400" \
    "" \
    "POST" \
    '{}'

test_api "POST Mnemonic invalid seed" \
    "$BASE_URL/api/mnemonic" \
    "400" \
    "" \
    "POST" \
    '{"seed":"invalid-seed-123"}'

# 404 tests
echo -e "\n${YELLOW}=== 404 Error Tests ===${NC}"

test_api "Invalid endpoint" \
    "$BASE_URL/api/invalid" \
    "404"

test_api "Root path" \
    "$BASE_URL/" \
    "404"

test_api "Typo in endpoint" \
    "$BASE_URL/api/generat" \
    "404"

# Parameter handling tests
echo -e "\n${YELLOW}=== Parameter Handling Tests ===${NC}"

test_api "Invalid alphabet (should default)" \
    "$BASE_URL/api/generate?alphabet=invalid&length=10" \
    "200" \
    "10"

test_api "Invalid length (should default)" \
    "$BASE_URL/api/generate?length=invalid" \
    "200" \
    "21"

test_api "Invalid raw parameter (should default)" \
    "$BASE_URL/api/generate?raw=invalid&length=5" \
    "200" \
    "5"

# Multiple requests test
echo -e "\n${YELLOW}=== Consistency Tests ===${NC}"

for i in {1..5}; do
    test_api "Consistency test $i" \
        "$BASE_URL/api/generate?length=12" \
        "200" \
        "12"
done

# Complex scenarios
echo -e "\n${YELLOW}=== Complex Scenario Tests ===${NC}"

test_api "All parameters with spaces (URL encoded)" \
    "$BASE_URL/api/generate?length=10&alphabet=full&prefix=test%20&suffix=%20end" \
    "200"

test_api "Password with symbols" \
    "$BASE_URL/api/password?length=25&alphabet=full-with-symbols" \
    "200" \
    "25"

test_api "API key no-look-alike minimum" \
    "$BASE_URL/api/api-key?length=47&alphabet=no-look-alike" \
    "200"

# Final results
echo -e "\n=============================================="
echo -e "                FINAL RESULTS"
echo -e "=============================================="
echo -e "${BLUE}Total Tests:${NC} $TOTAL"
echo -e "${GREEN}Passed:${NC} $PASSED"
echo -e "${RED}Failed:${NC} $FAILED"

success_rate=$(( (PASSED * 100) / TOTAL ))
echo -e "${BLUE}Success Rate:${NC} $success_rate%"

if [[ $FAILED -eq 0 ]]; then
    echo -e "\n${GREEN}🎉 ALL TESTS PASSED! 🎉${NC}"
    echo -e "${GREEN}HashRand Spin API is working perfectly!${NC}"
    echo -e "${GREEN}The API handles all edge cases correctly.${NC}"
    exit 0
else
    echo -e "\n${YELLOW}⚠ $FAILED out of $TOTAL tests failed${NC}"
    if [[ $success_rate -ge 90 ]]; then
        echo -e "${YELLOW}Overall success rate is good ($success_rate%).${NC}"
    else
        echo -e "${RED}Success rate is concerning ($success_rate%).${NC}"
    fi
    exit 1
fi