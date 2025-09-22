#!/bin/bash

# Signed Response Helpers for HashRand Test Scripts
#
# Provides bash functions to handle Ed25519 signed responses from the backend

# Global variable to store server public key for current session
SERVER_PUB_KEY=""

# Extract server public key from signed response
# Usage: extract_server_pub_key "response_json"
# Returns: server public key hex string or empty if not found
extract_server_pub_key() {
    local response="$1"
    local server_key

    server_key=$(node scripts/verify_signed_response.js extract-server-key "$response" 2>/dev/null)

    if [[ $? -eq 0 && -n "$server_key" ]]; then
        echo "$server_key"
        return 0
    else
        echo ""
        return 1
    fi
}

# Extract field from signed response payload without verification
# Usage: extract_field_from_payload "response_json" "field_name"
# Returns: field value or empty if not found
extract_field_from_payload() {
    local response="$1"
    local field_name="$2"
    local field_value

    field_value=$(node scripts/verify_signed_response.js extract-field "$response" "$field_name" 2>/dev/null)

    if [[ $? -eq 0 && -n "$field_value" ]]; then
        echo "$field_value"
        return 0
    else
        echo ""
        return 1
    fi
}

# Verify signed response signature
# Usage: verify_signed_response "response_json" "server_pub_key"
# Returns: 0 if valid, 1 if invalid
verify_signed_response() {
    local response="$1"
    local server_pub_key="$2"

    node scripts/verify_signed_response.js verify "$response" "$server_pub_key" >/dev/null 2>&1
    return $?
}

# Extract field from verified signed response
# Usage: extract_verified_field "response_json" "server_pub_key" "field_name"
# Returns: field value if signature is valid, empty otherwise
extract_verified_field() {
    local response="$1"
    local server_pub_key="$2"
    local field_name="$3"
    local field_value

    field_value=$(node scripts/verify_signed_response.js extract-verified "$response" "$server_pub_key" "$field_name" 2>/dev/null)

    if [[ $? -eq 0 && -n "$field_value" ]]; then
        echo "$field_value"
        return 0
    else
        echo ""
        return 1
    fi
}

# Process magic link response (extract and store server_pub_key, validate signature)
# Usage: process_magic_link_response "response_json"
# Returns: 0 if successful, 1 if failed
# Side effect: Sets global SERVER_PUB_KEY variable
process_magic_link_response() {
    local response="$1"

    echo -e "${BLUE}🔐 Processing signed magic link response...${NC}"

    # Extract server public key
    SERVER_PUB_KEY=$(extract_server_pub_key "$response")

    if [[ -z "$SERVER_PUB_KEY" ]]; then
        echo -e "${RED}✗ Failed to extract server_pub_key from magic link response${NC}"
        return 1
    fi

    echo -e "${GREEN}✓ Server public key extracted: ${SERVER_PUB_KEY:0:20}...${NC}"

    # Verify signature
    if verify_signed_response "$response" "$SERVER_PUB_KEY"; then
        echo -e "${GREEN}✓ Magic link response signature verified successfully${NC}"
        return 0
    else
        echo -e "${RED}✗ Magic link response signature verification failed${NC}"
        SERVER_PUB_KEY=""
        return 1
    fi
}

# Process refresh token response (validate signature using stored server_pub_key)
# Usage: process_refresh_response "response_json"
# Returns: 0 if successful, 1 if failed
# Requires: Global SERVER_PUB_KEY variable to be set
process_refresh_response() {
    local response="$1"

    if [[ -z "$SERVER_PUB_KEY" ]]; then
        echo -e "${RED}✗ No server public key available for refresh response verification${NC}"
        return 1
    fi

    echo -e "${BLUE}🔐 Verifying signed refresh response...${NC}"

    # Extract new server public key from refresh response (it should match the stored one)
    local new_server_key
    new_server_key=$(extract_server_pub_key "$response")

    if [[ -n "$new_server_key" ]]; then
        echo -e "${GREEN}✓ New server public key found: ${new_server_key:0:20}...${NC}"

        # Update stored key with new one from refresh response
        SERVER_PUB_KEY="$new_server_key"
    fi

    # Verify signature
    if verify_signed_response "$response" "$SERVER_PUB_KEY"; then
        echo -e "${GREEN}✓ Refresh response signature verified successfully${NC}"
        return 0
    else
        echo -e "${RED}✗ Refresh response signature verification failed${NC}"
        return 1
    fi
}

# Process regular signed response (validate signature using stored server_pub_key)
# Usage: process_regular_response "response_json"
# Returns: 0 if successful, 1 if failed
# Note: Regular responses typically don't include server_pub_key
process_regular_response() {
    local response="$1"

    if [[ -z "$SERVER_PUB_KEY" ]]; then
        echo -e "${YELLOW}⚠ No server public key available - skipping signature verification${NC}"
        return 0
    fi

    echo -e "${BLUE}🔐 Verifying signed response...${NC}"

    # Verify signature
    if verify_signed_response "$response" "$SERVER_PUB_KEY"; then
        echo -e "${GREEN}✓ Response signature verified successfully${NC}"
        return 0
    else
        echo -e "${RED}✗ Response signature verification failed${NC}"
        return 1
    fi
}

# Extract access token from signed JWT response
# Usage: extract_access_token "jwt_response"
# Returns: access token or empty if not found
extract_access_token() {
    local response="$1"

    # Try to extract from signed response first
    local access_token
    access_token=$(extract_field_from_payload "$response" "access_token")

    if [[ -n "$access_token" ]]; then
        echo "$access_token"
        return 0
    fi

    # Fallback to direct extraction (for backward compatibility)
    access_token=$(echo "$response" | grep -o '"access_token":"[^"]*' | cut -d'"' -f4)

    if [[ -n "$access_token" ]]; then
        echo "$access_token"
        return 0
    fi

    echo ""
    return 1
}

# Extract access token from verified signed JWT response
# Usage: extract_verified_access_token "jwt_response"
# Returns: access token if signature is valid, empty otherwise
extract_verified_access_token() {
    local response="$1"

    if [[ -z "$SERVER_PUB_KEY" ]]; then
        echo -e "${YELLOW}⚠ No server public key - extracting without verification${NC}"
        extract_access_token "$response"
        return $?
    fi

    local access_token
    access_token=$(extract_verified_field "$response" "$SERVER_PUB_KEY" "access_token")

    if [[ -n "$access_token" ]]; then
        echo "$access_token"
        return 0
    fi

    echo ""
    return 1
}

# Check if response is signed (has payload and signature fields)
# Usage: is_signed_response "response_json"
# Returns: 0 if signed, 1 if not signed
is_signed_response() {
    local response="$1"

    if echo "$response" | jq -e '.payload and .signature' >/dev/null 2>&1; then
        return 0
    else
        return 1
    fi
}

# Print debug information about signed response
# Usage: debug_signed_response "response_json"
debug_signed_response() {
    local response="$1"

    echo -e "${PURPLE}🔍 Signed Response Debug:${NC}"
    echo "Response length: ${#response}"

    if is_signed_response "$response"; then
        echo "✓ Has signed structure (payload + signature)"

        local signature
        signature=$(echo "$response" | jq -r '.signature' 2>/dev/null)
        if [[ -n "$signature" && "$signature" != "null" ]]; then
            echo "✓ Signature present: ${signature:0:20}..."
        fi

        local payload_keys
        payload_keys=$(echo "$response" | jq -r '.payload | keys | join(", ")' 2>/dev/null)
        if [[ -n "$payload_keys" && "$payload_keys" != "null" ]]; then
            echo "✓ Payload keys: $payload_keys"
        fi

        local server_key
        server_key=$(extract_server_pub_key "$response")
        if [[ -n "$server_key" ]]; then
            echo "✓ Server public key: ${server_key:0:20}..."
        else
            echo "- No server public key in payload"
        fi
    else
        echo "- Not a signed response (missing payload/signature)"
    fi
    echo ""
}