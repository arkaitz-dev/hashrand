#!/bin/bash

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
PURPLE='\033[0;35m'
NC='\033[0m'

# Source helpers
source scripts/signed_response_helpers.sh

# Generate signed URL for GET requests with Ed25519 signature
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
    fi

    # Generate Ed25519 signature for query parameters
    echo "DEBUG: Signing params JSON: $params_json"
    local signature=$(node ./scripts/sign_query_params.js "$pub_key" "$params_json" 2>/dev/null)
    if [[ -z "$signature" ]]; then
        echo -e "${RED}✗ Failed to generate Ed25519 signature for query parameters${NC}"
        echo "$base_url"
        return 1
    fi
    echo "DEBUG: Generated signature: ${signature:0:20}..."

    # Add only signature to URL (backend extracts pub_key from JWT Bearer token)
    local separator="?"
    if [[ "$base_url" == *"?"* ]]; then
        separator="&"
    fi

    echo "${base_url}${separator}signature=${signature}"
    return 0
}

echo "Testing signed URL generation..."
echo "Public key: $(cat .test-magiclink-pubkey)"
echo ""

signed_url=$(generate_signed_url "http://localhost:3000/api/custom?length=12")
echo "Result: $signed_url"