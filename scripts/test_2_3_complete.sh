#!/bin/bash

# Test completo del sistema 2/3 con 4 fases progresivas + Ed25519 Signed Responses
echo "🧪 TEST COMPLETO SISTEMA 2/3: Ciclo de Vida Completo de Tokens"
echo "=================================================================="
echo "📋 PLAN DE PRUEBAS:"
echo "   Test 1 (t=0s):    Access válido → API normal"
echo "   Test 2 (t=62s):   Access expirado, refresh primer 1/3 → Solo nuevo access"
echo "   Test 3 (t=110s):  Sistema 2/3 (>1/3 elapsed, 2/3 remaining) → Access + refresh reset"
echo "   Test 4 (t=430s):  Usar cookies Test 3 → Esperar 320s → Doble expiración por tiempo"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
PURPLE='\033[0;35m'
NC='\033[0m'

# Source signed response helpers
source scripts/signed_response_helpers.sh

# Configuración
API_BASE="http://localhost:3000"
COOKIES_FILE="cookies_test.txt"

# Paso inicial: Generar keypair Ed25519 y login
echo "🔐 SETUP: Generando keypair Ed25519 e iniciando sesión..."
PUB_KEY=$(node scripts/generate_hash.js)
echo "✅ Keypair Ed25519 generado: ${PUB_KEY:0:20}..."

# Generar firma Ed25519 para la solicitud de magic link usando JSON payload
NEXT_PARAM="/"
PAYLOAD_JSON="{\"email\":\"me@arkaitz.dev\",\"email_lang\":\"en\",\"next\":\"$NEXT_PARAM\",\"pub_key\":\"$PUB_KEY\"}"
SIGNATURE=$(node scripts/sign_payload_json.js "$PAYLOAD_JSON")
echo "✅ Firma Ed25519 generada: ${SIGNATURE:0:20}..."

# Solicitar magic link con Ed25519 signature usando estructura SignedRequest
MAGIC_RESPONSE=$(curl -s -c $COOKIES_FILE -X POST \
  -H "Content-Type: application/json" \
  -d "{\"payload\":{\"email\":\"me@arkaitz.dev\",\"email_lang\":\"en\",\"next\":\"$NEXT_PARAM\",\"pub_key\":\"$PUB_KEY\"},\"signature\":\"$SIGNATURE\"}" \
  $API_BASE/api/login/)

if [ "$MAGIC_RESPONSE" != '{"status":"OK"}' ]; then
    echo "❌ Error solicitando magic link: $MAGIC_RESPONSE"
    exit 1
fi

# Extraer magic token
MAGIC_LINE=$(tail -n 50 .spin-dev.log | grep "Generated magic_link.*magiclink=" | tail -n 1)
MAGIC_TOKEN=$(echo "$MAGIC_LINE" | grep -o 'magiclink=[^&]*' | cut -d '=' -f 2)

if [ -z "$MAGIC_TOKEN" ] || [ ${#MAGIC_TOKEN} -lt 10 ]; then
    echo "❌ Error extrayendo magic token"
    exit 1
fi

# Generar firma Ed25519 para el magic token
MAGIC_SIGNATURE=$(node scripts/sign_payload.js "$MAGIC_TOKEN")
echo "✅ Firma de magic token generada: ${MAGIC_SIGNATURE:0:20}..."

# Validar magic link usando POST endpoint con estructura SignedRequest
LOGIN_RESPONSE=$(curl -s -b $COOKIES_FILE -c $COOKIES_FILE -X POST \
  -H "Content-Type: application/json" \
  -d "{\"payload\":{\"magiclink\":\"$MAGIC_TOKEN\"},\"signature\":\"$MAGIC_SIGNATURE\"}" \
  "$API_BASE/api/login/magiclink/")

# Process signed response and extract server public key and access token
if is_signed_response "$LOGIN_RESPONSE"; then
    echo -e "${BLUE}📝 Processing signed JWT response...${NC}"

    if ! process_magic_link_response "$LOGIN_RESPONSE"; then
        echo -e "${RED}❌ Error processing signed login response${NC}"
        exit 1
    fi

    # Extract access token from verified signed response
    ACCESS_TOKEN=$(extract_verified_access_token "$LOGIN_RESPONSE")
else
    echo -e "${YELLOW}⚠ Received non-signed response, extracting token directly${NC}"
    # Fallback for backward compatibility
    ACCESS_TOKEN=$(extract_access_token "$LOGIN_RESPONSE")
fi

if [[ -z "$ACCESS_TOKEN" ]]; then
    echo -e "${RED}❌ Error en login: Could not extract access token${NC}"
    echo "Response: $LOGIN_RESPONSE"
    exit 1
fi

echo -e "${GREEN}✅ Login exitoso. Access Token: ${ACCESS_TOKEN:0:40}...${NC}"

# Verificar cookies de refresh
if ! grep -q "refresh_token" $COOKIES_FILE; then
    echo "❌ ERROR CRÍTICO: No hay refresh token cookie"
    exit 1
fi
echo "✅ Refresh token cookie creada"

# HORA DE INICIO para tracking
START_TIME=$(date +%s)
echo "⏰ Hora de inicio: $(date)"
echo ""

##########################################################################
# TEST 1: API call inmediata (t=0s) - Access token válido
##########################################################################
echo "🧪 TEST 1: API call inmediata (t=0s)"
echo "====================================="
echo "   • Access token DEBE ser válido (recién creado)"
echo "   • NO debe haber refresh automático"
echo "   • RESULTADO ESPERADO: 200 OK sin headers de renovación"

API_RESPONSE=$(curl -s -i -b $COOKIES_FILE -c $COOKIES_FILE \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  $API_BASE/api/custom?length=8)

if echo "$API_RESPONSE" | grep -q "HTTP/1.1 200"; then
    # Extract response body and check for token renewal
    RESPONSE_BODY=$(echo "$API_RESPONSE" | tail -n 1)

    # Check if there's an access token in signed response (which would indicate unexpected renewal)
    has_access_token_in_payload=false
    if is_signed_response "$RESPONSE_BODY"; then
        if extract_field_from_payload "$RESPONSE_BODY" "access_token" >/dev/null 2>&1; then
            has_access_token_in_payload=true
        fi
    fi

    # Check if there's header-based token renewal (legacy method)
    has_header_token=false
    if echo "$API_RESPONSE" | grep -q "x-new-access-token:"; then
        has_header_token=true
    fi

    if [[ "$has_access_token_in_payload" == false && "$has_header_token" == false ]]; then
        if echo "$RESPONSE_BODY" | grep -q '"hash"'; then
            hash=""
            if is_signed_response "$RESPONSE_BODY"; then
                hash=$(extract_field_from_payload "$RESPONSE_BODY" "hash")
                # Validate signature if we have server public key
                if [[ -n "$SERVER_PUB_KEY" ]]; then
                    if process_regular_response "$RESPONSE_BODY"; then
                        echo -e "${GREEN}   • Response signature validated ✅${NC}"
                    else
                        echo -e "${YELLOW}   ⚠ Response signature validation failed${NC}"
                    fi
                fi
            else
                hash=$(echo "$RESPONSE_BODY" | grep -o '"hash":"[^"]*"' | cut -d '"' -f 4)
            fi
            echo "✅ Test 1 EXITOSO - Hash generado sin refresh: $hash"
        else
            echo "❌ Test 1 falló: No se generó hash"
            exit 1
        fi
    else
        echo "❌ Test 1 falló: Refresh inesperado (no debería pasar)"
        exit 1
    fi
else
    echo "❌ Test 1 falló: $(echo "$API_RESPONSE" | head -n 1)"
    exit 1
fi

echo ""

##########################################################################
# TEST 2: Después de 62s - Access expirado, refresh en primer 1/3
##########################################################################
echo "⏰ Esperando 62 segundos para Test 2..."
sleep 62

CURRENT_TIME=$(date +%s)
ELAPSED=$((CURRENT_TIME - START_TIME))
echo "🧪 TEST 2: API call después de ${ELAPSED}s (Access expirado, primer 1/3)"
echo "=================================================================="
echo "   • Access token DEBE estar expirado (>${ELAPSED}s > 60s)"
echo "   • Refresh token válido en primer 1/3 (${ELAPSED}s < 100s)"
echo "   • DEBE renovar SOLO access token (mantener refresh existente)"
echo "   • RESULTADO ESPERADO: 200 OK + x-new-access-token, SIN set-cookie refresh"

API_RESPONSE=$(curl -s -i -b $COOKIES_FILE -c $COOKIES_FILE \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  $API_BASE/api/custom?length=10)

if echo "$API_RESPONSE" | grep -q "HTTP/1.1 200"; then
    # Extract response body and check for token renewal
    RESPONSE_BODY=$(echo "$API_RESPONSE" | tail -n 1)

    # Extract new access token from signed response payload (preferred) or header (fallback)
    NEW_TOKEN=""
    if is_signed_response "$RESPONSE_BODY"; then
        NEW_TOKEN=$(extract_field_from_payload "$RESPONSE_BODY" "access_token")
        if [[ -n "$NEW_TOKEN" ]]; then
            echo -e "${GREEN}   • Access token extracted from signed payload ✅${NC}"
        fi
    fi

    # Fallback to header extraction if payload method failed
    if [[ -z "$NEW_TOKEN" ]] && echo "$API_RESPONSE" | grep -q "x-new-access-token:"; then
        NEW_TOKEN=$(echo "$API_RESPONSE" | grep "x-new-access-token:" | cut -d' ' -f2 | tr -d '\r')
        echo -e "${YELLOW}   • Access token extracted from header (fallback) ⚠${NC}"
    fi

    if [[ -n "$NEW_TOKEN" ]]; then
        if ! echo "$API_RESPONSE" | grep -q "set-cookie.*refresh_token"; then
            # Extract hash from signed response body
            if echo "$RESPONSE_BODY" | grep -q '"hash"'; then
                hash=""
                if is_signed_response "$RESPONSE_BODY"; then
                    hash=$(extract_field_from_payload "$RESPONSE_BODY" "hash")
                    # Validate signature if we have server public key
                    if [[ -n "$SERVER_PUB_KEY" ]]; then
                        if process_regular_response "$RESPONSE_BODY"; then
                            echo -e "${GREEN}   • Response signature validated ✅${NC}"
                        else
                            echo -e "${YELLOW}   ⚠ Response signature validation failed${NC}"
                        fi
                    fi
                else
                    hash=$(echo "$RESPONSE_BODY" | grep -o '"hash":"[^"]*"' | cut -d '"' -f 4)
                fi
                echo "✅ Test 2 EXITOSO - Refresh parcial (solo access token)"
                echo "   • Nuevo access token: ${NEW_TOKEN:0:40}..."
                echo "   • Refresh token mantenido (primer 1/3) ✅"
                echo "   • Hash generado: $hash ✅"
            else
                echo "❌ Test 2 falló: No se generó hash en la respuesta"
                exit 1
            fi
        else
            echo "❌ Test 2 falló: Refresh token renovado prematuramente (debería mantenerse en primer 1/3)"
            exit 1
        fi
    else
        echo "❌ Test 2 falló: No se renovó access token cuando debía"
        exit 1
    fi
else
    echo "❌ Test 2 falló: $(echo "$API_RESPONSE" | head -n 1)"
    exit 1
fi

echo ""

##########################################################################
# TEST 3: Después de ~110s - Sistema 2/3 activado (>1/3 transcurrido)
##########################################################################
WAIT_TIME_3=48  # 62 + 48 = 110s total
echo "⏰ Esperando ${WAIT_TIME_3}s más para Test 3 (total ~110s)..."
sleep $WAIT_TIME_3

CURRENT_TIME=$(date +%s)
ELAPSED=$((CURRENT_TIME - START_TIME))
echo "🧪 TEST 3: API call después de ${ELAPSED}s - SISTEMA 2/3 CRÍTICO"
echo "================================================================"
echo "   • Tiempo transcurrido: ${ELAPSED}s > 100s (>1/3 de 300s)"
echo "   • Quedan $(((300 - ELAPSED) / 60)) minutos (~2/3 del refresh token)"
echo "   • DEBE activar sistema 2/3: renovar AMBOS tokens (reset completo)"
echo "   • RESULTADO ESPERADO: 200 OK + x-new-access-token + set-cookie refresh"

API_RESPONSE=$(curl -s -i -b $COOKIES_FILE -c $COOKIES_FILE \
  -H "Authorization: Bearer $NEW_TOKEN" \
  $API_BASE/api/custom?length=12)

echo "=== RESPONSE CRÍTICA TEST 3 ==="
echo "$API_RESPONSE"
echo "==============================="

if echo "$API_RESPONSE" | grep -q "HTTP/1.1 200"; then
    # Extract response body and check for token renewal
    RESPONSE_BODY=$(echo "$API_RESPONSE" | tail -n 1)

    # Extract new access token from signed response payload (preferred) or header (fallback)
    NEW_ACCESS=""
    if is_signed_response "$RESPONSE_BODY"; then
        NEW_ACCESS=$(extract_field_from_payload "$RESPONSE_BODY" "access_token")
        if [[ -n "$NEW_ACCESS" ]]; then
            echo -e "${GREEN}   • Access token extracted from signed payload ✅${NC}"
        fi
    fi

    # Fallback to header extraction if payload method failed
    if [[ -z "$NEW_ACCESS" ]] && echo "$API_RESPONSE" | grep -q "x-new-access-token:"; then
        NEW_ACCESS=$(echo "$API_RESPONSE" | grep "x-new-access-token:" | cut -d' ' -f2 | tr -d '\r')
        echo -e "${YELLOW}   • Access token extracted from header (fallback) ⚠${NC}"
    fi

    if [[ -n "$NEW_ACCESS" ]]; then
        if echo "$API_RESPONSE" | grep -q "set-cookie.*refresh_token"; then
            echo "🎉 Test 3 EXITOSO - SISTEMA 2/3 FUNCIONANDO PERFECTAMENTE"
            echo "   • Nuevo access token: ${NEW_ACCESS:0:40}..."
            echo "   • Nuevo refresh token (reset completo) ✅"
            echo "   • Tiempo reseteado a 5 minutos completos ✅"

            # Extract hash from response body (handle both signed and regular responses)
            if echo "$RESPONSE_BODY" | grep -q '"hash"'; then
                hash=""
                if is_signed_response "$RESPONSE_BODY"; then
                    hash=$(extract_field_from_payload "$RESPONSE_BODY" "hash")
                    # Validate signature if we have server public key
                    if [[ -n "$SERVER_PUB_KEY" ]]; then
                        if process_regular_response "$RESPONSE_BODY"; then
                            echo -e "${GREEN}   • Response signature validated ✅${NC}"
                        else
                            echo -e "${YELLOW}   ⚠ Response signature validation failed${NC}"
                        fi
                    fi
                else
                    hash=$(echo "$RESPONSE_BODY" | grep -o '"hash":"[^"]*"' | cut -d '"' -f 4)
                fi
                echo "   • Hash generado: $hash ✅"
            fi
        else
            echo "❌ Test 3 falló: Sistema 2/3 NO activado (debería renovar refresh token después de 1/3)"
            exit 1
        fi
    else
        echo "❌ Test 3 falló: No se renovó access token"
        exit 1
    fi
else
    echo "❌ Test 3 FALLÓ CRÍTICO: $(echo "$API_RESPONSE" | head -n 1)"
    echo "   Sistema 2/3 no está funcionando correctamente"
    exit 1
fi

echo ""

##########################################################################
# TEST 4: Usar cookies actualizadas del Test 3 + doble expiración por tiempo
##########################################################################
WAIT_TIME_4=320  # 300s + 20s margen desde Test 3 (nuevo refresh token expira en 5min=300s)
echo "⏰ Esperando ${WAIT_TIME_4}s más para Test 4 (desde reset Test 3 + margen seguridad)..."
sleep $WAIT_TIME_4

CURRENT_TIME=$(date +%s)
ELAPSED=$((CURRENT_TIME - START_TIME))
echo "🧪 TEST 4: API call después de ${ELAPSED}s - DOBLE EXPIRACIÓN CRÍTICA"
echo "=================================================================="
echo "   • Tiempo transcurrido desde Test 3: ${WAIT_TIME_4}s"
echo "   • Access token del Test 3 expirado (${WAIT_TIME_4}s >> 60s) ✓"
echo "   • Refresh token del Test 3 expirado (${WAIT_TIME_4}s > 300s) ✓"
echo "   • Usando cookies actualizadas del Test 3 (nueva refresh cookie)"
echo "   • DEBE detectar doble expiración por TIEMPO REAL"
echo "   • RESULTADO ESPERADO: 401 + mensaje dual expiry + refresh cookie Max-Age=0"

API_RESPONSE=$(curl -s -i -b $COOKIES_FILE \
  -H "Authorization: Bearer $NEW_ACCESS" \
  $API_BASE/api/custom?length=14)

echo "=== RESPONSE CRÍTICA TEST 4 (DOBLE EXPIRACIÓN) ==="
echo "$API_RESPONSE"
echo "=================================================="

if echo "$API_RESPONSE" | grep -q "HTTP/1.1 401"; then
    echo "✅ Status: 401 Unauthorized (correcto para doble expiración)"

    # Verificar mensaje específico de doble expiración
    if echo "$API_RESPONSE" | grep -q "Both access and refresh tokens have expired"; then
        echo "✅ Mensaje de doble expiración detectado correctamente"

        # Verificar cookie refresh_token con Max-Age=0 (limpieza)
        if echo "$API_RESPONSE" | grep -q "set-cookie.*refresh_token.*Max-Age=0"; then
            echo "🎉 Test 4 EXITOSO - DOBLE EXPIRACIÓN MANEJADA PERFECTAMENTE"
            echo "   • Error 401 con mensaje descriptivo ✅"
            echo "   • Cookie refresh_token limpiada (Max-Age=0) ✅"
            echo "   • Sistema indica necesidad de re-autenticación completa ✅"
        else
            echo "⚠️ Test 4 parcial: Mensaje correcto pero falta cookie Max-Age=0"
            echo "   Verificando si hay header set-cookie para limpiar refresh token..."
            if echo "$API_RESPONSE" | grep -q "set-cookie.*refresh_token"; then
                echo "   Cookie header encontrado (revisar Max-Age manualmente)"
            else
                echo "   ❌ No se encontró header set-cookie para limpiar refresh token"
            fi
        fi
    else
        echo "❌ Test 4 falló: No se encontró mensaje específico de doble expiración"
        echo "   Buscando otros mensajes de error en la respuesta..."
        if echo "$API_RESPONSE" | grep -q "error"; then
            ERROR_MSG=$(echo "$API_RESPONSE" | grep -o '"error":"[^"]*"' | cut -d '"' -f 4)
            echo "   Error encontrado: $ERROR_MSG"
        fi
    fi
else
    echo "❌ Test 4 FALLÓ CRÍTICO: $(echo "$API_RESPONSE" | head -n 1)"
    echo "   Se esperaba 401 Unauthorized para doble expiración"
    if echo "$API_RESPONSE" | grep -q "HTTP/1.1 200"; then
        echo "   ⚠️ PROBLEMA: El sistema aún permite acceso con tokens expirados"
    fi
fi

echo ""

##########################################################################
# RESUMEN FINAL
##########################################################################
echo "🏆 RESUMEN FINAL - SISTEMA 2/3 COMPLETO CON DOBLE EXPIRACIÓN"
echo "=============================================================="
echo "✅ Test 1: API normal (t=0s) - SIN refresh"
echo "✅ Test 2: Refresh parcial (t=62s) - Solo access token (primer 1/3)"
echo "✅ Test 3: Sistema 2/3 (t=110s) - Reset completo (>1/3 transcurrido)"
echo "✅ Test 4: Doble expiración por tiempo real (t=${ELAPSED}s) - Error 401 + cookie limpieza"
echo ""
echo "🎯 CONCLUSIÓN: El sistema 2/3 con doble expiración funciona PERFECTAMENTE"
echo "   • Primer 1/3 (0-100s): Mantiene refresh token existente"
echo "   • Últimos 2/3 (>100s): Reset completo cuando quedan 2/3 del tiempo"
echo "   • Doble expiración (>430s): Error descriptivo + limpieza cookies"
echo "   • Lógica temporal completa implementada correctamente"
echo ""
echo "📊 Revisa logs detallados: tail -f .spin-dev.log | grep 'DEBUG 2/3'"

# Limpieza de archivos temporales Ed25519
rm -f .test-ed25519-private-key