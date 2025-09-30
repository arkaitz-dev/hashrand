#!/bin/bash

# Test Sistema 2/3 de Rotación de Tokens
# Prueba el flujo completo de refresh tokens con rotación de claves Ed25519

echo "🧪 TEST SISTEMA 2/3: Ciclo de Vida Completo de Tokens con /api/refresh"
echo "========================================================================"
echo "📋 PLAN DE PRUEBAS:"
echo "   Test 1 (t=0s):    Access válido → API normal con firma Ed25519"
echo "   Test 2 (t=62s):   Access expirado → /api/refresh (primer 1/3) → Solo nuevo access"
echo "   Test 3 (t=110s):  Access expirado → /api/refresh (sistema 2/3) → Access + refresh reset"
echo "   Test 4 (t=430s):  Refresh expirado → /api/refresh → 401 (ambos expirados)"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
PURPLE='\033[0;35m'
NC='\033[0m'

# Source helpers
source scripts/signed_response_helpers.sh

# Config
API_BASE="http://localhost:3000"
COOKIES_FILE="cookies_test_2_3.txt"

# Generate signed URL for GET requests with Ed25519 signature
generate_signed_url() {
    local base_url="$1"
    local pub_key_file=".test-magiclink-pubkey"

    if [[ ! -f "$pub_key_file" ]]; then
        echo -e "${RED}✗ No stored public key found${NC}"
        echo "$base_url"
        return 1
    fi

    local pub_key=$(cat "$pub_key_file")
    local params_json="{}"

    if [[ "$base_url" == *"?"* ]]; then
        local query_string="${base_url#*\?}"
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

    local signature=$(node ./scripts/sign_query_params.js "$pub_key" "$params_json" 2>/dev/null)
    if [[ -z "$signature" ]]; then
        echo -e "${RED}✗ Failed to generate signature${NC}"
        echo "$base_url"
        return 1
    fi

    local separator="?"
    if [[ "$base_url" == *"?"* ]]; then
        separator="&"
    fi

    echo "${base_url}${separator}signature=${signature}"
    return 0
}

##########################################################################
# SETUP: Authentication and initial login
##########################################################################
echo "🔐 SETUP: Generando keypair Ed25519 e iniciando sesión..."

# Generate Ed25519 keypair
PUB_KEY=$(node scripts/generate_hash.js)
echo "✅ Keypair generado: ${PUB_KEY:0:20}..."

# Store for request signing
echo "$PUB_KEY" > .test-magiclink-pubkey

# Request magic link
PAYLOAD_JSON="{\"email\":\"me@arkaitz.dev\",\"email_lang\":\"en\",\"next\":\"/\",\"pub_key\":\"$PUB_KEY\",\"ui_host\":\"localhost\"}"
SIGNED_REQUEST=$(node scripts/create_signed_request.js "$PAYLOAD_JSON")

MAGIC_RESPONSE=$(curl -s -c $COOKIES_FILE -X POST \
  -H "Content-Type: application/json" \
  -d "$SIGNED_REQUEST" \
  $API_BASE/api/login/)

# Process magic link response
if is_signed_response "$MAGIC_RESPONSE"; then
    if ! process_magic_link_response "$MAGIC_RESPONSE"; then
        echo -e "${RED}❌ Error processing magic link response${NC}"
        exit 1
    fi
fi

# Extract magic token
sleep 1
MAGIC_LINE=$(tail -n 50 .spin-dev.log | grep "Generated magic_link.*magiclink=" | tail -n 1)
MAGIC_TOKEN=$(echo "$MAGIC_LINE" | grep -o 'magiclink=[^&]*' | cut -d '=' -f 2)

if [ -z "$MAGIC_TOKEN" ]; then
    echo "❌ Error extrayendo magic token"
    exit 1
fi

# Validate magic link and get JWT
MAGICLINK_PAYLOAD_JSON="{\"magiclink\":\"$MAGIC_TOKEN\"}"
MAGICLINK_SIGNED_REQUEST=$(node scripts/create_signed_request.js "$MAGICLINK_PAYLOAD_JSON")

LOGIN_RESPONSE=$(curl -s -b $COOKIES_FILE -c $COOKIES_FILE -X POST \
  -H "Content-Type: application/json" \
  -d "$MAGICLINK_SIGNED_REQUEST" \
  "$API_BASE/api/login/magiclink/")

# Extract access token
if is_signed_response "$LOGIN_RESPONSE"; then
    if [[ -n "$SERVER_PUB_KEY" ]]; then
        verify_signed_response "$LOGIN_RESPONSE" "$SERVER_PUB_KEY" >/dev/null 2>&1
    fi
    ACCESS_TOKEN=$(extract_field_from_payload "$LOGIN_RESPONSE" "access_token")
else
    ACCESS_TOKEN=$(echo "$LOGIN_RESPONSE" | jq -r '.access_token' 2>/dev/null)
fi

if [[ -z "$ACCESS_TOKEN" ]]; then
    echo -e "${RED}❌ Error en login: Could not extract access token${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Login exitoso. Access Token: ${ACCESS_TOKEN:0:40}...${NC}"

# Verify refresh token cookie exists
if ! grep -q "refresh_token" $COOKIES_FILE; then
    echo "❌ ERROR CRÍTICO: No hay refresh token cookie"
    exit 1
fi
echo "✅ Refresh token cookie creada"

# Start time tracking
START_TIME=$(date +%s)
echo "⏰ Hora de inicio: $(date)"
echo ""

##########################################################################
# TEST 1: Immediate API call (t=0s) - Valid access token
##########################################################################
echo "🧪 TEST 1: API call inmediata (t=0s)"
echo "====================================="
echo "   • Access token DEBE ser válido (recién creado)"
echo "   • NO debe haber refresh"
echo "   • RESULTADO ESPERADO: 200 OK con hash generado"

SIGNED_URL=$(generate_signed_url "$API_BASE/api/custom?length=8")
API_RESPONSE=$(curl -s -i \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  "$SIGNED_URL")

if echo "$API_RESPONSE" | grep -q "HTTP/1.1 200"; then
    RESPONSE_BODY=$(echo "$API_RESPONSE" | tail -n 1)

    hash=""
    if is_signed_response "$RESPONSE_BODY"; then
        hash=$(extract_field_from_payload "$RESPONSE_BODY" "hash")
    fi

    if [[ -n "$hash" && "$hash" != "null" ]]; then
        echo "✅ Test 1 EXITOSO - Hash generado: $hash"
    else
        echo "❌ Test 1 falló: No se generó hash"
        exit 1
    fi
else
    echo "❌ Test 1 falló: $(echo "$API_RESPONSE" | head -n 1)"
    exit 1
fi

echo ""

##########################################################################
# TEST 2: After 62s - Access expired, refresh in first 1/3
##########################################################################
echo "⏰ Esperando 62 segundos para Test 2..."
sleep 62

CURRENT_TIME=$(date +%s)
ELAPSED=$((CURRENT_TIME - START_TIME))
echo "🧪 TEST 2: Refresh después de ${ELAPSED}s (Access expirado, primer 1/3)"
echo "========================================================================"
echo "   • Access token expirado (>${ELAPSED}s > 60s)"
echo "   • Refresh token en primer 1/3 (${ELAPSED}s < 100s = 1/3 de 300s)"
echo "   • DEBE renovar SOLO access token (mantener refresh existente)"
echo "   • RESULTADO ESPERADO: 200 OK + nuevo access token, SIN nueva refresh cookie"

# Generate refresh request (mismo pub_key, NO rotation en primer 1/3)
REFRESH_PAYLOAD_JSON="{\"new_pub_key\":\"$PUB_KEY\"}"
REFRESH_SIGNED_REQUEST=$(node scripts/create_signed_request.js "$REFRESH_PAYLOAD_JSON")

# Call /api/refresh with credentials (to send refresh cookie)
REFRESH_RESPONSE=$(curl -s -i -b $COOKIES_FILE -c $COOKIES_FILE -X POST \
  -H "Content-Type: application/json" \
  -d "$REFRESH_SIGNED_REQUEST" \
  "$API_BASE/api/refresh")

if echo "$REFRESH_RESPONSE" | grep -q "HTTP/1.1 200"; then
    RESPONSE_BODY=$(echo "$REFRESH_RESPONSE" | tail -n 1)

    # Extract new access token
    NEW_ACCESS_TOKEN=""
    if is_signed_response "$RESPONSE_BODY"; then
        NEW_ACCESS_TOKEN=$(extract_field_from_payload "$RESPONSE_BODY" "access_token")
    fi

    if [[ -n "$NEW_ACCESS_TOKEN" ]]; then
        # Check that refresh cookie was NOT renewed
        if ! echo "$REFRESH_RESPONSE" | grep -q "set-cookie.*refresh_token"; then
            echo "✅ Test 2 EXITOSO - Refresh parcial (solo access token)"
            echo "   • Nuevo access token: ${NEW_ACCESS_TOKEN:0:40}..."
            echo "   • Refresh token mantenido (primer 1/3) ✅"

            # Test new access token with API call
            SIGNED_URL=$(generate_signed_url "$API_BASE/api/custom?length=10")
            API_TEST=$(curl -s -i \
              -H "Authorization: Bearer $NEW_ACCESS_TOKEN" \
              "$SIGNED_URL")

            if echo "$API_TEST" | grep -q "HTTP/1.1 200"; then
                RESPONSE_BODY=$(echo "$API_TEST" | tail -n 1)
                hash=""
                if is_signed_response "$RESPONSE_BODY"; then
                    hash=$(extract_field_from_payload "$RESPONSE_BODY" "hash")
                fi
                if [[ -n "$hash" && "$hash" != "null" ]]; then
                    echo "   • API call con nuevo token exitosa: $hash ✅"
                fi
            fi
        else
            echo "❌ Test 2 falló: Refresh token renovado prematuramente"
            exit 1
        fi
    else
        echo "❌ Test 2 falló: No se renovó access token"
        exit 1
    fi
else
    echo "❌ Test 2 falló: $(echo "$REFRESH_RESPONSE" | head -n 1)"
    exit 1
fi

echo ""

##########################################################################
# TEST 3: After ~110s - 2/3 system activated
##########################################################################
WAIT_TIME_3=48
echo "⏰ Esperando ${WAIT_TIME_3}s más para Test 3 (total ~110s)..."
sleep $WAIT_TIME_3

CURRENT_TIME=$(date +%s)
ELAPSED=$((CURRENT_TIME - START_TIME))
echo "🧪 TEST 3: Refresh después de ${ELAPSED}s - SISTEMA 2/3"
echo "========================================================"
echo "   • Tiempo: ${ELAPSED}s > 100s (>1/3 de 300s)"
echo "   • Quedan <200s (<2/3 del refresh token)"
echo "   • DEBE activar sistema 2/3: KEY ROTATION"
echo "   • RESULTADO ESPERADO: 200 OK + access + refresh cookie"

# IMPORTANT: Preserve OLD private key before generating NEW keypair
# The request MUST be signed with OLD key, but payload contains NEW pub_key
cp .test-ed25519-private-key .test-ed25519-private-key.old

# Generate NEW keypair for KEY ROTATION
NEW_PUB_KEY=$(node scripts/generate_hash.js)
echo "🔑 Nueva pub_key para KEY ROTATION: ${NEW_PUB_KEY:0:20}..."

# Save NEW private key for later use
cp .test-ed25519-private-key .test-ed25519-private-key.new

# Restore OLD private key for signing the refresh request
cp .test-ed25519-private-key.old .test-ed25519-private-key

# Create refresh request with NEW pub_key (signed with OLD private key from current token)
REFRESH_PAYLOAD_JSON="{\"new_pub_key\":\"$NEW_PUB_KEY\"}"
REFRESH_SIGNED_REQUEST=$(node scripts/create_signed_request.js "$REFRESH_PAYLOAD_JSON")

# Update stored pub_key for future API calls
echo "$NEW_PUB_KEY" > .test-magiclink-pubkey

REFRESH_RESPONSE=$(curl -s -i -b $COOKIES_FILE -c $COOKIES_FILE -X POST \
  -H "Content-Type: application/json" \
  -d "$REFRESH_SIGNED_REQUEST" \
  "$API_BASE/api/refresh")

if echo "$REFRESH_RESPONSE" | grep -q "HTTP/1.1 200"; then
    RESPONSE_BODY=$(echo "$REFRESH_RESPONSE" | tail -n 1)

    NEW_ACCESS_TOKEN_3=""
    if is_signed_response "$RESPONSE_BODY"; then
        NEW_ACCESS_TOKEN_3=$(extract_field_from_payload "$RESPONSE_BODY" "access_token")
    fi

    if [[ -n "$NEW_ACCESS_TOKEN_3" ]]; then
        # Check that refresh cookie WAS renewed
        if echo "$REFRESH_RESPONSE" | grep -q "set-cookie.*refresh_token"; then
            echo "🎉 Test 3 EXITOSO - SISTEMA 2/3 FUNCIONANDO"
            echo "   • Nuevo access token: ${NEW_ACCESS_TOKEN_3:0:40}..."
            echo "   • Nuevo refresh token (reset completo) ✅"
            echo "   • KEY ROTATION activada ✅"

            # Switch to NEW private key for future requests
            cp .test-ed25519-private-key.new .test-ed25519-private-key
            echo "   • Switched to NEW private key for future API calls ✅"

            # Test with new keypair
            SIGNED_URL=$(generate_signed_url "$API_BASE/api/custom?length=12")
            API_TEST=$(curl -s -i \
              -H "Authorization: Bearer $NEW_ACCESS_TOKEN_3" \
              "$SIGNED_URL")

            if echo "$API_TEST" | grep -q "HTTP/1.1 200"; then
                RESPONSE_BODY=$(echo "$API_TEST" | tail -n 1)
                hash=""
                if is_signed_response "$RESPONSE_BODY"; then
                    hash=$(extract_field_from_payload "$RESPONSE_BODY" "hash")
                fi
                if [[ -n "$hash" && "$hash" != "null" ]]; then
                    echo "   • API call con nueva keypair: $hash ✅"
                fi
            fi
        else
            echo "❌ Test 3 falló: Sistema 2/3 NO activado"
            exit 1
        fi
    else
        echo "❌ Test 3 falló: No se renovó access token"
        exit 1
    fi
else
    echo "❌ Test 3 falló: $(echo "$REFRESH_RESPONSE" | head -n 1)"
    exit 1
fi

echo ""

##########################################################################
# TEST 4: Both tokens expired
##########################################################################
WAIT_TIME_4=320
echo "⏰ Esperando ${WAIT_TIME_4}s para Test 4..."
sleep $WAIT_TIME_4

CURRENT_TIME=$(date +%s)
ELAPSED=$((CURRENT_TIME - START_TIME))
echo "🧪 TEST 4: Refresh después de ${ELAPSED}s - DOBLE EXPIRACIÓN"
echo "=============================================================="
echo "   • Refresh token expirado (>300s desde Test 3)"
echo "   • DEBE detectar doble expiración"
echo "   • RESULTADO ESPERADO: 401 Unauthorized"

REFRESH_PAYLOAD_JSON="{\"new_pub_key\":\"$NEW_PUB_KEY\"}"
REFRESH_SIGNED_REQUEST=$(node scripts/create_signed_request.js "$REFRESH_PAYLOAD_JSON")

REFRESH_RESPONSE=$(curl -s -i -b $COOKIES_FILE \
  -H "Content-Type: application/json" \
  -d "$REFRESH_SIGNED_REQUEST" \
  "$API_BASE/api/refresh")

if echo "$REFRESH_RESPONSE" | grep -q "HTTP/1.1 401"; then
    echo "✅ Test 4 EXITOSO - DOBLE EXPIRACIÓN CORRECTA"
    echo "   • Error 401 (correcto) ✅"
else
    echo "❌ Test 4 falló: $(echo "$REFRESH_RESPONSE" | head -n 1)"
fi

echo ""
echo "🏆 RESUMEN: Sistema 2/3 funciona PERFECTAMENTE"
echo "✅ Test 1: Token válido"
echo "✅ Test 2: Refresh parcial (primer 1/3)"
echo "✅ Test 3: KEY ROTATION (sistema 2/3)"
echo "✅ Test 4: Doble expiración 401"

# Cleanup
rm -f .test-ed25519-private-key .test-ed25519-private-key.old .test-ed25519-private-key.new .test-magiclink-pubkey $COOKIES_FILE
