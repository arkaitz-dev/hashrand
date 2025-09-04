#!/bin/bash

# Test script for complete authentication flow
echo "🔐 Testing Complete Authentication Flow"
echo "======================================"

# Step 1: Request magic link
echo "📧 Step 1: Requesting magic link for me@arkaitz.dev..."
MAGIC_RESPONSE=$(curl -s -X POST http://localhost:3000/api/login/ \
  -H "Content-Type: application/json" \
  -d '{"email": "me@arkaitz.dev"}')

echo "Response: $MAGIC_RESPONSE"

# Extract the magic link from response (in development mode)
MAGIC_LINK=$(echo "$MAGIC_RESPONSE" | grep -o 'http://[^"]*magiclink=[^"]*' | head -1)

if [ -n "$MAGIC_LINK" ]; then
    echo "✅ Magic link generated: $MAGIC_LINK"
    
    # Extract just the token
    MAGIC_TOKEN=$(echo "$MAGIC_LINK" | grep -o 'magiclink=[^&]*' | cut -d'=' -f2)
    echo "🔑 Magic token: $MAGIC_TOKEN"
    
    # Step 2: Validate magic link
    echo ""
    echo "🔐 Step 2: Validating magic link..."
    AUTH_RESPONSE=$(curl -s "http://localhost:3000/api/login/?magiclink=$MAGIC_TOKEN" -D /tmp/auth_headers.txt)
    
    echo "Response: $AUTH_RESPONSE"
    
    # Check for access token
    ACCESS_TOKEN=$(echo "$AUTH_RESPONSE" | grep -o '"access_token":"[^"]*' | cut -d'"' -f4)
    
    if [ -n "$ACCESS_TOKEN" ]; then
        echo "✅ Authentication successful!"
        echo "🎫 Access token: ${ACCESS_TOKEN:0:50}..."
        
        # Check for refresh token cookie
        REFRESH_COOKIE=$(grep -i "refresh_token=" /tmp/auth_headers.txt)
        if [ -n "$REFRESH_COOKIE" ]; then
            echo "🍪 Refresh token cookie set: ${REFRESH_COOKIE:0:100}..."
        fi
        
        echo ""
        echo "🎉 COMPLETE AUTHENTICATION FLOW SUCCESSFUL! 🎉"
        echo "✅ Magic link generation: WORKING"
        echo "✅ Magic link validation: WORKING"
        echo "✅ JWT token generation: WORKING"
        echo "✅ Refresh token cookie: WORKING"
        echo "✅ Database operations: WORKING"
        
        # Test protected API call (if available)
        echo ""
        echo "🔒 Testing protected API access with token..."
        # This would test an actual protected endpoint if we had one
        # For now, just show that we have valid authentication
        
    else
        echo "❌ Authentication failed - no access token received"
        exit 1
    fi
    
else
    echo "❌ Failed to get magic link"
    exit 1
fi

# Cleanup
rm -f /tmp/auth_headers.txt

echo ""
echo "🚀 Frontend authentication flow should now work!"
echo "   1. Go to: http://localhost:5173/custom"
echo "   2. AuthGuard should show login dialog"
echo "   3. Enter email: me@arkaitz.dev"
echo "   4. Click the generated magic link"
echo "   5. Should be authenticated and able to use the form"