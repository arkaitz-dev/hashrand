#!/bin/bash

# Test completo del sistema 2/3 token refresh con debug
echo "🧪 TEST COMPLETO: Sistema 2/3 Token Refresh"
echo "============================================="
echo ""

# Configuración
API_BASE="http://localhost:3000"
COOKIES_FILE="cookies_test.txt"

# Paso 1: Generar keypair Ed25519
echo "🔐 Paso 1: Generando keypair Ed25519..."
PUB_KEY=$(node scripts/generate_hash.js)
echo "✅ Public key generada: $PUB_KEY"

# Paso 2: Crear firma Ed25519
echo ""
echo "🔏 Paso 2: Firmando payload..."
EMAIL="me@arkaitz.dev"
# Crear mensaje: email + pub_key + next (next defaults to "/")
MESSAGE="${EMAIL}${PUB_KEY}/"
SIGNATURE=$(node scripts/sign_payload.js "$MESSAGE")
echo "✅ Signature generada: ${SIGNATURE:0:40}..."

# Paso 3: Solicitar magic link con Ed25519
echo ""
echo "📧 Paso 3: Solicitando magic link con Ed25519..."
MAGIC_RESPONSE=$(curl -s -c $COOKIES_FILE -X POST \
  -H "Content-Type: application/json" \
  -d "{\"email\":\"$EMAIL\",\"email_lang\":\"en\",\"ui_host\":\"$API_BASE\",\"pub_key\":\"$PUB_KEY\",\"signature\":\"$SIGNATURE\"}" \
  $API_BASE/api/login/)

if [ "$MAGIC_RESPONSE" = '{"status":"OK"}' ]; then
    echo "✅ Magic link solicitado exitosamente"

    # Extraer magic token del último log
    echo ""
    echo "🔍 Paso 4: Extrayendo magic token del log más reciente..."

    # Verificar que el log existe y contiene magic links
    if [ ! -f ".spin-dev.log" ]; then
        echo "❌ ERROR: Archivo .spin-dev.log no existe"
        exit 1
    fi

    # Buscar magic link en log con más debug
    MAGIC_LINE=$(tail -n 50 .spin-dev.log | grep "Generated magic_link\|🔗.*magiclink=" | tail -n 1)
    echo "   Línea encontrada en log: $MAGIC_LINE"

    MAGIC_TOKEN=$(echo "$MAGIC_LINE" | grep -o 'magiclink=[^&]*' | cut -d '=' -f 2)

    if [ -n "$MAGIC_TOKEN" ] && [ ${#MAGIC_TOKEN} -gt 10 ]; then
        echo "✅ Magic token extraído: ${MAGIC_TOKEN:0:40}..."
        echo "   Longitud: ${#MAGIC_TOKEN} caracteres"

        # Paso 5: Validar magic link (pub_key se extrae automáticamente del payload almacenado)
        echo ""
        echo "🔑 Paso 5: Validando magic link..."
        LOGIN_RESPONSE=$(curl -s -b $COOKIES_FILE -c $COOKIES_FILE \
          "$API_BASE/api/login/?magiclink=${MAGIC_TOKEN}")

        echo "Response: $LOGIN_RESPONSE"

        if echo "$LOGIN_RESPONSE" | grep -q '"access_token"'; then
            ACCESS_TOKEN=$(echo "$LOGIN_RESPONSE" | grep -o '"access_token":"[^"]*"' | sed 's/"access_token":"\([^"]*\)"/\1/')
            echo ""
            echo "✅ ¡LOGIN EXITOSO!"
            echo "   Access Token: ${ACCESS_TOKEN:0:40}..."

            # Verificar cookies CRÍTICO
            echo ""
            echo "🍪 Verificando cookies de refresh token..."
            if grep -q "refresh_token" $COOKIES_FILE; then
                REFRESH_COOKIE_LINE=$(cat $COOKIES_FILE | grep -v "^#" | grep refresh_token)
                echo "✅ Refresh token cookie encontrada:"
                echo "   $REFRESH_COOKIE_LINE"
                # Extraer timestamp de expiración para verificar duración
                COOKIE_EXPIRY=$(echo "$REFRESH_COOKIE_LINE" | awk '{print $5}')
                CURRENT_TIMESTAMP=$(date +%s)
                EXPIRY_DIFF=$((COOKIE_EXPIRY - CURRENT_TIMESTAMP))
                if [ $EXPIRY_DIFF -gt 0 ]; then
                    echo "   Expira en: $EXPIRY_DIFF segundos (~$((EXPIRY_DIFF/60)) minutos)"
                else
                    echo "   Cookie expiry timestamp: $COOKIE_EXPIRY (formato válido)"
                fi
            else
                echo "❌ ERROR CRÍTICO: No se encontró cookie de refresh token"
                echo "   Contenido completo del archivo cookies:"
                cat $COOKIES_FILE
                exit 1
            fi

            # Test inmediato
            echo ""
            echo "🧪 Test 1: API call inmediata (t=0s) - debe funcionar sin refresh"
            API_RESPONSE=$(curl -s -b $COOKIES_FILE -c $COOKIES_FILE \
              -H "Authorization: Bearer $ACCESS_TOKEN" \
              $API_BASE/api/custom?length=8)

            if echo "$API_RESPONSE" | grep -q '"hash"'; then
                echo "✅ Test 1 exitoso - Hash generado: $(echo "$API_RESPONSE" | grep -o '"hash":"[^"]*"' | cut -d '"' -f 4)"
            else
                echo "❌ Test 1 falló: $API_RESPONSE"
                exit 1
            fi

            # Test 2: Verificar refresh por access token expirado (1 minuto)
            WAIT_TIME_ACCESS=62
            echo ""
            echo "⏰ Test 2A: Esperando $WAIT_TIME_ACCESS segundos para verificar refresh por access token expirado..."
            sleep $WAIT_TIME_ACCESS

            echo ""
            echo "🔥 TEST 2A: Refresh por ACCESS TOKEN expirado después de ${WAIT_TIME_ACCESS}s"
            echo "========================================="
            echo "   • El access token DEBE estar expirado (>${WAIT_TIME_ACCESS}s > 60s)"
            echo "   • El refresh token DEBE estar válido (9 minutos desde creación)"
            echo "   • El middleware DEBE detectar token expirado y hacer refresh"
            echo ""

            # VERIFICACIÓN CRÍTICA: Asegurar que las cookies existen antes del test
            echo "🔍 Verificación pre-test: Cookies disponibles para refresh automático"
            if [ ! -f "$COOKIES_FILE" ]; then
                echo "❌ ERROR: Archivo de cookies no existe"
                exit 1
            fi
            if ! grep -q "refresh_token" $COOKIES_FILE; then
                echo "❌ ERROR: No hay refresh token en cookies antes del test crítico"
                cat $COOKIES_FILE
                exit 1
            fi
            echo "✅ Cookies verificadas, procediendo con test crítico..."
            echo ""

            # Test crítico con headers completos
            echo "📤 Enviando request con:"
            echo "   • Access token expirado (${WAIT_TIME}s > 60s): ${ACCESS_TOKEN:0:20}..."
            echo "   • Cookies de refresh token: SÍ"
            echo ""
            API_RESPONSE=$(curl -s -i -b $COOKIES_FILE -c $COOKIES_FILE \
              -H "Authorization: Bearer $ACCESS_TOKEN" \
              $API_BASE/api/custom?length=12)

            echo "=== RESPONSE COMPLETA ==="
            echo "$API_RESPONSE"
            echo "========================"
            echo ""

            # VERIFICACIÓN ULTRATHINK: ¿Hay respuesta válida?
            if [ -z "$API_RESPONSE" ] || [ ${#API_RESPONSE} -lt 10 ]; then
                echo "❌ ERROR CRÍTICO: Respuesta vacía o demasiado corta del servidor"
                echo "   Verificando conectividad con servidor..."
                curl -s "$API_BASE/api/version" || echo "   ❌ Servidor no responde"
                exit 1
            fi

            # ¿Contiene cabeceras HTTP válidas?
            if ! echo "$API_RESPONSE" | head -n 1 | grep -q "HTTP/1.1"; then
                echo "❌ ERROR: Respuesta no contiene cabeceras HTTP válidas"
                echo "   Primera línea: $(echo "$API_RESPONSE" | head -n 1)"
                exit 1
            fi

            # Análisis de la respuesta
            if echo "$API_RESPONSE" | grep -q "HTTP/1.1 200"; then
                echo "✅ Status: 200 OK"
                if echo "$API_RESPONSE" | grep -q "x-new-access-token:"; then
                    NEW_TOKEN=$(echo "$API_RESPONSE" | grep "x-new-access-token:" | cut -d' ' -f2 | tr -d '\r')
                    echo "🔄 ¡SISTEMA 2/3 FUNCIONANDO!"
                    echo "   • Token renovado automáticamente: ${NEW_TOKEN:0:40}..."

                    # VERIFICACIÓN ULTRATHINK: ¿Se actualizó también el refresh token?
                    if echo "$API_RESPONSE" | grep -q "set-cookie.*refresh_token"; then
                        echo "   • Refresh token también renovado: ✅"
                    else
                        echo "   • Refresh token NO renovado: ⚠️"
                    fi

                    # ¿El hash se generó correctamente con el nuevo token?
                    if echo "$API_RESPONSE" | grep -q '"hash"'; then
                        HASH=$(echo "$API_RESPONSE" | grep -o '"hash":"[^"]*"' | cut -d '"' -f 4)
                        echo "   • Hash generado con nuevo token: $HASH ✅"
                    else
                        echo "   • ERROR: No se generó hash con nuevo token ❌"
                    fi

                    echo "✅ Test 2 EXITOSO - Refresh automático funciona correctamente"
                else
                    echo "⚠️  Status 200 pero sin headers de renovación"
                    echo "   Verificando si el hash se generó correctamente..."
                    if echo "$API_RESPONSE" | grep -q '"hash"'; then
                        HASH=$(echo "$API_RESPONSE" | grep -o '"hash":"[^"]*"' | cut -d '"' -f 4)
                        echo "   Hash generado: $HASH (token aún válido?)"
                    fi
                fi
            else
                echo "❌ Test 2 FALLÓ:"
                echo "   Status: $(echo "$API_RESPONSE" | head -n 1)"
                if echo "$API_RESPONSE" | grep -q "401"; then
                    echo "   ❌ PROBLEMA: El sistema 2/3 NO está funcionando"
                    echo "   ❌ El middleware no está haciendo refresh automático"
                fi
            fi

        else
            echo "❌ Error en validación: $LOGIN_RESPONSE"
            exit 1
        fi
    else
        echo "❌ No se pudo extraer magic token del log"
        exit 1
    fi
else
    echo "❌ Error solicitando magic link: $MAGIC_RESPONSE"
    exit 1
fi

echo ""
echo "📊 Test completado. Revisa logs con debug para diagnóstico:"
echo "   tail -f .spin-dev.log | grep DEBUG"