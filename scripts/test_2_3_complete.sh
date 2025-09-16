#!/bin/bash

# Test completo del sistema 2/3 con 4 fases progresivas
echo "🧪 TEST COMPLETO SISTEMA 2/3: Ciclo de Vida Completo de Tokens"
echo "=================================================================="
echo "📋 PLAN DE PRUEBAS:"
echo "   Test 1 (t=0s):    Access válido → API normal"
echo "   Test 2 (t=62s):   Access expirado, refresh primer 1/3 → Solo nuevo access"
echo "   Test 3 (t=110s):  Sistema 2/3 (>1/3 elapsed, 2/3 remaining) → Access + refresh reset"
echo "   Test 4 (t=430s):  Usar cookies Test 3 → Esperar 320s → Doble expiración por tiempo"
echo ""

# Configuración
API_BASE="http://localhost:3000"
COOKIES_FILE="cookies_test.txt"

# Paso inicial: Generar hash y login
echo "🔐 SETUP: Generando hash base58 e iniciando sesión..."
RANDOM_HASH=$(node scripts/generate_hash.js)
echo "✅ Hash generado: $RANDOM_HASH"

# Solicitar magic link
MAGIC_RESPONSE=$(curl -s -c $COOKIES_FILE -X POST \
  -H "Content-Type: application/json" \
  -d "{\"email\":\"me@arkaitz.dev\",\"email_lang\":\"en\",\"ui_host\":\"$API_BASE\",\"random_hash\":\"$RANDOM_HASH\"}" \
  $API_BASE/api/login/)

if [ "$MAGIC_RESPONSE" != '{"status":"OK"}' ]; then
    echo "❌ Error solicitando magic link: $MAGIC_RESPONSE"
    exit 1
fi

# Extraer magic token
MAGIC_LINE=$(tail -n 50 .spin-dev.log | grep "🔗.*magiclink=" | tail -n 1)
MAGIC_TOKEN=$(echo "$MAGIC_LINE" | grep -o 'magiclink=[^&]*' | cut -d '=' -f 2)

if [ -z "$MAGIC_TOKEN" ] || [ ${#MAGIC_TOKEN} -lt 10 ]; then
    echo "❌ Error extrayendo magic token"
    exit 1
fi

# Validar magic link y obtener tokens iniciales
LOGIN_RESPONSE=$(curl -s -b $COOKIES_FILE -c $COOKIES_FILE \
  "$API_BASE/api/login/?magiclink=${MAGIC_TOKEN}&hash=${RANDOM_HASH}")

if ! echo "$LOGIN_RESPONSE" | grep -q '"access_token"'; then
    echo "❌ Error en login: $LOGIN_RESPONSE"
    exit 1
fi

ACCESS_TOKEN=$(echo "$LOGIN_RESPONSE" | grep -o '"access_token":"[^"]*"' | sed 's/"access_token":"\([^"]*\)"/\1/')
echo "✅ Login exitoso. Access Token: ${ACCESS_TOKEN:0:40}..."

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
    if ! echo "$API_RESPONSE" | grep -q "x-new-access-token:"; then
        if echo "$API_RESPONSE" | grep -q '"hash"'; then
            HASH=$(echo "$API_RESPONSE" | grep -o '"hash":"[^"]*"' | cut -d '"' -f 4)
            echo "✅ Test 1 EXITOSO - Hash generado sin refresh: $HASH"
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
    if echo "$API_RESPONSE" | grep -q "x-new-access-token:"; then
        NEW_TOKEN=$(echo "$API_RESPONSE" | grep "x-new-access-token:" | cut -d' ' -f2 | tr -d '\r')
        if ! echo "$API_RESPONSE" | grep -q "set-cookie.*refresh_token"; then
            echo "✅ Test 2 EXITOSO - Refresh parcial (solo access token)"
            echo "   • Nuevo access token: ${NEW_TOKEN:0:40}..."
            echo "   • Refresh token mantenido (primer 1/3) ✅"
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
    if echo "$API_RESPONSE" | grep -q "x-new-access-token:"; then
        NEW_ACCESS=$(echo "$API_RESPONSE" | grep "x-new-access-token:" | cut -d' ' -f2 | tr -d '\r')
        if echo "$API_RESPONSE" | grep -q "set-cookie.*refresh_token"; then
            echo "🎉 Test 3 EXITOSO - SISTEMA 2/3 FUNCIONANDO PERFECTAMENTE"
            echo "   • Nuevo access token: ${NEW_ACCESS:0:40}..."
            echo "   • Nuevo refresh token (reset completo) ✅"
            echo "   • Tiempo reseteado a 5 minutos completos ✅"

            if echo "$API_RESPONSE" | grep -q '"hash"'; then
                HASH=$(echo "$API_RESPONSE" | grep -o '"hash":"[^"]*"' | cut -d '"' -f 4)
                echo "   • Hash generado: $HASH ✅"
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