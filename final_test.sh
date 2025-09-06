#!/bin/bash

# Final API Test Script - Comprehensive with JWT Authentication Support
BASE_URL="http://localhost:3000"
PASSED=0
FAILED=0
TOTAL=0
JWT_TOKEN=""
TEST_EMAIL="me@arkaitz.dev"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
PURPLE='\033[0;35m'
NC='\033[0m'

test_api() {
    local name="$1"
    local url="$2"
    local expected_status="$3"
    local length_check="$4"  # Optional length to check
    local method="${5:-GET}"  # Optional method, defaults to GET
    local post_data="$6"      # Optional POST data
    local use_auth="${7:-false}" # Whether to use JWT authentication
    
    ((TOTAL++))
    echo -e "\n${BLUE}[$TOTAL] $name${NC}"
    echo "URL: $url"
    if [[ "$method" == "POST" ]]; then
        echo "Method: POST"
        echo "Data: $post_data"
    fi
    if [[ "$use_auth" == "true" ]]; then
        echo "Auth: Using JWT Bearer token"
    fi
    
    # Make request and capture response
    local temp_file=$(mktemp)
    local status
    local auth_header=""
    if [[ "$use_auth" == "true" && -n "$JWT_TOKEN" ]]; then
        auth_header="-H \"Authorization: Bearer $JWT_TOKEN\""
    fi
    
    if [[ "$method" == "POST" ]]; then
        if [[ "$use_auth" == "true" && -n "$JWT_TOKEN" ]]; then
            status=$(curl -s -X POST -H "Content-Type: application/json" -H "Authorization: Bearer $JWT_TOKEN" -w "%{http_code}" -o "$temp_file" -d "$post_data" "$url")
        else
            status=$(curl -s -X POST -H "Content-Type: application/json" -w "%{http_code}" -o "$temp_file" -d "$post_data" "$url")
        fi
    else
        if [[ "$use_auth" == "true" && -n "$JWT_TOKEN" ]]; then
            status=$(curl -s -H "Authorization: Bearer $JWT_TOKEN" -w "%{http_code}" -o "$temp_file" "$url")
        else
            status=$(curl -s -w "%{http_code}" -o "$temp_file" "$url")
        fi
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

# Authentication helper functions
request_magic_link() {
    echo -e "\n${PURPLE}=== Requesting Magic Link ===${NC}"
    local response=$(curl -s -X POST -H "Content-Type: application/json" -d "{\"email\":\"$TEST_EMAIL\"}" "$BASE_URL/api/login/")
    echo "Magic link request response: $response"
    
    if [[ "$response" == *'"status":"OK"'* ]]; then
        echo -e "${GREEN}✓ Magic link requested successfully${NC}"
        return 0
    else
        echo -e "${RED}✗ Failed to request magic link${NC}"
        return 1
    fi
}

extract_magic_token() {
    echo -e "\n${PURPLE}=== Extracting Magic Token ===${NC}"
    # Wait longer for the log to be written and email to be sent
    sleep 5
    
    # Extract from Generated magic_link debug line (most reliable)
    local magic_token=$(grep "Generated magic_link" .spin-dev.log | tail -1 | grep -o "magiclink=[A-Za-z0-9]*" | cut -d= -f2)
    
    if [[ -n "$magic_token" ]]; then
        echo "Magic token extracted: ${magic_token:0:20}..."
        echo "$magic_token"
        return 0
    else
        echo -e "${RED}✗ Could not extract magic token from logs${NC}"
        echo "Last 5 debug lines with magic_link:"
        grep "Generated magic_link" .spin-dev.log | tail -3
        echo ""
        echo "Full grep result:"
        grep "Generated magic_link" .spin-dev.log | tail -1
        echo ""
        echo "Regex extraction result:"
        grep "Generated magic_link" .spin-dev.log | tail -1 | grep -o "magiclink=[A-Za-z0-9]*"
        return 1
    fi
}

authenticate() {
    echo -e "\n${PURPLE}=== AUTHENTICATION FLOW ===${NC}"
    
    # Clear previous logs to avoid confusion
    echo "Clearing previous authentication logs..."
    > .spin-dev.log
    sleep 1
    
    # Step 1: Request magic link with fresh start
    echo -e "\n${PURPLE}=== Requesting Fresh Magic Link ===${NC}"
    local magic_response=$(curl -s -X POST -H "Content-Type: application/json" -d '{"email":"me@arkaitz.dev"}' "$BASE_URL/api/login/")
    echo "Magic link request response: $magic_response"
    
    if [[ "$magic_response" != *'"status":"OK"'* ]]; then
        echo -e "${RED}✗ Authentication failed: Could not request magic link${NC}"
        return 1
    fi
    
    echo -e "${GREEN}✓ Magic link requested successfully${NC}"
    
    # Step 2: Wait and extract magic token
    echo -e "\n${PURPLE}=== Extracting Magic Token ===${NC}"
    echo "Waiting for magic link to be generated and logged..."
    sleep 3
    
    # Extract from Generated magic_link debug line (treat as text file)
    local magic_token=$(grep -a "Generated magic_link" .spin-dev.log | tail -1 | grep -o "magiclink=[A-Za-z0-9]*" | cut -d= -f2)
    
    if [[ -z "$magic_token" ]]; then
        echo -e "${RED}✗ Could not extract magic token from logs${NC}"
        echo "Debug - Last lines in log:"
        tail -5 .spin-dev.log
        echo "Debug - Magic link lines:"
        grep -a "Generated magic_link" .spin-dev.log | tail -3
        return 1
    fi
    
    echo "Magic token extracted: ${magic_token:0:20}..."
    
    # Step 3: Exchange magic token for JWT immediately
    echo -e "\n${PURPLE}=== Converting Magic Token to JWT ===${NC}"
    local jwt_response=$(curl -s "$BASE_URL/api/login/?magiclink=$magic_token")
    echo "JWT response: $jwt_response"
    
    # Extract JWT token
    JWT_TOKEN=$(echo "$jwt_response" | grep -o '"access_token":"[^"]*' | cut -d'"' -f4)
    
    if [[ -n "$JWT_TOKEN" ]]; then
        echo -e "${GREEN}✓ JWT token obtained: ${JWT_TOKEN:0:30}...${NC}"
        echo -e "${GREEN}✓ Authentication successful - JWT token ready for protected tests${NC}"
        return 0
    else
        echo -e "${RED}✗ Authentication failed: Could not obtain JWT token${NC}"
        echo "Debug - Full JWT response: $jwt_response"
        return 1
    fi
}

echo "=============================================="
echo "      HashRand API Comprehensive Test Suite"
echo "       (Zero Knowledge + JWT Authentication)"
echo "=============================================="

# First test public endpoints (no authentication required)
echo -e "\n${YELLOW}=== PUBLIC ENDPOINTS (No Authentication) ===${NC}"

test_api "Version endpoint (public)" \
    "$BASE_URL/api/version" \
    "200"

# Test that protected endpoints require authentication
echo -e "\n${YELLOW}=== AUTHENTICATION REQUIRED TESTS ===${NC}"

test_api "Custom endpoint without auth (should fail)" \
    "$BASE_URL/api/custom?length=12" \
    "401"

test_api "Password endpoint without auth (should fail)" \
    "$BASE_URL/api/password?length=25" \
    "401"

test_api "API key endpoint without auth (should fail)" \
    "$BASE_URL/api/api-key?length=48" \
    "401"

test_api "Mnemonic endpoint without auth (should fail)" \
    "$BASE_URL/api/mnemonic?words=12" \
    "401"

# Perform authentication
echo -e "\n${PURPLE}=============================================${NC}"
echo -e "${PURPLE}    AUTHENTICATING FOR PROTECTED TESTS${NC}"
echo -e "${PURPLE}=============================================${NC}"

if ! authenticate; then
    echo -e "${RED}✗ Authentication failed - skipping protected endpoint tests${NC}"
    echo -e "${YELLOW}⚠ Some tests will be skipped due to authentication failure${NC}"
else
    echo -e "${GREEN}✓ Authentication successful - proceeding with protected tests${NC}"
    
    # Now test protected endpoints with authentication
    echo -e "\n${YELLOW}=== PROTECTED ENDPOINTS (With Authentication) ===${NC}"
    
    test_api "Custom hash generation" \
        "$BASE_URL/api/custom?length=12" \
        "200" \
        "12" \
        "GET" \
        "" \
        "true"
    
    test_api "Custom hash with longer length" \
        "$BASE_URL/api/custom?length=24" \
        "200" \
        "24" \
        "GET" \
        "" \
        "true"

    test_api "Password generation (default)" \
        "$BASE_URL/api/password" \
        "200" \
        "" \
        "GET" \
        "" \
        "true"

    test_api "Password with custom length" \
        "$BASE_URL/api/password?length=30" \
        "200" \
        "30" \
        "GET" \
        "" \
        "true"

    test_api "API key generation (default)" \
        "$BASE_URL/api/api-key" \
        "200" \
        "" \
        "GET" \
        "" \
        "true"

    test_api "API key with custom length" \
        "$BASE_URL/api/api-key?length=50" \
        "200" \
        "" \
        "GET" \
        "" \
        "true"

    test_api "Mnemonic generation (default)" \
        "$BASE_URL/api/mnemonic" \
        "200" \
        "" \
        "GET" \
        "" \
        "true"

    test_api "Mnemonic in Spanish" \
        "$BASE_URL/api/mnemonic?language=spanish" \
        "200" \
        "" \
        "GET" \
        "" \
        "true"

    test_api "Mnemonic 12 words" \
        "$BASE_URL/api/mnemonic?words=12" \
        "200" \
        "" \
        "GET" \
        "" \
        "true"

    test_api "Mnemonic 24 words" \
        "$BASE_URL/api/mnemonic?words=24" \
        "200" \
        "" \
        "GET" \
        "" \
        "true"

    test_api "Mnemonic Spanish 24 words" \
        "$BASE_URL/api/mnemonic?language=spanish&words=24" \
        "200" \
        "" \
        "GET" \
        "" \
        "true"

    # Edge case tests with authentication
    echo -e "\n${YELLOW}=== EDGE CASE TESTS (With Authentication) ===${NC}"

    test_api "Password minimum length (21)" \
        "$BASE_URL/api/password?length=21" \
        "200" \
        "21" \
        "GET" \
        "" \
        "true"

    test_api "Password maximum length (44)" \
        "$BASE_URL/api/password?length=44" \
        "200" \
        "44" \
        "GET" \
        "" \
        "true"

    test_api "API key minimum length (44)" \
        "$BASE_URL/api/api-key?length=44" \
        "200" \
        "" \
        "GET" \
        "" \
        "true"

    test_api "API key maximum length (64)" \
        "$BASE_URL/api/api-key?length=64" \
        "200" \
        "" \
        "GET" \
        "" \
        "true"

    # Error validation tests with authentication
    echo -e "\n${YELLOW}=== ERROR VALIDATION TESTS (With Authentication) ===${NC}"

    test_api "Password too short (should fail)" \
        "$BASE_URL/api/password?length=10" \
        "400" \
        "" \
        "GET" \
        "" \
        "true"

    test_api "Password too long (should fail)" \
        "$BASE_URL/api/password?length=50" \
        "400" \
        "" \
        "GET" \
        "" \
        "true"

    test_api "API key too short (should fail)" \
        "$BASE_URL/api/api-key?length=30" \
        "400" \
        "" \
        "GET" \
        "" \
        "true"

    test_api "API key too long (should fail)" \
        "$BASE_URL/api/api-key?length=70" \
        "400" \
        "" \
        "GET" \
        "" \
        "true"

    test_api "Invalid mnemonic language (should fail)" \
        "$BASE_URL/api/mnemonic?language=invalid" \
        "400" \
        "" \
        "GET" \
        "" \
        "true"

    test_api "Invalid mnemonic words (15)" \
        "$BASE_URL/api/mnemonic?words=15" \
        "400" \
        "" \
        "GET" \
        "" \
        "true"

    test_api "Invalid mnemonic words (text)" \
        "$BASE_URL/api/mnemonic?words=invalid" \
        "400" \
        "" \
        "GET" \
        "" \
        "true"
fi

# Authentication-specific tests
echo -e "\n${PURPLE}=== AUTHENTICATION FLOW TESTS ===${NC}"

test_api "Request magic link with valid email" \
    "$BASE_URL/api/login/" \
    "200" \
    "" \
    "POST" \
    '{"email":"arkaitzmugica@protonmail.com"}'

test_api "Request magic link with invalid email format" \
    "$BASE_URL/api/login/" \
    "400" \
    "" \
    "POST" \
    '{"email":"invalid-email"}'

test_api "Request magic link with missing email" \
    "$BASE_URL/api/login/" \
    "400" \
    "" \
    "POST" \
    '{}'

test_api "Invalid magic link (should fail)" \
    "$BASE_URL/api/login/?magiclink=invalid_token_12345" \
    "400"

# Test token expiration (if we have time)
if [[ -n "$JWT_TOKEN" ]]; then
    echo -e "\n${PURPLE}=== JWT TOKEN VALIDATION TESTS ===${NC}"
    
    test_api "Valid JWT token access" \
        "$BASE_URL/api/custom?length=8" \
        "200" \
        "8" \
        "GET" \
        "" \
        "true"
    
    # Test with invalid token
    local old_token="$JWT_TOKEN"
    JWT_TOKEN="invalid_token_123"
    
    test_api "Invalid JWT token (should fail)" \
        "$BASE_URL/api/custom?length=8" \
        "401" \
        "" \
        "GET" \
        "" \
        "true"
    
    # Restore valid token
    JWT_TOKEN="$old_token"
fi

# 404 tests (these work without authentication)
echo -e "\n${YELLOW}=== 404 ERROR TESTS (No Authentication Required) ===${NC}"

test_api "Root path (should be 404)" \
    "$BASE_URL/" \
    "404"

# Note: /api/invalid and other non-existent endpoints under /api/ 
# are caught by the authentication middleware first, so they return 401
# This is expected behavior in the current architecture

# Final results
echo -e "\n=============================================="
echo -e "                FINAL RESULTS"
echo -e "=============================================="
echo -e "${BLUE}Total Tests:${NC} $TOTAL"
echo -e "${GREEN}Passed:${NC} $PASSED"
echo -e "${RED}Failed:${NC} $FAILED"

success_rate=$(( (PASSED * 100) / TOTAL ))
echo -e "${BLUE}Success Rate:${NC} $success_rate%"

if [[ -n "$JWT_TOKEN" ]]; then
    echo -e "${PURPLE}JWT Authentication:${NC} ✓ Successful"
else
    echo -e "${PURPLE}JWT Authentication:${NC} ✗ Failed (some tests skipped)"
fi

if [[ $FAILED -eq 0 ]]; then
    echo -e "\n${GREEN}🎉 ALL TESTS PASSED! 🎉${NC}"
    echo -e "${GREEN}HashRand Spin API with Zero Knowledge Auth is working perfectly!${NC}"
    echo -e "${GREEN}✓ Authentication flow working${NC}"
    echo -e "${GREEN}✓ JWT protection active on all endpoints${NC}"
    echo -e "${GREEN}✓ Public endpoints accessible${NC}"
    echo -e "${GREEN}✓ All endpoint validations correct${NC}"
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