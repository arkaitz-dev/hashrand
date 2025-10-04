#!/bin/bash

# Shared Secret API Test Script - Ed25519 Signed Requests + JWT Auth
# Tests the complete Shared Secret flow with authentication

BASE_URL="http://localhost:3000"
PASSED=0
FAILED=0
TOTAL=0
JWT_TOKEN=""
TEST_EMAIL="me@arkaitz.dev"
SENDER_HASH=""
RECEIVER_HASH=""
REFERENCE_HASH=""
OTP_CODE=""
RESPONSE_PAYLOAD=""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
PURPLE='\033[0;35m'
NC='\033[0m'

# Source signed response helpers
source scripts/signed_response_helpers.sh

echo -e "${PURPLE}╔═══════════════════════════════════════════════════════╗${NC}"
echo -e "${PURPLE}║    Shared Secret API Test Suite (Ed25519 + JWT)     ║${NC}"
echo -e "${PURPLE}╚═══════════════════════════════════════════════════════╝${NC}"
echo ""

# Helper: Generate signed POST request
generate_signed_post_request() {
    local payload_json="$1"
    local signed_request=$(node ./scripts/create_signed_request.js "$payload_json" 2>/dev/null)

    if [[ -z "$signed_request" ]]; then
        echo ""
        return 1
    fi

    echo "$signed_request"
    return 0
}

# Helper: Generate signed URL for GET/DELETE requests
generate_signed_url() {
    local base_url="$1"
    local pub_key_file=".test-shared-secret-pubkey"

    # Check if we have a stored public key from authentication
    if [[ ! -f "$pub_key_file" ]]; then
        echo ""
        return 1
    fi

    local pub_key=$(cat "$pub_key_file")
    if [[ -z "$pub_key" ]]; then
        echo ""
        return 1
    fi

    # Extract query parameters from URL
    local params_json="{}"
    if [[ "$base_url" == *"?"* ]]; then
        local query_string="${base_url#*\?}"

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
    fi

    # Generate Ed25519 signature for query parameters
    local signature=$(node ./scripts/sign_query_params.js "$pub_key" "$params_json" 2>/dev/null)
    if [[ -z "$signature" ]]; then
        echo ""
        return 1
    fi

    # Add signature to URL
    local separator="?"
    if [[ "$base_url" == *"?"* ]]; then
        separator="&"
    fi

    echo "${base_url}${separator}signature=${signature}"
    return 0
}

# Authentication function
authenticate() {
    echo -e "\n${PURPLE}=== AUTHENTICATION FLOW ===${NC}"

    # Step 1: Request magic link
    echo "Requesting magic link..."

    # Generate Ed25519 keypair
    local pub_key=$(node ./scripts/generate_hash.js 2>/dev/null)
    if [[ -z "$pub_key" ]]; then
        echo -e "${RED}✗ Could not generate Ed25519 keypair${NC}"
        return 1
    fi

    echo "Generated Ed25519 public key: ${pub_key:0:20}..."

    # Create payload for magic link request
    local payload_json="{\"email\":\"$TEST_EMAIL\",\"email_lang\":\"en\",\"next\":\"/\",\"pub_key\":\"$pub_key\",\"ui_host\":\"localhost\"}"

    # Create signed request
    local signed_request=$(generate_signed_post_request "$payload_json")
    if [[ -z "$signed_request" ]]; then
        echo -e "${RED}✗ Could not create SignedRequest${NC}"
        return 1
    fi

    # Store pub_key for later use
    echo "$pub_key" > .test-shared-secret-pubkey

    # Send request
    local magic_response=$(curl -s -X POST -H "Content-Type: application/json" -d "$signed_request" "$BASE_URL/api/login/")

    # Check if response is OK
    local status_ok=false
    if is_signed_response "$magic_response"; then
        local status_field=$(extract_field_from_payload "$magic_response" "status")
        if [[ "$status_field" == "OK" ]]; then
            status_ok=true
        fi
    elif [[ "$magic_response" == *'"status":"OK"'* ]]; then
        status_ok=true
    fi

    if [[ "$status_ok" != "true" ]]; then
        echo -e "${RED}✗ Could not request magic link${NC}"
        rm -f .test-shared-secret-pubkey
        return 1
    fi

    echo -e "${GREEN}✓ Magic link requested successfully${NC}"

    # Step 2: Extract magic token from logs
    echo "Waiting for magic link to be generated..."
    sleep 3

    local magic_token=$(grep -a "Generated magic_link" .spin-dev.log 2>/dev/null | tail -1 | grep -o "magiclink=[A-Za-z0-9]*" | cut -d= -f2)

    if [[ -z "$magic_token" ]]; then
        echo -e "${RED}✗ Could not extract magic token from logs${NC}"
        rm -f .test-shared-secret-pubkey
        return 1
    fi

    echo "Magic token extracted: ${magic_token:0:20}..."

    # Step 3: Exchange magic token for JWT
    echo "Converting magic token to JWT..."

    # Create signed request for magic link validation
    local magiclink_payload_json="{\"magiclink\":\"$magic_token\"}"
    local magiclink_signed_request=$(generate_signed_post_request "$magiclink_payload_json")
    if [[ -z "$magiclink_signed_request" ]]; then
        echo -e "${RED}✗ Could not create magiclink SignedRequest${NC}"
        rm -f .test-shared-secret-pubkey
        return 1
    fi

    # Exchange for JWT
    local jwt_response=$(curl -s -X POST -H "Content-Type: application/json" \
        -d "$magiclink_signed_request" \
        "$BASE_URL/api/login/magiclink/")

    # Extract JWT token
    JWT_TOKEN=$(extract_access_token "$jwt_response")

    if [[ -n "$JWT_TOKEN" ]]; then
        echo -e "${GREEN}✓ JWT token obtained: ${JWT_TOKEN:0:30}...${NC}"
        echo -e "${GREEN}✓ Authentication successful${NC}"
        return 0
    else
        echo -e "${RED}✗ Could not obtain JWT token${NC}"
        rm -f .test-shared-secret-pubkey
        return 1
    fi
}

# Function to test API endpoint
test_endpoint() {
    local name="$1"
    local method="$2"
    local url="$3"
    local expected_status="$4"
    local post_data="$5"

    ((TOTAL++))
    echo -e "\n${BLUE}[$TOTAL] $name${NC}"
    echo "Method: $method"
    echo "URL: $url"
    [[ -n "$post_data" ]] && echo "Data: ${post_data:0:100}..."

    local temp_file=$(mktemp)
    local status

    if [[ "$method" == "POST" ]]; then
        # Create signed POST request
        local signed_request=$(generate_signed_post_request "$post_data")
        if [[ $? -ne 0 ]]; then
            echo -e "${RED}✗ FAIL - Failed to generate signed request${NC}"
            ((FAILED++))
            rm "$temp_file"
            return 1
        fi

        status=$(curl -s -X POST \
            -H "Content-Type: application/json" \
            -H "Authorization: Bearer $JWT_TOKEN" \
            -w "%{http_code}" \
            -o "$temp_file" \
            -d "$signed_request" \
            "$url")
    elif [[ "$method" == "DELETE" ]]; then
        # DELETE with Ed25519 signature
        local signed_url=$(generate_signed_url "$url")
        if [[ $? -ne 0 ]]; then
            echo -e "${RED}✗ FAIL - Failed to generate signed URL${NC}"
            ((FAILED++))
            rm "$temp_file"
            return 1
        fi

        status=$(curl -s -X DELETE \
            -H "Authorization: Bearer $JWT_TOKEN" \
            -w "%{http_code}" \
            -o "$temp_file" \
            "$signed_url")
    else
        # GET with Ed25519 signature
        local signed_url=$(generate_signed_url "$url")
        if [[ $? -ne 0 ]]; then
            echo -e "${RED}✗ FAIL - Failed to generate signed URL${NC}"
            ((FAILED++))
            rm "$temp_file"
            return 1
        fi

        status=$(curl -s \
            -H "Authorization: Bearer $JWT_TOKEN" \
            -w "%{http_code}" \
            -o "$temp_file" \
            "$signed_url")
    fi

    local body=$(cat "$temp_file")
    rm "$temp_file"

    echo "Status: $status"
    echo "Response: ${body:0:200}..."

    # Extract payload from signed response
    if is_signed_response "$body"; then
        echo -e "${BLUE}🔐 Validating signed response...${NC}"

        if ! process_regular_response "$body"; then
            echo -e "${RED}✗ FAIL - Response signature validation failed${NC}"
            ((FAILED++))
            return 1
        fi
        echo -e "${GREEN}✓ Response signature validated${NC}"

        # Extract and decode Base64 payload
        local payload_b64=$(echo "$body" | jq -r '.payload' 2>/dev/null)
        if [[ -n "$payload_b64" && "$payload_b64" != "null" ]]; then
            RESPONSE_PAYLOAD=$(echo "$payload_b64" | base64 -d 2>/dev/null)
        else
            RESPONSE_PAYLOAD=""
        fi
    else
        # Non-signed response
        RESPONSE_PAYLOAD="$body"
    fi

    # Check status
    if [[ "$status" != "$expected_status" ]]; then
        echo -e "${RED}✗ FAIL - Wrong status: expected $expected_status, got $status${NC}"
        ((FAILED++))
        return 1
    fi

    echo -e "${GREEN}✓ PASS${NC}"
    ((PASSED++))
    return 0
}

# 1. Authenticate first
echo -e "\n${YELLOW}=== STEP 1: Authentication ===${NC}"
echo "Authenticating as $TEST_EMAIL..."

if ! authenticate; then
    echo -e "${RED}✗ FAIL - Authentication failed${NC}"
    echo -e "\n${YELLOW}Summary: $PASSED passed, $FAILED failed, $TOTAL total${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Authenticated successfully${NC}"
echo "JWT Token: ${JWT_TOKEN:0:30}..."

# 2. Create Shared Secret (without OTP)
echo -e "\n${YELLOW}=== STEP 2: Create Shared Secret (No OTP) ===${NC}"

CREATE_PAYLOAD='{
  "receiver_email": "arkaitzmugica@protonmail.com",
  "secret_text": "This is a test secret message from bash script",
  "expires_hours": 24,
  "max_reads": 3,
  "require_otp": false,
  "send_copy_to_sender": false
}'

if test_endpoint \
    "Create shared secret" \
    "POST" \
    "$BASE_URL/api/shared-secret/create" \
    "200" \
    "$CREATE_PAYLOAD"; then

    # Extract url_sender, url_receiver, reference from response
    SENDER_HASH=$(echo "$RESPONSE_PAYLOAD" | jq -r '.url_sender' | sed 's|.*/||')
    RECEIVER_HASH=$(echo "$RESPONSE_PAYLOAD" | jq -r '.url_receiver' | sed 's|.*/||')
    REFERENCE_HASH=$(echo "$RESPONSE_PAYLOAD" | jq -r '.reference')

    echo -e "${GREEN}✓ Sender hash: $SENDER_HASH${NC}"
    echo -e "${GREEN}✓ Receiver hash: $RECEIVER_HASH${NC}"
    echo -e "${GREEN}✓ Reference: $REFERENCE_HASH${NC}"
fi

# 3. View Secret as Sender (unlimited reads)
echo -e "\n${YELLOW}=== STEP 3: View Secret as Sender ===${NC}"

test_endpoint \
    "View secret (sender)" \
    "GET" \
    "$BASE_URL/api/shared-secret/$SENDER_HASH" \
    "200"

# Verify role is 'sender' and pending_reads is -1
if [[ -n "$RESPONSE_PAYLOAD" ]]; then
    ROLE=$(echo "$RESPONSE_PAYLOAD" | jq -r '.role')
    PENDING_READS=$(echo "$RESPONSE_PAYLOAD" | jq -r '.pending_reads')

    if [[ "$ROLE" == "sender" && "$PENDING_READS" == "-1" ]]; then
        echo -e "${GREEN}✓ Correct role (sender) and unlimited reads (-1)${NC}"
    else
        echo -e "${RED}✗ Wrong role or pending_reads: role=$ROLE, pending_reads=$PENDING_READS${NC}"
        ((FAILED++))
    fi
fi

# 4. View Secret as Receiver (limited reads)
echo -e "\n${YELLOW}=== STEP 4: View Secret as Receiver ===${NC}"

test_endpoint \
    "View secret (receiver)" \
    "GET" \
    "$BASE_URL/api/shared-secret/$RECEIVER_HASH" \
    "200"

# Verify role is 'receiver' and pending_reads decremented
if [[ -n "$RESPONSE_PAYLOAD" ]]; then
    ROLE=$(echo "$RESPONSE_PAYLOAD" | jq -r '.role')
    PENDING_READS=$(echo "$RESPONSE_PAYLOAD" | jq -r '.pending_reads')

    if [[ "$ROLE" == "receiver" && "$PENDING_READS" == "2" ]]; then
        echo -e "${GREEN}✓ Correct role (receiver) and reads decremented (2/3)${NC}"
    else
        echo -e "${RED}✗ Wrong role or pending_reads: role=$ROLE, pending_reads=$PENDING_READS${NC}"
        ((FAILED++))
    fi
fi

# 5. Create Secret with OTP
echo -e "\n${YELLOW}=== STEP 5: Create Shared Secret (With OTP) ===${NC}"

CREATE_OTP_PAYLOAD='{
  "receiver_email": "arkaitzmugica@protonmail.com",
  "secret_text": "Secret with OTP protection",
  "expires_hours": 12,
  "max_reads": 1,
  "require_otp": true,
  "send_copy_to_sender": true
}'

SENDER_HASH_OTP=""
RECEIVER_HASH_OTP=""

if test_endpoint \
    "Create secret with OTP" \
    "POST" \
    "$BASE_URL/api/shared-secret/create" \
    "200" \
    "$CREATE_OTP_PAYLOAD"; then

    SENDER_HASH_OTP=$(echo "$RESPONSE_PAYLOAD" | jq -r '.url_sender' | sed 's|.*/||')
    RECEIVER_HASH_OTP=$(echo "$RESPONSE_PAYLOAD" | jq -r '.url_receiver' | sed 's|.*/||')
    OTP_CODE=$(echo "$RESPONSE_PAYLOAD" | jq -r '.otp')

    echo -e "${GREEN}✓ OTP Code: $OTP_CODE${NC}"
fi

# 6. Try to view OTP-protected secret without OTP (should fail)
echo -e "\n${YELLOW}=== STEP 6: Try View OTP Secret Without OTP ===${NC}"

test_endpoint \
    "View OTP secret without OTP (should fail)" \
    "GET" \
    "$BASE_URL/api/shared-secret/$RECEIVER_HASH_OTP" \
    "400"

# 7. View OTP-protected secret WITH OTP
echo -e "\n${YELLOW}=== STEP 7: View OTP Secret With Correct OTP ===${NC}"

OTP_PAYLOAD="{\"otp\":\"$OTP_CODE\"}"

test_endpoint \
    "View OTP secret with correct OTP" \
    "POST" \
    "$BASE_URL/api/shared-secret/$RECEIVER_HASH_OTP" \
    "200" \
    "$OTP_PAYLOAD"

# 8. Delete Secret (should work if pending_reads > 0)
echo -e "\n${YELLOW}=== STEP 8: Delete Secret ===${NC}"

test_endpoint \
    "Delete secret" \
    "DELETE" \
    "$BASE_URL/api/shared-secret/$RECEIVER_HASH" \
    "200"

# 9. Try to view deleted secret (should 404)
echo -e "\n${YELLOW}=== STEP 9: Try View Deleted Secret ===${NC}"

test_endpoint \
    "View deleted secret (should 404)" \
    "GET" \
    "$BASE_URL/api/shared-secret/$RECEIVER_HASH" \
    "404"

# Cleanup
rm -f .test-shared-secret-pubkey

# Summary
echo -e "\n${PURPLE}╔═══════════════════════════════════════════════════════╗${NC}"
echo -e "${PURPLE}║                    Test Summary                      ║${NC}"
echo -e "${PURPLE}╠═══════════════════════════════════════════════════════╣${NC}"
echo -e "${PURPLE}║${NC}  ${GREEN}Passed:${NC} $PASSED/${TOTAL}                                    ${PURPLE}║${NC}"
echo -e "${PURPLE}║${NC}  ${RED}Failed:${NC} $FAILED/${TOTAL}                                    ${PURPLE}║${NC}"
echo -e "${PURPLE}╚═══════════════════════════════════════════════════════╝${NC}"

if [[ $FAILED -eq 0 ]]; then
    echo -e "\n${GREEN}🎉 All tests passed!${NC}"
    exit 0
else
    echo -e "\n${RED}❌ Some tests failed${NC}"
    exit 1
fi
