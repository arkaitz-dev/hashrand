#!/bin/bash

# Test rápido de doble expiración
echo "🧪 TEST DOBLE EXPIRACIÓN RÁPIDO"
echo "==============================="

API_BASE="http://localhost:3000"
COOKIES_FILE="cookies_double_test.txt"

# Generar hash y login
echo "🔐 Iniciando sesión..."
RANDOM_HASH=$(node scripts/generate_hash.js)
echo "Hash: $RANDOM_HASH"

# Solicitar magic link
MAGIC_RESPONSE=$(curl -s -c $COOKIES_FILE -X POST \
  -H "Content-Type: application/json" \
  -d "{\"email\":\"me@arkaitz.dev\",\"ui_host\":\"$API_BASE\",\"random_hash\":\"$RANDOM_HASH\"}" \
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

# Login y obtener tokens
LOGIN_RESPONSE=$(curl -s -b $COOKIES_FILE -c $COOKIES_FILE \
  "$API_BASE/api/login/?magiclink=${MAGIC_TOKEN}&hash=${RANDOM_HASH}")

if ! echo "$LOGIN_RESPONSE" | grep -q '"access_token"'; then
    echo "❌ Error en login: $LOGIN_RESPONSE"
    exit 1
fi

ACCESS_TOKEN=$(echo "$LOGIN_RESPONSE" | grep -o '"access_token":"[^"]*"' | sed 's/"access_token":"\([^"]*\)"/\1/')
echo "✅ Login exitoso. Access Token: ${ACCESS_TOKEN:0:40}..."

# Esperar 70 segundos para que expire el access token
echo "⏰ Esperando 70s para que expire access token..."
sleep 70

# Test: Llamada API que debería activar doble expiración
echo "🧪 Haciendo llamada API con tokens expirados..."
API_RESPONSE=$(curl -s -i -b $COOKIES_FILE -c $COOKIES_FILE \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  $API_BASE/api/custom?length=8)

echo "=== RESPUESTA ==="
echo "$API_RESPONSE"
echo "================="

# Verificar si contiene mensaje de doble expiración
if echo "$API_RESPONSE" | grep -q "Both access and refresh tokens have expired"; then
    echo "🎉 ✅ DOBLE EXPIRACIÓN DETECTADA CORRECTAMENTE"
elif echo "$API_RESPONSE" | grep -q "expired"; then
    echo "⚠️ Detectó expiración pero no doble expiración"
else
    echo "❌ No detectó expiración"
fi

echo ""
echo "📊 Logs relevantes:"
tail -20 .spin-dev.log | grep "DEBUG.*expired\|DEBUG.*Cookie\|DEBUG.*DUAL"