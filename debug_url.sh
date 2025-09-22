#!/bin/bash

# Extract query parameters from URL
debug_parse_url() {
    local base_url="$1"
    local params_json="{}"

    echo "=== DEBUG URL PARSING ==="
    echo "Input URL: $base_url"

    if [[ "$base_url" == *"?"* ]]; then
        local query_string="${base_url#*\?}"
        local base_url_no_query="${base_url%%\?*}"

        echo "Query string: '$query_string'"
        echo "Base URL no query: '$base_url_no_query'"

        # Convert query string to JSON for signing
        params_json="{"
        local first=true
        IFS='&' read -ra PAIRS <<< "$query_string"
        echo "Number of pairs: ${#PAIRS[@]}"
        for pair in "${PAIRS[@]}"; do
            echo "Processing pair: '$pair'"
            if [[ "$pair" == *"="* ]]; then
                local key="${pair%%=*}"
                local value="${pair#*=}"
                echo "  Key: '$key', Value: '$value'"
                if [[ "$first" == "true" ]]; then
                    first=false
                else
                    params_json+=","
                fi
                params_json+="\"$key\":\"$value\""
            else
                echo "  No '=' found in pair"
            fi
        done
        params_json+="}"
    else
        local base_url_no_query="$base_url"
        echo "No query string found"
    fi

    echo "Final JSON: '$params_json'"
}

debug_parse_url "http://localhost:3000/api/custom?length=12"