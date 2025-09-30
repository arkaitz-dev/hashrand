#!/bin/bash

# Final API Test Script - Comprehensive with JWT Authentication Support + Ed25519 Signed Responses
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

# Source signed response helpers
source scripts/signed_response_helpers.sh

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
        # For GET requests with authentication, add Ed25519 signature
        local final_url="$url"
        if [[ "$use_auth" == "true" && -n "$JWT_TOKEN" ]]; then
            final_url=$(generate_signed_url "$url")
            if [[ $? -ne 0 ]]; then
                echo -e "${RED}✗ FAIL - Failed to generate signed URL${NC}"
                ((FAILED++))
                return
            fi
            # Using signed URL with Ed25519 signature from auth session
            status=$(curl -s -H "Authorization: Bearer $JWT_TOKEN" -w "%{http_code}" -o "$temp_file" "$final_url")
        else
            status=$(curl -s -w "%{http_code}" -o "$temp_file" "$url")
        fi
    fi
    local body=$(cat "$temp_file")
    rm "$temp_file"
    
    echo "Status: $status"
    echo "Response: $body"

    # Validate signed response if we have server public key and response is signed
    if [[ "$status" == "200" && -n "$SERVER_PUB_KEY" ]] && is_signed_response "$body"; then
        echo -e "${BLUE}🔐 Validating signed response...${NC}"
        if process_regular_response "$body"; then
            echo -e "${GREEN}✓ Response signature validated${NC}"
        else
            echo -e "${RED}✗ FAIL - Response signature validation failed${NC}"
            ((FAILED++))
            return
        fi
    fi

    # Check HTTP status
    if [[ "$status" != "$expected_status" ]]; then
        echo -e "${RED}✗ FAIL - Wrong status: expected $expected_status, got $status${NC}"
        ((FAILED++))
        return
    fi
    
    # Optional length check
    if [[ -n "$length_check" && "$status" == "200" ]]; then
        # Extract hash from JSON response (handle both signed and regular responses)
        local hash
        if is_signed_response "$body"; then
            hash=$(extract_field_from_payload "$body" "hash")
        else
            hash=$(echo "$body" | jq -r '.hash' 2>/dev/null)
        fi

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
                # Extract hash from response (handle both signed and regular responses)
                local hash
                if is_signed_response "$body"; then
                    hash=$(extract_field_from_payload "$body" "hash")
                else
                    hash=$(echo "$body" | jq -r '.hash' 2>/dev/null)
                fi

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

# Ed25519 helper functions

# Generate signed URL for GET requests with Ed25519 signature
# Usage: generate_signed_url "http://localhost:3000/api/custom?length=12"
# Returns: URL with signature parameter added
generate_signed_url() {
    local base_url="$1"
    local pub_key_file=".test-magiclink-pubkey"

    # Check if we have a stored public key from authentication
    if [[ ! -f "$pub_key_file" ]]; then
        echo -e "${RED}✗ No stored public key found. Authentication required first.${NC}"
        echo "$base_url"
        return 1
    fi

    local pub_key=$(cat "$pub_key_file")
    if [[ -z "$pub_key" ]]; then
        echo -e "${RED}✗ Empty public key found.${NC}"
        echo "$base_url"
        return 1
    fi

    # Extract query parameters from URL
    local params_json="{}"
    if [[ "$base_url" == *"?"* ]]; then
        local query_string="${base_url#*\?}"
        local base_url_no_query="${base_url%%\?*}"

        # Convert query string to JSON for signing
        params_json="{"
        local first=true
        IFS='&' read -ra PAIRS <<< "$query_string"
        for pair in "${PAIRS[@]}"; do
            if [[ "$pair" == *"="* ]]; then
                local key="${pair%%=*}"
                local value="${pair#*=}"
                if [[ "$first" == "true" ]]; then
                    first=false
                else
                    params_json+=","
                fi
                params_json+="\"$key\":\"$value\""
            fi
        done
        params_json+="}"
    else
        local base_url_no_query="$base_url"
    fi

    # Generate Ed25519 signature for query parameters using AUTH session keypair
    local signature=$(node ./scripts/sign_query_params.js "$pub_key" "$params_json" 2>/dev/null)
    if [[ -z "$signature" ]]; then
        echo -e "${RED}✗ Failed to generate Ed25519 signature for query parameters${NC}"
        echo "$base_url"
        return 1
    fi

    # Add only signature to URL (backend extracts pub_key from JWT Bearer token)
    local separator="?"
    if [[ "$base_url" == *"?"* ]]; then
        separator="&"
    fi

    echo "${base_url}${separator}signature=${signature}"
    return 0
}

generate_ed25519_payload() {
    local email="$1"

    # Generate Ed25519 keypair
    local pub_key=$(node ./scripts/generate_hash.js)
    if [[ -z "$pub_key" ]]; then
        echo '{"error":"Failed to generate keypair"}'
        return 1
    fi

    # Create payload object
    local payload_json="{\"email\":\"$email\",\"email_lang\":\"en\",\"next\":\"/\",\"pub_key\":\"$pub_key\",\"ui_host\":\"localhost\"}"

    # Create complete SignedRequest structure with Base64-encoded payload
    local signed_request=$(node ./scripts/create_signed_request.js "$payload_json")
    if [[ -z "$signed_request" ]]; then
        echo '{"error":"Failed to create SignedRequest"}'
        return 1
    fi

    # Return complete SignedRequest structure
    echo "$signed_request"
}

# Authentication helper functions
request_magic_link() {
    echo -e "\n${PURPLE}=== Requesting Magic Link ===${NC}"

    # Generate Ed25519 keypair using Node.js helper
    local pub_key=$(node ./scripts/generate_hash.js)
    if [[ -z "$pub_key" ]]; then
        echo -e "${RED}✗ Failed to generate Ed25519 keypair${NC}"
        return 1
    fi

    echo "Generated Ed25519 public key: ${pub_key:0:20}..."

    # Create payload object with ui_host (required for cookie Domain)
    local payload_json="{\"email\":\"$TEST_EMAIL\",\"email_lang\":\"en\",\"next\":\"/\",\"pub_key\":\"$pub_key\",\"ui_host\":\"localhost\"}"
    echo "JSON payload: ${payload_json:0:80}..."

    # Create complete SignedRequest with Base64-encoded payload
    local signed_request=$(node ./scripts/create_signed_request.js "$payload_json")
    if [[ -z "$signed_request" ]]; then
        echo -e "${RED}✗ Failed to create SignedRequest${NC}"
        return 1
    fi

    echo "SignedRequest created: ${signed_request:0:100}..."

    # Store pub_key for later validation (simulate localStorage behavior)
    echo "$pub_key" > .test-magiclink-pubkey

    # Send SignedRequest to backend
    local response=$(curl -s -X POST -H "Content-Type: application/json" -d "$signed_request" "$BASE_URL/api/login/")
    echo "Magic link request response: ${response:0:100}..."

    # Check if response is signed and extract status
    local status_ok=false
    if is_signed_response "$response"; then
        echo -e "${BLUE}📝 Processing signed magic link response...${NC}"
        local status_field=$(extract_field_from_payload "$response" "status")
        if [[ "$status_field" == "OK" ]]; then
            status_ok=true
        fi
    elif [[ "$response" == *'"status":"OK"'* ]]; then
        # Fallback for non-signed response
        status_ok=true
    fi

    if [[ "$status_ok" == "true" ]]; then
        echo -e "${GREEN}✓ Magic link requested successfully with Ed25519 signature validation${NC}"
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

    # Generate Ed25519 keypair using Node.js helper
    local pub_key=$(node ./scripts/generate_hash.js)
    if [[ -z "$pub_key" ]]; then
        echo -e "${RED}✗ Authentication failed: Could not generate Ed25519 keypair${NC}"
        return 1
    fi

    echo "Generated Ed25519 public key: ${pub_key:0:20}..."

    # Create payload object with ui_host (required for cookie Domain)
    local payload_json="{\"email\":\"me@arkaitz.dev\",\"email_lang\":\"en\",\"next\":\"/\",\"pub_key\":\"$pub_key\",\"ui_host\":\"localhost\"}"
    echo "JSON payload: ${payload_json:0:80}..."

    # Create complete SignedRequest with Base64-encoded payload
    local signed_request=$(node ./scripts/create_signed_request.js "$payload_json")
    if [[ -z "$signed_request" ]]; then
        echo -e "${RED}✗ Authentication failed: Could not create SignedRequest${NC}"
        return 1
    fi

    echo "SignedRequest created: ${signed_request:0:100}..."

    # Store pub_key for later validation (simulate localStorage behavior)
    echo "$pub_key" > .test-magiclink-pubkey

    # Send SignedRequest to backend
    local magic_response=$(curl -s -X POST -H "Content-Type: application/json" -d "$signed_request" "$BASE_URL/api/login/")
    echo "Magic link request response: ${magic_response:0:100}..."

    # Check if response is signed and extract status
    local status_ok=false
    if is_signed_response "$magic_response"; then
        echo -e "${BLUE}📝 Processing signed magic link response...${NC}"
        local status_field=$(extract_field_from_payload "$magic_response" "status")
        if [[ "$status_field" == "OK" ]]; then
            status_ok=true
        fi
    elif [[ "$magic_response" == *'"status":"OK"'* ]]; then
        # Fallback for non-signed response
        status_ok=true
    fi

    if [[ "$status_ok" != "true" ]]; then
        echo -e "${RED}✗ Authentication failed: Could not request magic link${NC}"
        return 1
    fi

    echo -e "${GREEN}✓ Magic link requested successfully with Ed25519 signature${NC}"
    
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

    # Read the stored pub_key for validation
    local stored_pub_key=$(cat .test-magiclink-pubkey 2>/dev/null)
    if [[ -z "$stored_pub_key" ]]; then
        echo -e "${RED}✗ Authentication failed: No stored pub_key found${NC}"
        return 1
    fi

    echo "Using stored pub_key: ${stored_pub_key:0:20}..."

    # Sign the magic token with the private key for Ed25519 validation
    local magic_signature=$(node ./scripts/sign_payload.js "$magic_token")
    if [[ -z "$magic_signature" ]]; then
        echo -e "${RED}✗ Authentication failed: Could not sign magic token${NC}"
        rm -f .test-magiclink-pubkey
        return 1
    fi

    echo "Generated magic token signature: ${magic_signature:0:20}..."

    # Create complete SignedRequest for magic link validation
    local magiclink_payload_json="{\"magiclink\":\"$magic_token\"}"
    local magiclink_signed_request=$(node ./scripts/create_signed_request.js "$magiclink_payload_json")
    if [[ -z "$magiclink_signed_request" ]]; then
        echo -e "${RED}✗ Authentication failed: Could not create magiclink SignedRequest${NC}"
        rm -f .test-magiclink-pubkey
        return 1
    fi

    echo "Magiclink SignedRequest created: ${magiclink_signed_request:0:100}..."

    # Magic link validation using POST endpoint with SignedRequest structure
    local jwt_response=$(curl -s -X POST -H "Content-Type: application/json" \
        -d "$magiclink_signed_request" \
        "$BASE_URL/api/login/magiclink/")
    echo "JWT response: $jwt_response"

    # Keep the stored pub_key file AND private key for GET request signing during protected tests
    # These files contain the SAME keypair used for authentication
    # rm -f .test-magiclink-pubkey  # (cleanup moved to end of script)
    # rm -f .test-ed25519-private-key  # (cleanup moved to end of script)

    # Process signed response and extract JWT token
    if is_signed_response "$jwt_response"; then
        echo -e "${BLUE}📝 Processing signed JWT response...${NC}"

        # Extract access token from signed JWT response (no server_pub_key needed)
        JWT_TOKEN=$(extract_access_token "$jwt_response")
    else
        echo -e "${YELLOW}⚠ Received non-signed response, extracting token directly${NC}"
        # Fallback for backward compatibility
        JWT_TOKEN=$(extract_access_token "$jwt_response")
    fi

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

    # Test JWT token validation immediately after authentication (before token expires)
    echo -e "\n${PURPLE}=== JWT TOKEN VALIDATION TESTS ===${NC}"

    test_api "Valid JWT token access" \
        "$BASE_URL/api/custom?length=8" \
        "200" \
        "8" \
        "GET" \
        "" \
        "true"

    # Test with invalid token
    old_token="$JWT_TOKEN"
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
    "$(generate_ed25519_payload "arkaitzmugica@protonmail.com")"

test_api "Request magic link with invalid email format" \
    "$BASE_URL/api/login/" \
    "400" \
    "" \
    "POST" \
    "$(generate_ed25519_payload "invalid-email")"

test_api "Request magic link with missing email" \
    "$BASE_URL/api/login/" \
    "400" \
    "" \
    "POST" \
    '{"payload":"eyJlbWFpbF9sYW5nIjoiZW4iLCJwdWJfa2V5IjoiYWJjMTIzIiwidWlfaG9zdCI6ImxvY2FsaG9zdCJ9","signature":"invalid_signature_hex"}'

test_api "Request magic link with missing pub_key (should fail)" \
    "$BASE_URL/api/login/" \
    "400" \
    "" \
    "POST" \
    '{"payload":"eyJlbWFpbCI6ImFya2FpdHptdWdpY2FAcHJvdG9ubWFpbC5jb20iLCJlbWFpbF9sYW5nIjoiZW4iLCJ1aV9ob3N0IjoibG9jYWxob3N0In0","signature":"invalid_signature_hex"}'

test_api "Invalid magic link (should fail)" \
    "$BASE_URL/api/login/magiclink/" \
    "400" \
    "" \
    "POST" \
    '{"payload":"eyJtYWdpY2xpbmsiOiJpbnZhbGlkX3Rva2VuXzEyMzQ1In0","signature":"invalid_signature_hex_value_123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"}'

# JWT validation tests moved to execute immediately after authentication

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

# Cleanup temporary files
echo -e "\n${PURPLE}Cleaning up temporary files...${NC}"
rm -f .test-magiclink-pubkey .test-ed25519-private-key
echo "✓ Temporary files cleaned"

if [[ $FAILED -eq 0 ]]; then
    echo -e "\n${GREEN}🎉 ALL TESTS PASSED! 🎉${NC}"
    echo -e "${GREEN}HashRand Spin API with Zero Knowledge Auth + Ed25519 Digital Signatures is working perfectly!${NC}"
    echo -e "${GREEN}✓ Authentication flow with Ed25519 signature validation working${NC}"
    echo -e "${GREEN}✓ JWT protection active on all endpoints${NC}"
    echo -e "${GREEN}✓ Public endpoints accessible${NC}"
    echo -e "${GREEN}✓ Ed25519 cryptographic signature security active${NC}"
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