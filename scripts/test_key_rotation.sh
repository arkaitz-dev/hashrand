#!/bin/bash

# Test script for Ed25519 Key Rotation (TRAMO 1/3 and 2/3)
# Tests the 2/3 time window logic for automatic key rotation

set -e

API_BASE="http://localhost:3000/api"
TEST_EMAIL="test@example.com"
UI_HOST="http://localhost:5173"

echo "🔐 ===== ED25519 KEY ROTATION TEST ====="
echo ""

# Helper function to extract JSON field
extract_json() {
    echo "$1" | grep -o "\"$2\":\"[^\"]*\"" | cut -d'"' -f4
}

# STEP 1: Request magic link
echo "📧 STEP 1: Requesting magic link..."
MAGIC_RESPONSE=$(curl -s -X POST "$API_BASE/login/" \
    -H "Content-Type: application/json" \
    -d "{\"payload\":\"$(echo "{\"email\":\"$TEST_EMAIL\",\"ui_host\":\"$UI_HOST\",\"next\":\"/\",\"email_lang\":\"en\",\"pub_key\":\"0000000000000000000000000000000000000000000000000000000000000000\"}" | base64 -w0)\",\"signature\":\"dummy\"}")

echo "✅ Magic link requested"
echo ""

# STEP 2: Extract magic token from logs (simulated - in real scenario would come from email)
echo "⚠️  STEP 2: En producción, extraerías el magic token del email"
echo "   Para este test, necesitas autenticarte manualmente en el navegador"
echo ""

# STEP 3: Wait for manual authentication
echo "🔑 STEP 3: Por favor, auténticate en http://localhost:5173"
echo "   Presiona ENTER cuando hayas completado el login..."
read -r

# STEP 4: Test TRAMO 1/3 (0-40s) - NO rotation expected
echo ""
echo "⏱️  STEP 4: Testing TRAMO 1/3 (0-40s) - NO rotation expected"
echo "   Esperando 5 segundos para estar en ventana 1/3..."
sleep 5

echo "📤 Enviando request /api/refresh (esperando TRAMO 1/3)..."
REFRESH_RESPONSE_1=$(curl -s -X POST "$API_BASE/refresh" \
    -H "Content-Type: application/json" \
    -b "refresh_token=dummy" \
    -d "{\"payload\":\"$(echo "{\"new_pub_key\":\"1111111111111111111111111111111111111111111111111111111111111111\"}" | base64 -w0)\",\"signature\":\"dummy\"}")

echo "📥 Response received:"
echo "$REFRESH_RESPONSE_1" | jq . 2>/dev/null || echo "$REFRESH_RESPONSE_1"

if echo "$REFRESH_RESPONSE_1" | grep -q "server_pub_key"; then
    echo "❌ FAIL: server_pub_key present in TRAMO 1/3 (should NOT be present)"
    exit 1
else
    echo "✅ PASS: No server_pub_key in TRAMO 1/3 (expected)"
fi
echo ""

# STEP 5: Wait for TRAMO 2/3 window
echo "⏱️  STEP 5: Testing TRAMO 2/3 (40-120s) - rotation expected"
echo "   Esperando 45 segundos para entrar en ventana 2/3..."
sleep 45

echo "📤 Enviando request /api/refresh (esperando TRAMO 2/3)..."
REFRESH_RESPONSE_2=$(curl -s -X POST "$API_BASE/refresh" \
    -H "Content-Type: application/json" \
    -b "refresh_token=dummy" \
    -d "{\"payload\":\"$(echo "{\"new_pub_key\":\"2222222222222222222222222222222222222222222222222222222222222222\"}" | base64 -w0)\",\"signature\":\"dummy\"}")

echo "📥 Response received:"
echo "$REFRESH_RESPONSE_2" | jq . 2>/dev/null || echo "$REFRESH_RESPONSE_2"

if echo "$REFRESH_RESPONSE_2" | grep -q "server_pub_key"; then
    echo "✅ PASS: server_pub_key present in TRAMO 2/3 (expected)"
else
    echo "❌ FAIL: No server_pub_key in TRAMO 2/3 (should be present)"
    exit 1
fi

echo ""
echo "🎉 ===== ALL TESTS PASSED ====="
echo ""
echo "Summary:"
echo "  ✅ TRAMO 1/3 (0-40s): No key rotation (access_token only)"
echo "  ✅ TRAMO 2/3 (40-120s): Full key rotation (with server_pub_key)"